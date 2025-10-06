use orderbook_rs::zerocopy::{ZeroCopyMetrics, ZeroCopyMetricsConfig};

#[test]
fn metrics_records_fallback_and_pool_state() {
    let metrics = ZeroCopyMetrics::new(ZeroCopyMetricsConfig::default());

    metrics.record_pool_bytes(256);
    assert_eq!(metrics.pool_bytes_for_test(), 256);

    metrics.record_pool_bytes(128);
    assert_eq!(metrics.pool_bytes_for_test(), 128);

    assert_eq!(metrics.fallback_count_for_test(), 0);
    metrics.record_fallback();
    metrics.record_fallback();
    assert_eq!(metrics.fallback_count_for_test(), 2);
}
