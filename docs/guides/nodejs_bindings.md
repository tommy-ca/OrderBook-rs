# Node.js Binding Guide - OrderBook-rs

This guide covers the Node.js bindings for the OrderBook-rs high-performance order book engine.

## 🚨 **Known Compatibility Issue**

**Current Status**: Node.js v22+ has compatibility issues with the NAPI bindings.

**Affected Versions**: Node.js v22.0.0 and above  
**Error**: `Module did not self-register`  
**Working Versions**: Node.js v16.x - v18.x  

## 🔧 **Quick Fix**

### Option 1: Use Compatible Node.js Version

```bash
# Install Node.js 18 (recommended)
nvm install 18
nvm use 18

# Rebuild the bindings
npm run build

# Test the binding
node -e "const { JsOrderBook } = require('./index.js'); console.log('Success!');"
```

### Option 2: Use Python Bindings (Alternative)

If you need immediate functionality, the Python bindings work perfectly:

```bash
# Use Python bindings instead
python3 src/bindings/tests/benchmark_tests.py
```

## 📦 **Installation & Setup**

### Prerequisites

```bash
# Required: Node.js 16-18 (see compatibility issue above)
node --version  # Should show v16.x or v18.x

# Required: Rust toolchain
rustc --version

# Required: Build tools
npm install
```

### Building the Binding

```bash
# Build for development
npm run build

# Build with release optimizations
npm run build --release

# Clean rebuild
rm -f index.node && npm run build
```

### Verification

```bash
# Test basic loading
node test_nodejs_binding.js

# Run comprehensive benchmarks
node benchmark_nodejs.js
```

## 🚀 **Usage Examples**

### Basic Usage

```javascript
const { JsOrderBook } = require('./index.js');

// Create a new order book
const orderbook = new JsOrderBook('BTCUSD');

// Add orders
const buyOrderId = orderbook.addLimitOrder(50000, 100, 'Buy', 'GTC');
const sellOrderId = orderbook.addLimitOrder(50100, 100, 'Sell', 'GTC');

// Query prices
const bestBid = orderbook.bestBid();  // 50000
const bestAsk = orderbook.bestAsk();  // 50100

console.log(`Spread: ${bestAsk - bestBid} (${bestBid} - ${bestAsk})`);
```

### Advanced Usage with Error Handling

```javascript
const { JsOrderBook } = require('./index.js');

class OrderBookManager {
    constructor(symbol) {
        this.orderbook = new JsOrderBook(symbol);
        this.symbol = symbol;
    }
    
    addOrder(price, quantity, side, timeInForce = 'GTC') {
        try {
            const orderId = this.orderbook.addLimitOrder(price, quantity, side, timeInForce);
            console.log(`Order added: ${orderId}`);
            return orderId;
        } catch (error) {
            console.error(`Failed to add order: ${error.message}`);
            throw error;
        }
    }
    
    getSpread() {
        const bid = this.orderbook.bestBid();
        const ask = this.orderbook.bestAsk();
        
        if (bid && ask) {
            return {
                bid,
                ask,
                spread: ask - bid,
                midPrice: (bid + ask) / 2
            };
        }
        
        return null;
    }
    
    getMarketDepth() {
        return {
            bestBid: this.orderbook.bestBid(),
            bestAsk: this.orderbook.bestAsk(),
            symbol: this.symbol
        };
    }
}

// Usage
const manager = new OrderBookManager('AAPL');
manager.addOrder(15000, 100, 'Buy', 'GTC');  // $150.00, 100 shares
manager.addOrder(15010, 50, 'Sell', 'GTC');  // $150.10, 50 shares

const depth = manager.getMarketDepth();
console.log('Market Depth:', depth);
```

### High-Frequency Trading Example

```javascript
const { JsOrderBook } = require('./index.js');

class HFTOrderBook {
    constructor(symbol) {
        this.orderbook = new JsOrderBook(symbol);
        this.orderCount = 0;
    }
    
    // High-frequency order insertion
    bulkAddOrders(orders) {
        const results = [];
        const start = process.hrtime.bigint();
        
        for (const order of orders) {
            try {
                const orderId = this.orderbook.addLimitOrder(
                    order.price,
                    order.quantity,
                    order.side,
                    order.timeInForce || 'GTC'
                );
                results.push({ success: true, orderId });
                this.orderCount++;
            } catch (error) {
                results.push({ success: false, error: error.message });
            }
        }
        
        const end = process.hrtime.bigint();
        const duration = Number(end - start) / 1e6; // milliseconds
        const throughput = orders.length / (duration / 1000);
        
        return {
            results,
            performance: {
                duration: `${duration.toFixed(2)}ms`,
                throughput: `${Math.round(throughput).toLocaleString()} ops/sec`,
                totalOrders: this.orderCount
            }
        };
    }
    
    // Fast price queries
    getQuotes() {
        return {
            bid: this.orderbook.bestBid(),
            ask: this.orderbook.bestAsk(),
            timestamp: Date.now()
        };
    }
}

// Performance test
const hft = new HFTOrderBook('SPY');

// Generate test orders
const orders = [];
for (let i = 0; i < 1000; i++) {
    orders.push({
        price: 40000 + (i % 100),
        quantity: 100,
        side: i % 2 === 0 ? 'Buy' : 'Sell'
    });
}

const result = hft.bulkAddOrders(orders);
console.log('HFT Performance:', result.performance);
```

## 📊 **Performance Benchmarks**

### Expected Performance (Node.js v16-18)

When working correctly with compatible Node.js versions:

| Operation | Performance | Notes |
|-----------|-------------|--------|
| Order Insertion | 40,000-60,000 ops/sec | Limit orders |
| Price Queries | 100,000-150,000 ops/sec | Best bid/ask |
| Mixed Operations | 50,000-80,000 ops/sec | Combined ops |
| Order Book Creation | 10,000+ instances/sec | New instances |

### Running Benchmarks

```bash
# Comprehensive benchmark suite
node benchmark_nodejs.js

# Basic functionality test
node test_nodejs_binding.js

# Performance comparison with Python
node benchmark_nodejs.js  # Then check output
```

### Sample Benchmark Output

```
🚀 Node.js OrderBook Benchmark Suite
====================================
✅ Order Book Creation: 45,234 ops/sec
✅ Order Insertion: 52,143 ops/sec
✅ Price Queries: 128,376 ops/sec
✅ Mixed Operations: 61,892 ops/sec
--------------------------------------------
Overall Average: 71,911 ops/sec
```

## 🔍 **Troubleshooting**

### Common Issues

#### 1. "Module did not self-register" Error

**Cause**: Node.js v22+ compatibility issue  
**Solution**: Use Node.js v16-18

```bash
nvm install 18
nvm use 18
npm run build
```

#### 2. Build Errors

**Cause**: Missing dependencies or incompatible toolchain  
**Solution**: Check prerequisites

```bash
# Check Node.js version
node --version  # Should be v16-18

# Check Rust installation
rustc --version

# Reinstall dependencies
rm -rf node_modules package-lock.json
npm install
npm run build
```

#### 3. Import Errors

**Cause**: Incorrect import syntax  
**Solution**: Use correct import pattern

```javascript
// ✅ Correct
const { JsOrderBook } = require('./index.js');

// ❌ Incorrect
const OrderBook = require('./index.js');  // Missing destructuring
import { JsOrderBook } from './index.js';  // ES6 imports not supported
```

#### 4. Performance Issues

**Cause**: Debug build or wrong Node.js version  
**Solution**: Use release build

```bash
# Build in release mode
npm run build --release

# Verify Node.js version
node --version  # v16-18 for best performance
```

### Diagnostic Commands

```bash
# Check binding file
file index.node

# Check dependencies
ldd index.node

# Verify exports
node -e "console.log(Object.keys(require('./index.node')))"

# Test with multiple Node.js versions
nvm use 16 && node test_nodejs_binding.js
nvm use 18 && node test_nodejs_binding.js
nvm use 22 && node test_nodejs_binding.js  # Will fail
```

## 🔄 **Version Compatibility Matrix**

| Node.js Version | Status | Performance | Notes |
|----------------|--------|-------------|--------|
| v16.x | ✅ Working | Excellent | Recommended |
| v17.x | ✅ Working | Excellent | - |
| v18.x | ✅ Working | Excellent | Recommended |
| v19.x | ⚠️ Untested | Unknown | - |
| v20.x | ⚠️ Untested | Unknown | - |
| v21.x | ⚠️ Untested | Unknown | - |
| v22.x+ | ❌ Broken | N/A | NAPI compatibility issue |

## 🛠 **Development Workflow**

### Making Changes

```bash
# 1. Edit Rust source
vim src/bindings/node.rs

# 2. Rebuild binding
npm run build

# 3. Test changes
node test_nodejs_binding.js

# 4. Run benchmarks
node benchmark_nodejs.js

# 5. Run linting
npm run lint  # If configured
```

### Testing with Different Node.js Versions

```bash
# Test compatibility across versions
for version in 16 18 22; do
    echo "Testing Node.js v$version"
    nvm use $version 2>/dev/null && npm run build && node test_nodejs_binding.js
done
```

## 📚 **API Reference**

### JsOrderBook Class

#### Constructor
```typescript
new JsOrderBook(symbol: string)
```
Creates a new order book for the specified trading symbol.

#### Methods

##### addLimitOrder
```typescript
addLimitOrder(price: number, quantity: number, side: string, timeInForce: string): string
```
- **price**: Price in smallest unit (e.g., cents)
- **quantity**: Order quantity
- **side**: "Buy" or "Sell"
- **timeInForce**: "GTC", "IOC", or "FOK"
- **Returns**: Order ID (UUID string)

##### bestBid
```typescript
bestBid(): number | null
```
Returns the best (highest) bid price, or null if no bids exist.

##### bestAsk
```typescript
bestAsk(): number | null
```
Returns the best (lowest) ask price, or null if no asks exist.

### Types

```typescript
type OrderSide = "Buy" | "Sell";
type TimeInForce = "GTC" | "IOC" | "FOK";

interface OrderDetails {
    id: string;
    price: number;
    quantity: number;
    side: OrderSide;
    timeInForce: TimeInForce;
}

interface PriceLevel {
    price: number;
    quantity: number;
    orderCount: number;
}
```

## 🔗 **Related Documentation**

- [Python Binding Guide](./src/bindings/tests/python_tests.py)
- [Development Workflows](./DEVELOPMENT_WORKFLOWS.md)
- [Performance Benchmarks](./benchmark_nodejs.js)
- [Main README](./README.md)

## 💡 **Performance Tips**

1. **Use Node.js v16-18** for best compatibility and performance
2. **Build in release mode** for production use
3. **Batch operations** when possible for better throughput
4. **Reuse OrderBook instances** rather than creating new ones
5. **Profile your usage** with the provided benchmark tools

## 🤝 **Contributing**

If you encounter issues or have improvements:

1. Check compatibility matrix above
2. Run diagnostic commands
3. Test with multiple Node.js versions
4. Submit issues with full environment details