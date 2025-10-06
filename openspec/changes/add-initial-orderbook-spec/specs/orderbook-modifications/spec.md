# Delta for OrderBook Modifications Specification

## ADDED Requirements
### Requirement: Update Order Price In-Place
The system SHALL allow updating an order’s price by cancelling the resting entry and re-inserting it at the new price while preserving its identifier and metadata.

#### Scenario: Reject no-op price update
- WHEN `update_order(UpdatePrice { new_price })` is called with the same resting price
- THEN the engine rejects the update with `OrderBookError::InvalidOperation`
- AND the order remains at its original price level.

#### Scenario: Rehome order to new price
- WHEN `update_order` is invoked with a different price
- THEN the engine cancels the old entry, updates the price field, and inserts the order at the new level
- AND `order_locations` maps the order ID to the new `(price, side)` pair.

### Requirement: Update Order Quantity Safely
The system SHALL resize order quantities without violating type-specific rules or corrupting price levels.

#### Scenario: Shrink quantity within level
- WHEN quantity decreases via `UpdateQuantity`
- THEN the engine updates the order in place inside its price level and retains FIFO position.

#### Scenario: Adjust reserve order slices
- WHEN a reserve order quantity is reduced
- THEN the engine decreases visible quantity first, then hidden quantity, performing replenishment if visible becomes zero while hidden remains.

### Requirement: Replace Orders Atomically
The system SHALL support `Replace` updates that rewrite price, quantity, and side fields in a single operation.

#### Scenario: Replace order keeps identifier
- WHEN `UpdatePriceAndQuantity` or `Replace` is invoked
- THEN the resulting resting order retains the original `OrderId`
- AND `order_locations` reflects the new placement.

### Requirement: Cancel Orders by Identifier
The system SHALL cancel orders in O(1) using the location index and clean up empty price levels.

#### Scenario: Cancel removes location tracking
- WHEN `cancel_order(id)` succeeds
- THEN the order is removed from the `PriceLevel`, `order_locations`, and price level map if empty
- AND cache entries for best prices are invalidated.

#### Scenario: Cancel missing order is a no-op
- WHEN cancellation targets an unknown `OrderId`
- THEN the method returns `Ok(None)` without modifying book state.

