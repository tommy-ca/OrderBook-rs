//! Zero-copy data transport and archival utilities.

pub mod archive;
pub mod flight;
pub mod metrics;

pub use archive::{
    OrderIdArchive, SnapshotArchive, TradeEventArchive, deserialize_snapshot_archive,
    deserialize_trade_event_archive, serialize_snapshot_archive, serialize_trade_event_archive,
    uuid_from_bytes,
};
pub use flight::{
    MemoryPoolStats, OrderBookSnapshotProvider, SnapshotProvider, ZeroCopyFlightConfig,
    ZeroCopyFlightService,
};
pub use metrics::{ZeroCopyMetrics, ZeroCopyMetricsConfig};
