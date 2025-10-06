use crate::orderbook::OrderBook;
use crate::zerocopy::metrics::ZeroCopyMetrics;
use arrow_array::{
    ArrayRef, Float64Array, Int32Array, RecordBatch, StringArray, TimestampNanosecondArray,
};
use arrow_data::ArrayData;
use arrow_flight::encode::FlightDataEncoderBuilder;
use arrow_flight::error::FlightError;
use arrow_flight::flight_descriptor::DescriptorType;
use arrow_flight::flight_service_server::{FlightService, FlightServiceServer};
use arrow_flight::{
    Action, ActionType, Criteria, FlightData, FlightDescriptor, FlightEndpoint, FlightInfo,
    HandshakeRequest, HandshakeResponse, Location, PollInfo, PutResult, Result as FlightResult,
    SchemaResult, Ticket,
};
use arrow_ipc::writer::{DictionaryTracker, IpcDataGenerator, IpcWriteOptions};
use arrow_schema::{DataType, Field, Schema, SchemaRef, TimeUnit};
use bytes::Bytes;
use futures::{Stream, StreamExt, stream};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tonic::async_trait;
use tonic::{Request, Response, Status};

use crate::orderbook::OrderBookSnapshot;

/// Provides snapshots for a given symbol.
pub trait SnapshotProvider: Send + Sync {
    /// Produce a snapshot at the requested depth.
    fn snapshot(&self, depth: usize) -> OrderBookSnapshot;
}

/// [`SnapshotProvider`] backed by an [`OrderBook`].
pub struct OrderBookSnapshotProvider<T>
where
    T: Clone + Send + Sync + Default + 'static,
{
    symbol: String,
    book: Arc<OrderBook<T>>,
}

impl<T> OrderBookSnapshotProvider<T>
where
    T: Clone + Send + Sync + Default + 'static,
{
    pub fn new(symbol: impl Into<String>, book: Arc<OrderBook<T>>) -> Self {
        Self {
            symbol: symbol.into(),
            book,
        }
    }

    /// Symbol handled by this provider.
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

impl<T> SnapshotProvider for OrderBookSnapshotProvider<T>
where
    T: Clone + Send + Sync + Default + 'static,
{
    fn snapshot(&self, depth: usize) -> OrderBookSnapshot {
        self.book.create_snapshot(depth)
    }
}

/// Lightweight view of memory pool usage for instrumentation.
#[derive(Debug, Default, Clone, Copy)]
pub struct MemoryPoolStats {
    pub bytes_in_use: u64,
}

/// Configuration for the zero-copy Flight service.
#[derive(Clone, Debug)]
pub struct ZeroCopyFlightConfig {
    /// Maximum depth exported per snapshot.
    pub depth: usize,
    /// Schema semantic version exposed via metadata.
    pub schema_version: String,
    /// URIs advertised for DoGet endpoints.
    pub locations: Vec<String>,
    /// Descriptor prefix required in FlightDescriptor.path (e.g. ["orderbook", "l2"]).
    pub descriptor_prefix: Vec<String>,
}

impl Default for ZeroCopyFlightConfig {
    fn default() -> Self {
        Self {
            depth: 50,
            schema_version: "1.0.0".to_string(),
            locations: vec!["grpc+tcp://127.0.0.1:50051".to_string()],
            descriptor_prefix: vec!["orderbook".to_string(), "l2".to_string()],
        }
    }
}

/// Arrow Flight service that streams order book snapshots without additional copies.
pub struct ZeroCopyFlightService {
    config: ZeroCopyFlightConfig,
    registry: Arc<RwLock<HashMap<String, Arc<dyn SnapshotProvider>>>>,
    schema: SchemaRef,
    ipc_options: IpcWriteOptions,
    metrics: Arc<ZeroCopyMetrics>,
    locations: Arc<Vec<Location>>,
}

impl ZeroCopyFlightService {
    /// Create a new Flight service using the provided configuration.
    pub fn new(config: ZeroCopyFlightConfig) -> Self {
        Self::with_metrics(config, Arc::new(ZeroCopyMetrics::default()))
    }

    /// Create a new Flight service with explicit metrics instrumentation.
    pub fn with_metrics(config: ZeroCopyFlightConfig, metrics: Arc<ZeroCopyMetrics>) -> Self {
        let schema = Arc::new(level2_schema());
        let locations = Arc::new(
            config
                .locations
                .iter()
                .map(|uri| Location { uri: uri.clone() })
                .collect::<Vec<_>>(),
        );

        Self {
            config,
            registry: Arc::new(RwLock::new(HashMap::new())),
            schema,
            ipc_options: IpcWriteOptions::default(),
            metrics,
            locations,
        }
    }

    /// Register a snapshot provider for a symbol.
    pub fn register_snapshot_provider(
        &self,
        symbol: impl Into<String>,
        provider: Arc<dyn SnapshotProvider>,
    ) {
        if let Ok(mut guard) = self.registry.write() {
            guard.insert(symbol.into(), provider);
        }
    }

    /// Convert the service into a gRPC server implementation.
    pub fn into_server(self) -> FlightServiceServer<Self> {
        FlightServiceServer::new(self)
    }

    fn schema_bytes(&self) -> Bytes {
        let generator = IpcDataGenerator::default();
        let mut tracker = DictionaryTracker::new(false);
        let encoded = generator.schema_to_bytes_with_dictionary_tracker(
            &self.schema,
            &mut tracker,
            &self.ipc_options,
        );
        encoded.ipc_message.into()
    }

    fn build_descriptor(&self, symbol: &str) -> FlightDescriptor {
        let mut path = self.config.descriptor_prefix.clone();
        path.push(symbol.to_string());
        FlightDescriptor {
            r#type: DescriptorType::Path as i32,
            cmd: Bytes::new(),
            path,
        }
    }

    fn build_flight_info(&self, symbol: &str) -> Result<FlightInfo, Status> {
        let descriptor = self.build_descriptor(symbol);
        let ticket = Ticket {
            ticket: Bytes::from(symbol.to_string()),
        };
        let endpoint = FlightEndpoint {
            ticket: Some(ticket),
            location: (*self.locations).clone(),
            expiration_time: None,
            app_metadata: Bytes::from(self.config.schema_version.clone()),
        };

        Ok(FlightInfo {
            schema: self.schema_bytes(),
            flight_descriptor: Some(descriptor),
            endpoint: vec![endpoint],
            total_records: 1,
            total_bytes: -1,
            ordered: true,
            app_metadata: Bytes::from(self.config.schema_version.clone()),
        })
    }

    fn snapshot_to_batch(&self, snapshot: OrderBookSnapshot) -> Result<RecordBatch, Status> {
        let best_bid = snapshot.best_bid();
        let best_ask = snapshot.best_ask();

        let symbol_array: ArrayRef = Arc::new(StringArray::from(vec![snapshot.symbol.clone()]));
        let ts_array: ArrayRef = Arc::new(TimestampNanosecondArray::from(vec![
            (snapshot.timestamp as i64) * 1_000_000,
        ]));

        let bid_px = best_bid
            .map(|(price, _)| price as f64)
            .unwrap_or(std::f64::NAN);
        let bid_qty = best_bid.map(|(_, qty)| qty as f64).unwrap_or(0.0);
        let bid_count: i32 = snapshot
            .bids
            .len()
            .try_into()
            .map_err(|_| Status::internal("bid count overflow"))?;

        let ask_px = best_ask
            .map(|(price, _)| price as f64)
            .unwrap_or(std::f64::NAN);
        let ask_qty = best_ask.map(|(_, qty)| qty as f64).unwrap_or(0.0);
        let ask_count: i32 = snapshot
            .asks
            .len()
            .try_into()
            .map_err(|_| Status::internal("ask count overflow"))?;

        let batch = RecordBatch::try_new(
            self.schema.clone(),
            vec![
                symbol_array,
                ts_array,
                Arc::new(Float64Array::from(vec![bid_px])) as ArrayRef,
                Arc::new(Float64Array::from(vec![bid_qty])) as ArrayRef,
                Arc::new(Int32Array::from(vec![bid_count])) as ArrayRef,
                Arc::new(Float64Array::from(vec![ask_px])) as ArrayRef,
                Arc::new(Float64Array::from(vec![ask_qty])) as ArrayRef,
                Arc::new(Int32Array::from(vec![ask_count])) as ArrayRef,
            ],
        )
        .map_err(|err| Status::internal(err.to_string()))?;

        if let Err(err) = validate_batch_alignment(&batch) {
            self.metrics.record_fallback();
            return Err(err);
        }

        Ok(batch)
    }

    fn record_metrics(&self, stats: MemoryPoolStats, latency: Duration) {
        if let Ok(bytes) = i64::try_from(stats.bytes_in_use) {
            self.metrics.record_pool_bytes(bytes);
        }
        self.metrics
            .observe_snapshot_export(stats.bytes_in_use, latency);
    }

    fn parse_symbol_from_descriptor(
        &self,
        descriptor: &FlightDescriptor,
    ) -> Result<String, Status> {
        let descriptor_type = DescriptorType::try_from(descriptor.r#type)
            .map_err(|_| Status::invalid_argument("unknown descriptor type"))?;

        if descriptor_type != DescriptorType::Path {
            return Err(Status::invalid_argument("descriptor must use PATH"));
        }

        if descriptor.path.len() != self.config.descriptor_prefix.len() + 1 {
            return Err(Status::invalid_argument("descriptor path length mismatch"));
        }

        for (expected, actual) in self.config.descriptor_prefix.iter().zip(&descriptor.path) {
            if expected != actual {
                return Err(Status::invalid_argument("descriptor prefix mismatch"));
            }
        }

        Ok(descriptor.path.last().cloned().unwrap())
    }

    fn parse_symbol_ticket(&self, ticket: &Ticket) -> Result<String, Status> {
        String::from_utf8(ticket.ticket.clone().to_vec())
            .map_err(|_| Status::invalid_argument("ticket must be utf8"))
    }

    fn lookup_provider(&self, symbol: &str) -> Result<Arc<dyn SnapshotProvider>, Status> {
        let registry = self
            .registry
            .read()
            .map_err(|_| Status::internal("snapshot registry poisoned"))?;
        registry
            .get(symbol)
            .cloned()
            .ok_or_else(|| Status::not_found(format!("symbol {symbol} not registered")))
    }
}

type FlightStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

#[async_trait]
impl FlightService for ZeroCopyFlightService {
    type HandshakeStream = FlightStream<HandshakeResponse>;
    type ListFlightsStream = FlightStream<FlightInfo>;
    type DoGetStream = FlightStream<FlightData>;
    type DoPutStream = FlightStream<PutResult>;
    type DoExchangeStream = FlightStream<FlightData>;
    type DoActionStream = FlightStream<FlightResult>;
    type ListActionsStream = FlightStream<ActionType>;

    async fn handshake(
        &self,
        _request: Request<tonic::Streaming<HandshakeRequest>>,
    ) -> Result<Response<Self::HandshakeStream>, Status> {
        Ok(Response::new(Box::pin(stream::empty())))
    }

    async fn list_flights(
        &self,
        _request: Request<Criteria>,
    ) -> Result<Response<Self::ListFlightsStream>, Status> {
        let symbols = self
            .registry
            .read()
            .map_err(|_| Status::internal("snapshot registry poisoned"))?
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        let infos = symbols
            .into_iter()
            .map(|symbol| self.build_flight_info(&symbol))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Response::new(Box::pin(stream::iter(
            infos.into_iter().map(Ok),
        ))))
    }

    async fn get_flight_info(
        &self,
        request: Request<FlightDescriptor>,
    ) -> Result<Response<FlightInfo>, Status> {
        let symbol = self.parse_symbol_from_descriptor(request.get_ref())?;
        self.lookup_provider(&symbol)?;
        let info = self.build_flight_info(&symbol)?;
        Ok(Response::new(info))
    }

    async fn poll_flight_info(
        &self,
        _request: Request<FlightDescriptor>,
    ) -> Result<Response<PollInfo>, Status> {
        Err(Status::unimplemented("poll_flight_info is not supported"))
    }

    async fn get_schema(
        &self,
        request: Request<FlightDescriptor>,
    ) -> Result<Response<SchemaResult>, Status> {
        let symbol = self.parse_symbol_from_descriptor(request.get_ref())?;
        self.lookup_provider(&symbol)?;
        Ok(Response::new(SchemaResult {
            schema: self.schema_bytes(),
        }))
    }

    async fn do_get(
        &self,
        request: Request<Ticket>,
    ) -> Result<Response<Self::DoGetStream>, Status> {
        let start = Instant::now();
        let symbol = self.parse_symbol_ticket(request.get_ref())?;
        let provider = match self.lookup_provider(&symbol) {
            Ok(provider) => provider,
            Err(status) => {
                self.metrics.record_fallback();
                return Err(status);
            }
        };
        let snapshot = provider.snapshot(self.config.depth);
        let batch = self.snapshot_to_batch(snapshot)?;
        let approx_bytes = batch.get_array_memory_size() as u64;
        self.record_metrics(
            MemoryPoolStats {
                bytes_in_use: approx_bytes,
            },
            start.elapsed(),
        );

        let descriptor = self.build_descriptor(&symbol);
        let stream = FlightDataEncoderBuilder::new()
            .with_schema(self.schema.clone())
            .with_flight_descriptor(Some(descriptor))
            .build(stream::iter(vec![Ok::<_, FlightError>(batch)]))
            .map(|result| result.map_err(|err| Status::internal(err.to_string())));

        Ok(Response::new(Box::pin(stream)))
    }

    async fn do_put(
        &self,
        _request: Request<tonic::Streaming<FlightData>>,
    ) -> Result<Response<Self::DoPutStream>, Status> {
        Err(Status::unimplemented("do_put is not supported"))
    }

    async fn do_exchange(
        &self,
        _request: Request<tonic::Streaming<FlightData>>,
    ) -> Result<Response<Self::DoExchangeStream>, Status> {
        Err(Status::unimplemented("do_exchange is not supported"))
    }

    async fn do_action(
        &self,
        _request: Request<Action>,
    ) -> Result<Response<Self::DoActionStream>, Status> {
        Err(Status::unimplemented("do_action is not supported"))
    }

    async fn list_actions(
        &self,
        _request: Request<arrow_flight::Empty>,
    ) -> Result<Response<Self::ListActionsStream>, Status> {
        let stream = stream::iter(vec![Ok(ActionType {
            r#type: "zero-copy.dual-publish".to_string(),
            description: "Inspect or toggle dual-publish zero-copy mode".to_string(),
        })]);

        Ok(Response::new(Box::pin(stream)))
    }
}

fn level2_schema() -> Schema {
    Schema::new(vec![
        Field::new("symbol", DataType::Utf8, false),
        Field::new(
            "ts_event",
            DataType::Timestamp(TimeUnit::Nanosecond, None),
            false,
        ),
        Field::new("bid_px", DataType::Float64, true),
        Field::new("bid_qty", DataType::Float64, true),
        Field::new("bid_count", DataType::Int32, false),
        Field::new("ask_px", DataType::Float64, true),
        Field::new("ask_qty", DataType::Float64, true),
        Field::new("ask_count", DataType::Int32, false),
    ])
}

fn validate_batch_alignment(batch: &RecordBatch) -> Result<(), Status> {
    for column in batch.columns() {
        validate_array_alignment(column)?;
    }
    Ok(())
}

fn validate_array_alignment(array: &ArrayRef) -> Result<(), Status> {
    let data = array.to_data();
    validate_data_alignment(&data)
}

fn validate_data_alignment(data: &ArrayData) -> Result<(), Status> {
    for buffer in data.buffers() {
        let ptr = buffer.as_ptr();
        if ptr.is_null() {
            continue;
        }
        if (ptr as usize) % 8 != 0 {
            return Err(Status::internal("unaligned Arrow buffer detected"));
        }
    }

    for child in data.child_data() {
        validate_data_alignment(child)?;
    }

    Ok(())
}
