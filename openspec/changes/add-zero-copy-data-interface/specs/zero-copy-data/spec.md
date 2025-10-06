# Delta for Zero-Copy Data Interfaces Specification

## ADDED Requirements
### Requirement: Stream Snapshots via Arrow Flight
The system SHALL expose order book snapshots as Apache Arrow RecordBatches published over Arrow Flight without intermediate copies.

#### Scenario: Export Level 2 snapshot
- WHEN a client requests a book snapshot through the Flight service
- THEN the engine constructs a `RecordBatch` directly from in-memory price levels using zero-copy buffers
- AND the generated Flight stream delivers bid and ask data in separate zero-copy `StructArray` columns.

#### Scenario: Reuse allocation pool
- WHEN multiple snapshot requests run concurrently
- THEN the Flight producer reuses a shared memory pool to avoid redundant allocations
- AND latency remains under the target 1 ms budget for Level 2 exports at 50k levels.

### Requirement: Provide Arrow Flight Schema Discovery
The system SHALL serve schema introspection endpoints so consumers can negotiate snapshot layouts.

#### Scenario: Expose schema endpoint
- WHEN a client invokes `GetFlightInfo` without ticket data
- THEN the service responds with Flight descriptors describing available snapshot feeds and Arrow schema metadata (field names, encodings)
- AND the schema version matches the spec revision recorded in OpenSpec change history.

#### Scenario: Notify schema changes
- WHEN the snapshot schema evolves (e.g., new field for imbalance)
- THEN the Flight service increments the schema version and surfaces change notes in the `FlightInfo` response
- AND downstream clients can compare versions before consuming data.

#### Scenario: Reject unknown descriptors
- WHEN `GetFlightInfo` or `GetSchema` is called with a symbol or prefix that does not exist
- THEN the service returns a `NOT_FOUND` error without streaming data
- AND the failure is logged so operators can diagnose descriptor mismatches.

#### Scenario: Enumerate registered feeds
- WHEN a client calls `ListFlights`
- THEN the response includes a descriptor per registered symbol with `PATH` layout `["orderbook", "l2", <symbol>]`
- AND descriptors omit command payloads to keep feeds discoverable across clients.

#### Scenario: Reject unknown tickets
- WHEN `DoGet` is invoked with a ticket referencing an unknown or deregistered symbol
- THEN the service responds with `NOT_FOUND` without buffering any RecordBatches
- AND increments fallback telemetry so operators can detect misrouted requests.

### Requirement: Define Snapshot Schemas and Versioning
The system SHALL publish versioned Arrow schemas for at least Level 2 depth-of-book snapshots and incremental trade deltas.

#### Scenario: Level 2 schema contract
- WHEN `GetFlightInfo` is called for `orderbook/l2` streams
- THEN the service returns a schema containing columns `[symbol (utf8), ts_event (timestamp[ns]), bid_px (float64), bid_qty (float64), bid_count (int32), ask_px (float64), ask_qty (float64), ask_count (int32)]`
- AND any schema change (add/remove column) increments the semantic version and provides release notes in application metadata.

#### Scenario: Incremental delta schema
- WHEN clients request `orderbook/delta` streams
- THEN the schema includes `[symbol, ts_event, level, side, price, quantity, action]` with `action` encoded as dictionary-encoded strings (`NEW`, `UPDATE`, `DELETE`)
- AND the service documents allowed enum values so downstream consumers can validate payloads.

### Requirement: Horizontal Scaling via Flight Endpoints
The system SHALL leverage multiple Flight endpoints to parallelize data transport for large order books.

#### Scenario: Publish shard endpoints
- WHEN `GetFlightInfo` is called for high-volume symbols
- THEN the response includes multiple `FlightEndpoint` entries partitioned by shard identifier, enabling clients to run concurrent `DoGet` calls
- AND each endpoint advertises throughput targets (e.g., 20+ Gbit/s per core) in application metadata.

#### Scenario: Poll for long-running queries
- WHEN generating a snapshot requires upstream aggregation
- THEN clients MAY use `PollFlightInfo` to monitor job completion without blocking
- AND the service preserves zero-copy semantics once batches become available.

### Requirement: Serialize Snapshots and Events with rkyv
The system SHALL provide zero-copy binary serialization of snapshots and trade events using `rkyv` for storage and replay pipelines.

#### Scenario: Archive snapshot to rkyv
- WHEN archiving an intraday snapshot
- THEN the engine serializes the `OrderBookSnapshotPackage` with `rkyv` and writes the resulting bytes directly to storage
- AND restoring the snapshot avoids deserialization copies by leveraging archived struct references.

#### Scenario: Stream trades via rkyv frames
- WHEN broadcasting trade events for high-frequency consumers
- THEN the manager encodes each `TradeEvent` into a rkyv frame and publishes it over the configured transport without allocating intermediate JSON representations.

#### Scenario: Memory-map archives
- WHEN replay jobs need historical data
- THEN the system allows `mmap`-based access to rkyv archives, exposing archived structs directly without materialization while honoring lifetime guarantees
- AND replay benchmarks demonstrate ≥2× faster deserialize/startup time versus the existing serde-based pipeline for 10M-record test fixtures.

### Requirement: Maintain Zero-Copy Safety Guarantees
The system SHALL validate alignment, lifetime, and mutability constraints for zero-copy buffers before exposing them externally.

#### Scenario: Validate alignment before publish
- WHEN constructing Arrow buffers from internal structures
- THEN the engine verifies pointer alignment and capacity matches the declared Arrow datatype requirements
- AND rejects publication with a diagnostic error if the validation fails.

#### Scenario: Guard shared buffers from mutation
- WHEN a buffer is shared with external consumers through Flight or rkyv archives
- THEN the engine marks the data as immutable for the duration of the transfer
- AND any attempt to mutate the source data results in cloning or raising `OrderBookError::InvalidOperation`.

#### Scenario: Fallback with telemetry
- WHEN zero-copy transfer is not possible (e.g., padding consolidation bug)
- THEN the service falls back to copy-based transfer, emits warnings, and reports the affected batch size so operators can react.

#### Scenario: Record fallback metrics
- WHEN the service triggers a fallback
- THEN it increments `flight_fallback_count` and records the batch size so dashboards reflect degraded runs.

### Requirement: Memory Pool Governance
The system SHALL standardize on Arrow `MemoryPool` interfaces to track allocations and enforce per-tenant budgets.

#### Scenario: Pool-aware allocation
- WHEN Flight producers allocate RecordBatch buffers
- THEN they use tagged `MemoryPool` instances so allocator metrics (bytes outstanding, max) are exposed for observability dashboards.

#### Scenario: Reclaim on disconnect
- WHEN a client disconnects mid-stream
- THEN the server releases buffers back to the pool immediately and logs the reclaimed capacity.

### Requirement: Performance and Observability Targets
The system SHALL monitor throughput and latency for zero-copy transports and enforce regression thresholds.

#### Scenario: Flight throughput SLO
- WHEN benchmarking snapshot exports on reference hardware (dual-socket 100 GbE servers)
- THEN the system sustains ≥6 GB/s aggregate DoGet throughput with ≤1 ms 95th percentile latency under 16 concurrent streams
- AND breaches trigger automated alerts and rollbacks to the last known good build.

#### Scenario: rkyv replay benchmarks
- WHEN loading archived trade events via `mmap`
- THEN the system demonstrates at least 40% faster startup versus serde-based deserialization baselines
- AND benchmark reports are stored alongside the change artefacts.

#### Scenario: Emit zero-copy telemetry
- WHEN zero-copy transports are active
- THEN the system exports OpenTelemetry metrics for throughput, tail latency, fallback counts, and memory-pool utilization for each feed
- AND dashboards compare these metrics against the retired JSON/gRPC pipeline to track migration impact.

### Requirement: Manage Legacy Serialization Migration
The system SHALL provide a controlled rollout from the existing JSON/gRPC pipelines to the zero-copy Arrow Flight and rkyv workflows.

#### Scenario: Dual-publish during migration
- WHEN the zero-copy pipeline is enabled for a symbol
- THEN the system continues publishing the legacy feed in parallel, tagging both outputs with identical sequence numbers so downstream clients can validate parity before cutover.

#### Scenario: Safe rollback
- WHEN operators detect blocking issues in zero-copy delivery
- THEN a configuration toggle disables the Flight/rkyv exporters and reverts to the legacy pipeline within 60 seconds without restarting matching services.
