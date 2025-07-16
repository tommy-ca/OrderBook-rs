# OrderBook-rs Binding Test Documentation

## Test Overview
This document describes the behavior and expected outcomes of the OrderBook-rs binding tests.

## Test Categories

### 1. Unit Tests
- **Python**: `python_tests.py`
- **Node.js**: `node_tests.js`
- **Purpose**: Basic functionality validation

### 2. Integration Tests
- **Python**: `python_integration_tests.py`
- **Node.js**: `node_integration_tests.js`
- **Purpose**: Complex scenarios and edge cases

## Expected Behaviors

### Time in Force (TIF) Orders
- **GTC (Good Till Cancelled)**: Always accepted, remains in book
- **IOC (Immediate Or Cancel)**: Executes immediately against available liquidity
  - If no matching orders exist, throws `InsufficientLiquidity` error
  - Partial fills are allowed, remainder is cancelled
- **FOK (Fill Or Kill)**: Executes fully or not at all
  - If cannot fill completely, throws `InsufficientLiquidity` error

### Error Handling

#### Python Bindings
- **Negative Values**: Throws `OverflowError` due to unsigned integer conversion
- **Invalid Side**: Accepts only "Buy" and "Sell" (case-sensitive)
- **Invalid TIF**: Accepts only "GTC", "IOC", "FOK" (case-sensitive)
- **None Values**: Throws `TypeError`

#### Node.js Bindings
- **Negative Values**: May convert to large positive numbers (JavaScript behavior)
- **Invalid Side**: Accepts only "Buy" and "Sell" (case-sensitive)
- **Invalid TIF**: Accepts only "GTC", "IOC", "FOK" (case-sensitive)
- **null/undefined**: Throws appropriate errors

### Performance Expectations
- **Python**: > 30,000 orders/second
- **Node.js**: > 40,000 orders/second
- **Memory**: Stable usage, no significant leaks

### Validation Behavior
- **Zero Price/Quantity**: Currently accepted by implementation (design decision)
- **Large Values**: Handled up to platform limits
- **Unicode Symbols**: Supported for OrderBook creation

## Test Status

### Passing Tests
- ✅ Basic order operations (add, query)
- ✅ Multiple order handling
- ✅ Performance tests
- ✅ Concurrency tests
- ✅ Memory stability
- ✅ Error handling (most cases)

### Known Limitations
- Some edge cases in parameter validation may vary between implementations
- IOC/FOK behavior depends on existing liquidity
- Platform-specific number handling differences

## Running Tests

### All Tests
```bash
make test-bindings
```

### Python Only
```bash
python3 src/bindings/tests/python_tests.py
python3 src/bindings/tests/python_integration_tests.py
```

### Node.js Only
```bash
node src/bindings/tests/node_tests.js
node src/bindings/tests/node_integration_tests.js
```

## Test Maintenance

### Adding New Tests
1. Add to appropriate test file
2. Update this documentation
3. Run full test suite to ensure no regressions

### Debugging Test Failures
1. Check expected vs actual behavior
2. Verify platform-specific differences
3. Update test expectations if behavior is correct

## Performance Monitoring
Tests include performance benchmarks to detect regressions:
- Order insertion rate
- Query response time
- Memory usage patterns

Regular monitoring ensures the bindings maintain high performance standards.