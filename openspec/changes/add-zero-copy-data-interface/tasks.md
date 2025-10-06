## 1. Design & Dependencies
- [x] 1.1 Evaluate Arrow Flight Rust bindings and select memory pool strategy _(see design.md §Snapshot Producer)_
- [x] 1.2 Define Arrow schema for order book snapshots (bids, asks, metadata)
- [x] 1.3 Plan rkyv archive structures for snapshots and trade events

## 2. Implementation
- [x] 2.1 Implement Flight server endpoints providing zero-copy RecordBatch snapshots
- [x] 2.2 Integrate schema versioning and discovery endpoints
- [x] 2.3 Add rkyv serialization for `OrderBookSnapshotPackage` and `TradeEvent`
- [x] 2.4 Enforce buffer alignment and immutability checks before publication
- [x] 2.5 Implement dual-publish controller and rollback toggle _(prototype; see results.md)_
- [x] 2.6 Instrument OpenTelemetry metrics for throughput, latency, fallbacks, memory pool usage _(OTLP exporter live in prototype)_
- [x] 2.7 Build benchmark harness per `benchmarks.md`

## 3. Validation
- [x] 3.1 Benchmark snapshot export latency to confirm < 1 ms target at 50k price levels _(P95 0.94 ms)_
- [x] 3.2 Add integration tests covering Flight schema negotiation and rkyv round-trips
- [x] 3.3 Run `openspec validate add-zero-copy-data-interface --strict`
- [ ] 3.4 Capture baseline vs zero-copy telemetry comparison during canary rollout
