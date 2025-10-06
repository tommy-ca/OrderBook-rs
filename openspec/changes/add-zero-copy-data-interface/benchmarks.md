# Zero-Copy Performance Benchmarks (Initial Plan)

## Snapshot Streaming Harness
- **Topology**: dual-socket AMD EPYC 7H12, 100 GbE NIC, Tokio-based Flight server, 16 concurrent rust clients using `arrow-flight` DoGet.
- **Dataset**: synthetic BTC/ETH depth, 50 k price levels, 10 M rows per batch, refresh every 250 ms.
- **Metrics Captured**:
  - Throughput (GB/s aggregate)
  - 50/95/99th percentile DoGet batch latency
  - Memory pool outstanding bytes per shard
  - Fallback counts (copy-based path invocations)
- **Acceptance**: ≥6 GB/s aggregate, ≤1 ms P95 latency, 0 fallbacks during steady-state.

## Dual Publish Parity Check
- Compare Arrow batches vs legacy JSON feed by sequence ID; report mismatch rate (target 0) and average downstream decode time.
- Store parity summaries alongside CI artefacts for canary runs.

## Replay Benchmarks
- **Baseline**: serde JSON snapshot deserialization of 10 M-record archive.
- **Variant**: `rkyv` memory-mapped replay of identical dataset.
- Measure startup time (ms), peak RSS, and CPU utilization.
- Target ≥40 % faster startup and ≤60 % of baseline CPU cost.

## Telemetry Verification
- Validate OpenTelemetry exporters emit: `flight_throughput_bytes`, `flight_latency_ms`, `fallback_count`, `memory_pool_in_use_bytes`, `rkyv_replay_startup_ms`.
- Ensure dashboards compare zero-copy vs legacy metrics during canary period.
