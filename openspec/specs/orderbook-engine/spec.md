# OrderBook Engine Specification

## Requirement: Manage Price Levels Concurrently
The system SHALL store bid and ask books in concurrent maps so multiple threads can mutate independent price levels without blocking.

#### Scenario: Add order to empty price level
- WHEN an order is accepted at a price with no existing level
- THEN the corresponding `DashMap` gains a new `PriceLevel` entry backed by `Arc`
- AND other threads MAY enqueue additional orders against that level without coordination locks.

#### Scenario: Remove depleted price level
- WHEN the final order at a price level is matched or cancelled
- THEN the engine removes the empty `PriceLevel` from the map
- AND downstream iterators do not observe the depleted price level on subsequent reads.

## Requirement: Track Orders by Identifier
The system SHALL maintain an `order_locations` index so orders can be located or cancelled in O(1) time by `OrderId`.

#### Scenario: Record accepted order location
- WHEN an order is inserted into the book
- THEN `order_locations` stores the order’s price and side keyed by its `OrderId`
- AND cancellation or modification requests use the stored location instead of scanning price levels.

#### Scenario: Release location on exit
- WHEN an order fully fills or is cancelled
- THEN the engine removes the associated `OrderId` entry from `order_locations`
- AND subsequent lookups for that `OrderId` report it as absent.

## Requirement: Preserve Price-Time Priority During Matching
The system SHALL match resting orders using price-time priority, iterating best prices first and honoring FIFO within each `PriceLevel`.

#### Scenario: Execute market buy with available asks
- WHEN a buy market order is submitted while ask levels exist
- THEN the engine iterates ask prices in ascending order, withdrawing quantity according to arrival order
- AND it records transactions in the returned `MatchResult` while updating the last trade price.

#### Scenario: Respect limit price guardrails
- WHEN a limit order match would cross beyond its limit price
- THEN matching stops before violating the price constraint
- AND the remaining quantity is reported as unfilled in the `MatchResult`.

#### Scenario: Surface insufficient liquidity errors
- WHEN a market order cannot fill any quantity because the opposite book is empty
- THEN the engine returns `OrderBookError::InsufficientLiquidity` with the requested and available quantities.

## Requirement: Maintain Trade Metadata
The system SHALL update trade metadata whenever executions occur so downstream consumers can observe fills.

#### Scenario: Update last trade price flag
- WHEN a trade executes at price `p`
- THEN `last_trade_price` stores `p` and `has_traded` flips to true using relaxed atomics
- AND future snapshot serialization surfaces the new values.

#### Scenario: Notify trade listener
- WHEN a trade listener callback is configured and a trade executes
- THEN the engine forwards each generated transaction to the listener for side-effect processing.

## Requirement: Support Advanced Order Types
The system SHALL persist and process all supported `OrderType` variants while preserving their type-specific semantics.

#### Scenario: Store standard order quantities
- WHEN a standard limit order is accepted
- THEN its `quantity` field determines both matching quantity and resting remainder
- AND conversions between generic and unit types retain the supplied `extra_fields` default values.

#### Scenario: Maintain iceberg visible slices
- WHEN an iceberg order rests in the book
- THEN the visible slice is stored in the price level while hidden quantity remains off-book
- AND replenishment logic only refills visible quantity when matching activity depletes it.

#### Scenario: Replenish reserve orders
- WHEN a reserve order’s visible slice reaches zero while hidden quantity remains
- THEN the engine replenishes the visible slice using the configured `replenish_amount`
- AND matching continues without requiring external intervention.

## Requirement: Enforce Order Entry Constraints
The system SHALL reject or adjust orders that violate crossing, expiry, or fill semantics before mutating book state.

#### Scenario: Reject crossing post-only order
- WHEN a post-only order would immediately cross the opposite best price
- THEN the engine rejects the submission with `OrderBookError::PriceCrossing`
- AND the book state remains unchanged.

#### Scenario: Guard fill-or-kill capacity
- WHEN a fill-or-kill order cannot be fully satisfied at or within its limit
- THEN the engine returns `OrderBookError::InsufficientLiquidity`
- AND no partial execution or book insertion occurs.

#### Scenario: Skip expired order
- WHEN an order arrives with `time_in_force` indicating it is already expired
- THEN the engine rejects the order with `OrderBookError::InvalidOperation`
- AND no order state or cache entries change.

## Requirement: Maintain Matching Pools and Caches
The system SHALL reuse memory pools and invalidate caches consistently to keep hot-path operations deterministic.

#### Scenario: Reuse thread-local buffers
- WHEN the matcher processes orders
- THEN it obtains reusable vectors from `MatchingPool` thread-locals and returns them after the match concludes
- AND no heap allocations leak across iterations.

#### Scenario: Invalidate price cache on mutation
- WHEN orders are added, updated, or cancelled
- THEN the engine invalidates `PriceLevelCache` so subsequent best bid/ask queries recompute from source data
- AND snapshot exports reflect the updated top-of-book values.

## Requirement: Provide Snapshot Serialization
The system SHALL export deterministic snapshots of book state for diagnostics and recovery.

#### Scenario: Serialize book snapshot
- WHEN `snapshot()` is invoked
- THEN the engine collects bid and ask `PriceLevel` snapshots plus order location metadata into an `OrderBookSnapshot`
- AND serialization encodes atomic fields (last price, market close) and checksum for integrity checks.
