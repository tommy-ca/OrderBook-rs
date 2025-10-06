# Zero-Copy Data Interface Design

## Goals
- Deliver depth-of-book snapshots and deltas with <1 ms tail latency using Apache Arrow Flight.
- Persist and replay snapshots/trade events via `rkyv` archives with ≥2× faster startup than serde.
- Maintain observability, migration safety, and rollback controls while introducing the zero-copy pipeline.

## Architecture Overview
1. **Snapshot Producer**
   - Extend existing `OrderBookSnapshot` exporter to produce Arrow `RecordBatch`es backed by a shared `arrow::memory::Pool`.  
   - Level 2 schema (per spec): `symbol`, `ts_event`, `bid_px`, `bid_qty`, `bid_count`, `ask_px`, `ask_qty`, `ask_count`.
   - Delta schema: `symbol`, `ts_event`, `level`, `side`, `price`, `quantity`, `action` (`NEW`, `UPDATE`, `DELETE`).
   - Batches tagged with `schema_semver` and `sequence` metadata for dual-publish parity checks.

2. **Flight Service**
   - gRPC server exposing `/flight/orderbook/l2` and `/flight/orderbook/delta` descriptors.
   - `FlightInfo` advertises shard-aware `FlightEndpoint`s; heavy symbols distribute across multiple DoGet URLs.
   - `PollFlightInfo` implementation allows clients to poll long snapshot jobs while batches assemble in memory.
   - Enforces per-tenant quotas via custom `TaggedMemoryPool` wrapper instrumented with OpenTelemetry gauges/counters.

3. **Legacy Pipeline Coordination**
   - `DualPublishController` duplicates outgoing batches into legacy JSON/gRPC endpoints while zero-copy is in canary mode.
   - Single configuration flag toggles Flight/rkyv exporters. Rollback path drains in-flight batches, flushes metrics, reverts to legacy within 60 s.

4. **rkyv Archive Layer**
   - Serialize `OrderBookSnapshotPackage` and `TradeEvent` into append-only segments using `rkyv::AlignedVec` buffers.
   - Archives stored with checksum + schema version metadata for replay compatibility.
   - `ArchiveReplay` module memory-maps segments, exposes archived structs directly, and validates alignment before use.

5. **Telemetry & Observability**
   - Metrics: `flight_throughput_bytes`, `flight_latency_ms_p95`, `flight_fallback_count`, `memory_pool_bytes_in_use`, `rkyv_replay_startup_ms`.
   - Traces annotate batch lifecycles (snapshot build → batch export → dual publish). Alerts trip when throughput <6 GB/s or latency >1 ms for sustained intervals.

## Implementation Phases
1. **Phase A – Foundations**
   - Integrate Arrow + Flight crates, create memory pool abstraction.
   - Implement schema builders + metadata versioning.
   - Scaffold dual-publish toggles and placeholder metrics.

2. **Phase B – Zero-Copy Exporters**
   - Wire snapshot producer to Flight `DoGet` streams (L2 + deltas).
   - Add rkyv serialization for snapshots and trade events.
   - Ensure alignment and immutability checks prior to exposure.

3. **Phase C – Validation & Rollout**
   - Build benchmark harness (Tokio runtime + load generator) measuring throughput/latency across 16 concurrent clients.
   - Capture serde vs rkyv replay timings (memory-mapped vs deserialize).
   - Enable telemetry dashboards and set up regression alerts.
   - Perform dual-publish canary, compare sequence parity, document rollback steps.

## Open Questions / Follow-Ups
- Evaluate whether to store schema history in Git-tracked directory or dedicated registry service.
- Confirm compatibility of Arrow Flight security posture with production ACL requirements (TLS + mutual auth).
- Determine retention policy for rkyv archives and storage tiering.
