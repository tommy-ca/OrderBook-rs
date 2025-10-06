## Why
The repository lacks an authoritative OpenSpec description of the lock-free order book engine. Capturing the existing behavior prevents regressions and aligns future changes with the implemented architecture.

## What Changes
- Document core order book concurrency, order indexing, order matching, and validation semantics in `orderbook-engine`
- Capture book management trade routing in `book-manager` and mutation workflows in `orderbook-modifications`
- Describe snapshot packaging, checksum validation, and metadata guarantees
- Establish OpenSpec project context so agents share conventions and constraints

## Impact
- Affected specs: introduces `openspec/specs/orderbook-engine/spec.md`, `openspec/specs/book-manager/spec.md`, `openspec/specs/orderbook-modifications/spec.md`
- Affected code: documents current behavior of modules in `src/orderbook/`
- Tooling: enables OpenSpec validation for future proposals touching order management and multi-book coordination
