# Project Context

## Purpose
Provide a high-performance, lock-free limit order book engine in Rust that can serve as the matching core for exchange simulators, trading systems, and research tooling.

## Tech Stack
- Rust 2024 edition crate published as `orderbook-rs`
- Concurrency primitives: `DashMap`, atomics, thread-local pools
- Supporting crates: `pricelevel` (per-price data structure), `uuid`, `tokio` (sync utilities), `serde`
- Planned zero-copy interfaces: Apache Arrow Flight, `arrow-array`, `arrow-flight`, and `rkyv`
- Instrumentation via `tracing`

## Project Conventions

### Code Style
- Follow `rustfmt` defaults; run via `cargo fmt` or `make fmt`
- Clippy linting enforced with `make lint`
- Prefer explicit structs and enums over tuples for domain data

### Architecture Patterns
- Core engine lives under `src/orderbook/` with modules for book state, matching, operations, modifications, snapshots, and pools
- Order data stored per-price-level using `PriceLevel` (from `pricelevel` crate) behind `Arc`
- Lock-free concurrency: read/write via `DashMap` and atomic flags, thread-local pools reuse allocations for matching
- Optional trade listener callback allows external systems to observe fills

### Testing Strategy
- Unit tests under `src/orderbook/tests/` and `tests/unit`
- Benchmarks driven by Criterion in `benches/`
- CI expects `make check` (fmt + lint + tests) before merging

### Git Workflow
- Default branch: `main`
- New behavior or architecture changes require an OpenSpec change proposal prior to implementation
- Use descriptive branch names that mirror OpenSpec change IDs (e.g., `feature/add-depth-snapshots`)
- Conventional commits are encouraged but not mandated; keep commit messages implementation-focused

## Domain Context
- Engine must support multiple advanced order types (limit, iceberg, post-only, trailing stop, etc.)
- Target workloads involve millions of orders with high concurrency and tight latency budgets
- Consumers rely on deterministic matching rules consistent with traditional exchange behavior (price-time priority)

## Important Constraints
- Maintain lock-free behavior; avoid introducing global locks or blocking cross-thread waits
- Preserve FIFO ordering of orders within each price level
- API consumers depend on `OrderBook` being `Send + Sync` safe
- Avoid heap churn in hot paths; leverage pooling utilities when extending matching logic

## External Dependencies
- `pricelevel` crate supplies low-level price level data structure and matching primitives
- Optional integration points (trade listeners, snapshots) serialize data via `serde`
- Planned zero-copy data plane depends on Arrow ecosystem crates (`arrow-array`, `arrow-flight`, `arrow-ipc`) and `rkyv`
