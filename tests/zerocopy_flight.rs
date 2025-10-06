use std::{convert::TryFrom, error::Error, sync::Arc, time::Duration};

use arrow_flight::{FlightClient, FlightDescriptor, Ticket, flight_descriptor::DescriptorType};
use bytes::Bytes;
use futures::TryStreamExt;
use orderbook_rs::orderbook::OrderBook;
use orderbook_rs::zerocopy::{
    OrderBookSnapshotProvider, ZeroCopyFlightConfig, ZeroCopyFlightService, ZeroCopyMetrics,
};
use pricelevel::{OrderId, Side, TimeInForce};
use tokio::net::TcpListener;
use tokio::time::timeout;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::{Channel, Server};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn flight_round_trip_returns_depth_snapshot() -> Result<(), Box<dyn Error>> {
    let btc_levels = [Level::new_buy(100, 5), Level::new_sell(105, 4)];
    let feeds = [Feed::new("BTC-USD", &btc_levels)];
    let (server, mut client, config, _metrics) = start_service(&feeds).await?;

    let descriptor = level2_descriptor("BTC-USD");
    let info = client.get_flight_info(descriptor).await?;
    assert_eq!(info.endpoint.len(), 1);
    assert_eq!(info.app_metadata, config.schema_version.as_bytes());

    let ticket = info.endpoint[0]
        .ticket
        .clone()
        .expect("ticket missing for endpoint");

    let stream = client.do_get(ticket).await?;
    let batches = timeout(Duration::from_secs(1), stream.try_collect::<Vec<_>>()).await??;

    assert_eq!(batches.len(), 1);
    let batch = &batches[0];
    assert_eq!(batch.num_rows(), 1);

    let symbol_col = batch
        .column(0)
        .as_any()
        .downcast_ref::<arrow_array::StringArray>()
        .unwrap();
    assert_eq!(symbol_col.value(0), "BTC-USD");

    let bid_px = batch
        .column(2)
        .as_any()
        .downcast_ref::<arrow_array::Float64Array>()
        .unwrap();
    assert_eq!(bid_px.value(0), 100.0);

    let ask_px = batch
        .column(5)
        .as_any()
        .downcast_ref::<arrow_array::Float64Array>()
        .unwrap();
    assert_eq!(ask_px.value(0), 105.0);

    server.abort();

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn flight_lists_symbols_and_provides_schema() -> Result<(), Box<dyn Error>> {
    let btc_levels = [Level::new_buy(99, 1), Level::new_sell(101, 1)];
    let eth_levels = [Level::new_buy(200, 2), Level::new_sell(205, 2)];
    let feeds = [
        Feed::new("BTC-USD", &btc_levels),
        Feed::new("ETH-USD", &eth_levels),
    ];
    let (server, mut client, config, _metrics) = start_service(&feeds).await?;

    let flights: Vec<_> = client
        .list_flights(Bytes::new())
        .await?
        .try_collect()
        .await?;
    assert_eq!(flights.len(), 2);

    let mut symbols: Vec<String> = flights
        .iter()
        .map(|info| {
            assert_eq!(info.app_metadata, config.schema_version.as_bytes());
            assert_eq!(info.endpoint.len(), 1);
            let descriptor = info.flight_descriptor.as_ref().expect("descriptor missing");
            assert_eq!(descriptor.path[0], "orderbook");
            assert_eq!(descriptor.path[1], "l2");
            assert_eq!(
                DescriptorType::try_from(descriptor.r#type).unwrap(),
                DescriptorType::Path
            );
            descriptor.path[2].clone()
        })
        .collect();
    symbols.sort();
    assert_eq!(symbols, vec!["BTC-USD", "ETH-USD"]);

    // Unknown symbol should return NOT_FOUND for metadata requests
    let err = client
        .get_flight_info(level2_descriptor("UNKNOWN"))
        .await
        .unwrap_err();
    let status = match err {
        arrow_flight::error::FlightError::Tonic(status) => status,
        other => panic!("expected tonic status, got {:?}", other),
    };
    assert_eq!(status.code(), tonic::Code::NotFound);

    server.abort();

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn flight_do_get_unknown_symbol_records_fallback() -> Result<(), Box<dyn Error>> {
    let btc_levels = [Level::new_buy(100, 1), Level::new_sell(101, 1)];
    let feeds = [Feed::new("BTC-USD", &btc_levels)];
    let (server, mut client, _config, metrics) = start_service(&feeds).await?;

    assert_eq!(metrics.fallback_count_for_test(), 0);

    let err = client
        .do_get(Ticket {
            ticket: Bytes::from("UNKNOWN"),
        })
        .await
        .unwrap_err();
    let status = match err {
        arrow_flight::error::FlightError::Tonic(status) => status,
        other => panic!("expected tonic status, got {:?}", other),
    };
    assert_eq!(status.code(), tonic::Code::NotFound);
    assert_eq!(metrics.fallback_count_for_test(), 1);

    server.abort();

    Ok(())
}

#[derive(Clone, Copy)]
struct Level {
    price: u64,
    qty: u64,
    side: Side,
}

impl Level {
    fn new_buy(price: u64, qty: u64) -> Self {
        Self {
            price,
            qty,
            side: Side::Buy,
        }
    }

    fn new_sell(price: u64, qty: u64) -> Self {
        Self {
            price,
            qty,
            side: Side::Sell,
        }
    }
}

struct Feed<'a> {
    symbol: &'a str,
    levels: &'a [Level],
}

impl<'a> Feed<'a> {
    fn new(symbol: &'a str, levels: &'a [Level]) -> Self {
        Self { symbol, levels }
    }
}

async fn start_service(
    feeds: &[Feed<'_>],
) -> Result<
    (
        tokio::task::JoinHandle<()>,
        FlightClient,
        ZeroCopyFlightConfig,
        Arc<ZeroCopyMetrics>,
    ),
    Box<dyn Error>,
> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    let mut config = ZeroCopyFlightConfig::default();
    config.locations = vec![format!("grpc+tcp://{}", addr)];
    config.schema_version = "1.0.0-test".to_string();
    config.depth = 10;

    let metrics = Arc::new(ZeroCopyMetrics::default());
    let flight_service = ZeroCopyFlightService::with_metrics(config.clone(), Arc::clone(&metrics));

    for feed in feeds {
        let orderbook = Arc::new(OrderBook::<()>::new(feed.symbol));
        for level in feed.levels {
            orderbook
                .add_limit_order(
                    OrderId::new(),
                    level.price,
                    level.qty,
                    level.side,
                    TimeInForce::Gtc,
                    None,
                )
                .unwrap();
        }

        flight_service.register_snapshot_provider(
            feed.symbol,
            Arc::new(OrderBookSnapshotProvider::new(
                feed.symbol,
                orderbook.clone(),
            )),
        );
    }

    let incoming = TcpListenerStream::new(listener);
    let server = tokio::spawn(async move {
        Server::builder()
            .add_service(flight_service.into_server())
            .serve_with_incoming(incoming)
            .await
            .unwrap();
    });

    let endpoint = format!("http://{}", addr);
    let channel = Channel::from_shared(endpoint)?.connect().await?;
    let client = FlightClient::new(channel);

    Ok((server, client, config, metrics))
}

fn level2_descriptor(symbol: &str) -> FlightDescriptor {
    FlightDescriptor::new_path(vec![
        "orderbook".to_string(),
        "l2".to_string(),
        symbol.to_string(),
    ])
}
