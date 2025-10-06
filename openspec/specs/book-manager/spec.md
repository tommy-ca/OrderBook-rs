# Book Manager Specification

## Requirement: Register Books by Symbol
The system SHALL allow creation, lookup, and removal of `OrderBook` instances keyed by trading symbol.

#### Scenario: Add new book with listener
- WHEN `add_book(symbol)` is invoked
- THEN a fresh `OrderBook` is created with a trade listener that forwards executions through the manager’s channel
- AND the symbol becomes visible via `symbols()` and `book_count()` increment.

#### Scenario: Retrieve mutable handle
- WHEN `get_book_mut(symbol)` is called for an existing symbol
- THEN the method returns a mutable reference to the underlying `OrderBook`
- AND callers may submit or cancel orders using the same interface as standalone books.

#### Scenario: Remove book cleans registry
- WHEN `remove_book(symbol)` is invoked
- THEN the manager returns the removed `OrderBook` (if present) and deletes the symbol from internal maps
- AND `has_book(symbol)` subsequently returns false.

## Requirement: Route Trades via Event Channels
The system SHALL deliver trade events emitted by managed books over the configured channel implementation.

#### Scenario: Emit event from standard manager
- GIVEN `BookManagerStd` has started its trade processor thread
- WHEN a managed order book produces a `TradeResult`
- THEN the associated `TradeEvent` is enqueued on the std `mpsc::Sender`
- AND the processor thread logs the trade summary and iterates over individual transactions.

#### Scenario: Emit event from tokio manager
- GIVEN `BookManagerTokio` has started its async processor
- WHEN a managed order book produces a `TradeResult`
- THEN a `TradeEvent` is sent through the `tokio::sync::mpsc::UnboundedSender`
- AND the async task logs the executed quantity per transaction.

## Requirement: Prevent Duplicate Processor Startup
The system SHALL ensure trade processors start exactly once per manager instance.

#### Scenario: Guard std processor start
- WHEN `start_trade_processor` is called a second time on `BookManagerStd`
- THEN the method panics with "Trade processor already started"
- AND no additional thread is spawned.

#### Scenario: Guard tokio processor start
- WHEN `start_trade_processor` is called after a Tokio manager has already taken ownership of its receiver
- THEN the method panics with "Trade processor already started"
- AND no new async task is launched.

