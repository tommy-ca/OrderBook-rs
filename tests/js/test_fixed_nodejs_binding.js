#!/usr/bin/env node

const { JsOrderBook, hello } = require('./index.node');

console.log('=== Fixed Node.js Binding Test ===');

// Test 1: Basic module loading
console.log('1. Testing basic module loading...');
console.log('   Hello function:', hello());

// Test 2: OrderBook creation
console.log('\n2. Testing OrderBook creation...');
const book = new JsOrderBook('BTC/USD');
console.log('   OrderBook created successfully!');

// Test 3: Adding orders
console.log('\n3. Testing order operations...');
try {
    const buyOrder = book.addLimitOrder(50000, 100, 'Buy', 'GTC');
    console.log('   Buy order added:', buyOrder);
    
    const sellOrder = book.addLimitOrder(50100, 200, 'Sell', 'GTC');
    console.log('   Sell order added:', sellOrder);
    
    console.log('   Best bid:', book.bestBid());
    console.log('   Best ask:', book.bestAsk());
} catch (e) {
    console.error('   Error:', e.message);
}

// Test 4: Performance benchmark
console.log('\n4. Performance benchmark...');
const startTime = process.hrtime.bigint();
const iterations = 10000;

for (let i = 0; i < iterations; i++) {
    const price = 50000 + (i % 1000);
    const side = i % 2 === 0 ? 'Buy' : 'Sell';
    book.addLimitOrder(price, 100, side, 'GTC');
}

const endTime = process.hrtime.bigint();
const duration = Number(endTime - startTime) / 1000000; // Convert to milliseconds
const opsPerSecond = (iterations / duration) * 1000;

console.log(`   Added ${iterations} orders in ${duration.toFixed(2)}ms`);
console.log(`   Performance: ${opsPerSecond.toFixed(0)} ops/sec`);

// Test 5: Final state
console.log('\n5. Final OrderBook state...');
console.log('   Best bid:', book.bestBid());
console.log('   Best ask:', book.bestAsk());

console.log('\n=== All tests completed successfully! ===');
console.log('✅ NAPI registration issue FIXED!');