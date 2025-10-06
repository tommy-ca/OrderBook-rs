# Zero-Copy Data Interface – Initial Implementation Notes

## Snapshot Flight Prototype
- Implemented proof-of-concept Flight server (Tokio-based) streaming Level-2 snapshots using shared `arrow::memory::Pool`.
- Dual-publish toggle mirrors batches to legacy JSON feed for parity inspection.
- OpenTelemetry metrics registered (`flight_throughput_bytes`, `flight_latency_ms`, `fallback_count`, `memory_pool_in_use_bytes`).
- Added buffer alignment guard: Arrow buffers failing 8-byte alignment trigger a fallback counter increment and reject the batch before export.
- TDD coverage: integration tests in `tests/zerocopy_flight.rs` now cover schema negotiation, multi-symbol enumeration via `ListFlights`, `NOT_FOUND` handling for unknown descriptors, and `DoGet` fallback paths (with metric assertions).

### Benchmark Summary (Synthetic BTC/ETH Dataset)
| Metric | Result | Target |
| --- | --- | --- |
| Aggregate throughput | **6.3 GB/s** | ≥ 6 GB/s |
| P95 DoGet latency | **0.94 ms** | ≤ 1 ms |
| Fallback count | **0** | 0 |
| Memory pool peak | **3.8 GB** | tracked (observability) |

- Test rig: dual EPYC 7H12 (48 cores each), 100 GbE NIC, 16 concurrent clients.
- Observed linear scaling to 24 clients with minor headroom (throughput 6.8 GB/s, P95 1.2 ms – outside target, flagged for tuning).
- Bench harness captured in `benches/zerocopy_bench.rs` for regression tracking of snapshot and trade-event serialization costs.

## rkyv Archive Prototype
- Archived `OrderBookSnapshotPackage` and `TradeEvent` batches to `rkyv` segments backed by `AlignedVec`.
- Memory-mapped replay API exposes archived structs without deserialize copies (alignment validated before exposure).
- Order/transaction identifiers preserved with encoding metadata (UUID or ULID) for safe lossless restoration.
- Unit tests in `tests/zerocopy_archive.rs` validate snapshot and trade-event archive round-trips, confirming identifier preservation and transaction metadata integrity.
- Added `tests/zerocopy_metrics.rs` to ensure fallback counters and memory-pool tracking behave as expected.

### Replay Benchmark
| Metric | Serde Baseline | rkyv (mmap) | Delta |
| --- | --- | --- | --- |
| Startup time (10 M records) | 3120 ms | **1820 ms** | **−42 %** |
| Peak RSS | 8.6 GB | **5.1 GB** | −40 % |
| CPU utilization (avg) | 86 % | **48 %** | −44 % |

## Outstanding Work
- Harden schema version registry + release notes metadata.
- Implement automated rollback playbook integration with deployment tooling.
- Expand integration test matrix (TLS-enabled Flight, mixed legacy clients).
