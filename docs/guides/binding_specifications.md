# OrderBook-rs Binding Specifications

## Overview
This document specifies the Python and Node.js bindings for the OrderBook-rs library, providing a high-performance, lock-free price level implementation for limit order books.

## Architecture

### Core Components
- **Rust Core**: High-performance OrderBook implementation with lock-free operations
- **Python Bindings**: PyO3-based bindings providing Python-friendly interface
- **Node.js Bindings**: NAPI-RS-based bindings for JavaScript/TypeScript integration

### Performance Targets
- **Rust Core**: > 1M operations/second
- **Python Bindings**: > 100K operations/second
- **Node.js Bindings**: > 500K operations/second

## Python Binding Specification

### Installation
```bash
pip install orderbook-rs
```

### API Reference

#### PyOrderBook Class
```python
class PyOrderBook:
    def __init__(self, symbol: str) -> None
    def add_limit_order(self, price: int, quantity: int, side: str, time_in_force: str) -> str
    def best_bid(self) -> Optional[int]
    def best_ask(self) -> Optional[int]
```

#### Parameters
- **symbol**: Trading symbol (e.g., "BTCUSD")
- **price**: Price in smallest unit (e.g., cents for USD)
- **quantity**: Order quantity
- **side**: "Buy" or "Sell"
- **time_in_force**: "GTC" (Good Till Cancelled), "IOC" (Immediate Or Cancel), "FOK" (Fill Or Kill)

#### Return Values
- **add_limit_order**: Returns UUID string of the created order
- **best_bid/best_ask**: Returns price as integer or None if no orders

### Error Handling
- **ValueError**: Invalid parameters (side, time_in_force, zero values)
- **RuntimeError**: Internal order book errors

### Example Usage
```python
from orderbook_rs import PyOrderBook

# Create order book
ob = PyOrderBook("BTCUSD")

# Add orders
buy_order = ob.add_limit_order(50000, 100, "Buy", "GTC")
sell_order = ob.add_limit_order(50100, 100, "Sell", "GTC")

# Check best prices
print(f"Best bid: {ob.best_bid()}")    # 50000
print(f"Best ask: {ob.best_ask()}")    # 50100
```

## Node.js Binding Specification

### Installation
```bash
npm install orderbook-rs
```

### API Reference

#### JsOrderBook Class
```typescript
class JsOrderBook {
    constructor(symbol: string);
    addLimitOrder(price: number, quantity: number, side: string, timeInForce: string): string;
    bestBid(): number | null;
    bestAsk(): number | null;
}
```

#### Parameters
- **symbol**: Trading symbol (e.g., "BTCUSD")
- **price**: Price as number
- **quantity**: Order quantity as number
- **side**: "Buy" or "Sell"
- **timeInForce**: "GTC", "IOC", or "FOK"

#### Return Values
- **addLimitOrder**: Returns UUID string of the created order
- **bestBid/bestAsk**: Returns price as number or null if no orders

### Error Handling
- **Error**: Invalid parameters or internal errors

### Example Usage
```javascript
const { JsOrderBook } = require('orderbook-rs');

// Create order book
const ob = new JsOrderBook("BTCUSD");

// Add orders
const buyOrder = ob.addLimitOrder(50000, 100, "Buy", "GTC");
const sellOrder = ob.addLimitOrder(50100, 100, "Sell", "GTC");

// Check best prices
console.log(`Best bid: ${ob.bestBid()}`);    // 50000
console.log(`Best ask: ${ob.bestAsk()}`);    // 50100
```

## Build System

### Python Build
```bash
# Install maturin
pip install maturin

# Build wheel
maturin build --release --features python

# Install
pip install target/wheels/*.whl
```

### Node.js Build
```bash
# Install dependencies
npm install

# Build native module
npm run build
```

### Universal Build Script
```bash
# Build everything
./scripts/build_bindings.sh all

# Build specific binding
./scripts/build_bindings.sh python
./scripts/build_bindings.sh nodejs
```

## Testing

### Test Coverage
- **Initialization**: Order book creation and initial state
- **Order Management**: Adding limit orders with various parameters
- **Best Price Queries**: Bid/ask price retrieval
- **Error Handling**: Invalid parameters and edge cases
- **Performance**: Bulk operations and timing tests

### Running Tests
```bash
# Python tests
python src/bindings/tests/python_tests.py

# Node.js tests
node src/bindings/tests/node_tests.js

# All tests via build script
./scripts/build_bindings.sh test
```

## Performance Benchmarks

### Benchmark Categories
1. **Single Order Operations**: Add/remove individual orders
2. **Bulk Operations**: Large batch insertions
3. **Query Performance**: Best bid/ask retrieval
4. **Memory Usage**: Resource consumption patterns

### Running Benchmarks
```bash
# Rust benchmarks
cargo bench

# Binding benchmarks
./scripts/build_bindings.sh bench
```

## CI/CD Pipeline

### GitHub Actions
- **Multi-platform builds**: Linux, macOS, Windows
- **Multiple Python versions**: 3.8, 3.9, 3.10, 3.11
- **Multiple Node.js versions**: 16, 18, 20
- **Automated testing**: Unit tests and integration tests
- **Performance monitoring**: Benchmark regression detection

### Quality Gates
- **Code coverage**: > 90%
- **Performance regression**: < 5% degradation
- **Memory leaks**: Zero tolerance
- **API compatibility**: Semantic versioning

## Distribution

### Python Package
- **PyPI**: Automated wheel distribution
- **Supported platforms**: Linux, macOS, Windows
- **Architecture**: x86_64, aarch64

### Node.js Package
- **npm**: Automated native module distribution
- **Supported platforms**: Linux, macOS, Windows
- **Architecture**: x86_64, aarch64

## Security Considerations

### Memory Safety
- **Rust guarantees**: Memory safety without garbage collection
- **Binding safety**: Proper error handling and resource management
- **Thread safety**: Lock-free operations with proper synchronization

### Input Validation
- **Parameter validation**: All inputs validated before processing
- **Error propagation**: Proper error handling across language boundaries
- **Resource limits**: Protection against resource exhaustion

## Future Enhancements

### Planned Features
- **Order modification**: Cancel and replace operations
- **Market orders**: Immediate execution orders
- **Order book depth**: Full depth visibility
- **Time priority**: FIFO order matching
- **Async operations**: Non-blocking API variants

### Performance Optimizations
- **SIMD instructions**: Vectorized operations
- **Memory pooling**: Reduced allocation overhead
- **Batch operations**: Bulk order processing
- **Compression**: Efficient data representation