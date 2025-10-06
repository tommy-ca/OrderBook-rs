use opentelemetry::{
    KeyValue, global,
    metrics::{Counter, Histogram, Unit, UpDownCounter},
};
use std::borrow::Cow;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::time::Duration;

/// Configuration for zero-copy metrics instrumentation.
#[derive(Clone, Debug)]
pub struct ZeroCopyMetricsConfig {
    /// OpenTelemetry instrumentation scope name.
    pub instrumentation_name: Cow<'static, str>,
    /// Attributes applied to every metric event.
    pub attributes: Vec<KeyValue>,
}

impl Default for ZeroCopyMetricsConfig {
    fn default() -> Self {
        Self {
            instrumentation_name: Cow::Borrowed("orderbook.zero_copy"),
            attributes: Vec::new(),
        }
    }
}

/// Helper for recording zero-copy throughput, latency, and fallback telemetry.
pub struct ZeroCopyMetrics {
    throughput_bytes: Counter<u64>,
    latency_ms: Histogram<f64>,
    fallback_count: Counter<u64>,
    pool_usage: UpDownCounter<i64>,
    attributes: Arc<[KeyValue]>,
    pool_state: AtomicI64,
    fallback_counter: AtomicU64,
}

impl ZeroCopyMetrics {
    /// Create a new metrics helper using the provided configuration.
    pub fn new(config: ZeroCopyMetricsConfig) -> Self {
        let meter = global::meter(config.instrumentation_name);
        let attributes: Arc<[KeyValue]> = Arc::from(config.attributes.into_boxed_slice());

        let throughput_bytes = meter
            .u64_counter("flight_throughput_bytes")
            .with_description("Total bytes streamed via Arrow Flight DoGet")
            .with_unit(Unit::new("By"))
            .init();

        let latency_ms = meter
            .f64_histogram("flight_latency_ms")
            .with_description("DoGet batch latency (ms)")
            .with_unit(Unit::new("ms"))
            .init();

        let fallback_count = meter
            .u64_counter("flight_fallback_count")
            .with_description("Number of zero-copy fallbacks to copy-based path")
            .init();

        let pool_usage = meter
            .i64_up_down_counter("flight_memory_pool_bytes")
            .with_description("Approximate Arrow memory pool bytes in use")
            .with_unit(Unit::new("By"))
            .init();

        Self {
            throughput_bytes,
            latency_ms,
            fallback_count,
            pool_usage,
            attributes,
            pool_state: AtomicI64::new(0),
            fallback_counter: AtomicU64::new(0),
        }
    }

    /// Record a successful snapshot export.
    pub fn observe_snapshot_export(&self, bytes: u64, latency: Duration) {
        self.throughput_bytes.add(bytes, &self.attributes);
        self.latency_ms
            .record(latency.as_secs_f64() * 1_000.0, &self.attributes);
    }

    /// Record that the pipeline fell back to a copy-based path.
    pub fn record_fallback(&self) {
        self.fallback_count.add(1, &self.attributes);
        self.fallback_counter.fetch_add(1, Ordering::Relaxed);
    }

    /// Update the observed memory pool usage in bytes.
    pub fn record_pool_bytes(&self, bytes: i64) {
        let previous = self.pool_state.swap(bytes, Ordering::Relaxed);
        let delta = bytes.saturating_sub(previous);
        if delta != 0 {
            self.pool_usage.add(delta, &self.attributes);
        }
    }
}

impl Default for ZeroCopyMetrics {
    fn default() -> Self {
        Self::new(ZeroCopyMetricsConfig::default())
    }
}

impl ZeroCopyMetrics {
    /// Testing helper exposing the number of recorded fallbacks.
    pub fn fallback_count_for_test(&self) -> u64 {
        self.fallback_counter.load(Ordering::Relaxed)
    }

    /// Testing helper exposing the currently tracked memory usage.
    pub fn pool_bytes_for_test(&self) -> i64 {
        self.pool_state.load(Ordering::Relaxed)
    }
}
