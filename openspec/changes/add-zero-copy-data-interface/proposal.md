## Why
Clients need zero-copy access to order book data for analytics, archival, and low-latency event processing. Publishing snapshots and trade streams over Arrow Flight and rkyv eliminates redundant serialization work and reduces latency.

## What Changes
- Introduce a Flight-based snapshot service that streams Arrow RecordBatches sourced directly from in-memory book state
- Provide schema negotiation and versioning so downstream consumers can evolve safely
- Add rkyv-based binary serialization for snapshots and trade events to support replay pipelines and fan-out transports
- Enforce safety rules (alignment, immutability) for zero-copy buffers shared outside the matching engine

## Impact
- Affected specs: adds `openspec/specs/zero-copy-data/spec.md`
- Affected code: touches snapshot export pipeline, trade manager publishing, and any new Arrow Flight server implementation
- Tooling: requires Arrow and rkyv dependencies plus benchmarking to validate latency budgets
