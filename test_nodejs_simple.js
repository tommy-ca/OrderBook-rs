#!/usr/bin/env node

const { JsOrderBook, hello } = require('./index.js');

console.log('=== OrderBook-rs Basic Node.js Binding Test ===\n');

// Test hello function
console.log('1. Testing hello function:', hello());

// Create orderbook
const book = new JsOrderBook('BTC/USD');
console.log('2. Created orderbook for symbol:', book.symbol());

// Test basic functionality
console.log('\n3. Testing basic functionality:');
console.log('   Best bid:', book.bestBid());
console.log('   Best ask:', book.bestAsk());
console.log('   Spread:', book.spread());
console.log('   Mid price:', book.midPrice());
console.log('   Total orders:', book.totalOrders());

// Test adding limit orders
console.log('\n4. Testing limit orders:');
try {
    const buyOrderId = book.addLimitOrder(100.0, 10.0, 'buy', 'gtc');
    console.log('   Added buy order:', buyOrderId);
    
    const sellOrderId = book.addLimitOrder(101.0, 10.0, 'sell', 'gtc');
    console.log('   Added sell order:', sellOrderId);
    
    console.log('   Best bid after orders:', book.bestBid());
    console.log('   Best ask after orders:', book.bestAsk());
    console.log('   Spread after orders:', book.spread());
    console.log('   Mid price after orders:', book.midPrice());
    console.log('   Total orders after orders:', book.totalOrders());
} catch (error) {
    console.error('   Error adding limit orders:', error);
}

// Test iceberg orders
console.log('\n5. Testing iceberg orders:');
try {
    const icebergOrderId = book.addIcebergOrder(99.0, 5.0, 15.0, 'buy', 'gtc');
    console.log('   Added iceberg order:', icebergOrderId);
    console.log('   Total orders after iceberg:', book.totalOrders());
} catch (error) {
    console.error('   Error adding iceberg order:', error);
}

// Test post-only orders
console.log('\n6. Testing post-only orders:');
try {
    const postOnlyOrderId = book.addPostOnlyOrder(98.0, 8.0, 'buy', 'gtc');
    console.log('   Added post-only order:', postOnlyOrderId);
    console.log('   Total orders after post-only:', book.totalOrders());
} catch (error) {
    console.error('   Error adding post-only order:', error);
}

// Test market orders
console.log('\n7. Testing market orders:');
try {
    const marketResult = book.submitMarketOrder(5.0, 'buy');
    console.log('   Market order result:');
    console.log('     Order ID:', marketResult.orderId);
    console.log('     Executed quantity:', marketResult.executedQuantity);
    console.log('     Remaining quantity:', marketResult.remainingQuantity);
    console.log('     Is complete:', marketResult.isComplete);
    console.log('     Transactions:', marketResult.transactions.length);
    console.log('     Filled order IDs:', marketResult.filledOrderIds);
    console.log('     Executed value:', marketResult.executedValue);
    console.log('     Average price:', marketResult.averagePrice);
} catch (error) {
    console.error('   Error with market order:', error);
}

// Test snapshot functionality
console.log('\n8. Testing snapshot functionality:');
try {
    const snapshot = book.createSnapshot(3);
    console.log('   Snapshot:');
    console.log('     Symbol:', snapshot.symbol);
    console.log('     Timestamp:', snapshot.timestamp);
    console.log('     Bids:', snapshot.bids.length, 'levels');
    console.log('     Asks:', snapshot.asks.length, 'levels');
    
    if (snapshot.bids.length > 0) {
        const topBid = snapshot.bids[0];
        console.log('     Top bid: price=', topBid.price, 'visible=', topBid.visibleQuantity, 'hidden=', topBid.hiddenQuantity, 'orders=', topBid.orderCount);
    }
    
    if (snapshot.asks.length > 0) {
        const topAsk = snapshot.asks[0];
        console.log('     Top ask: price=', topAsk.price, 'visible=', topAsk.visibleQuantity, 'hidden=', topAsk.hiddenQuantity, 'orders=', topAsk.orderCount);
    }
} catch (error) {
    console.error('   Error creating snapshot:', error);
}

console.log('\n=== Test Complete ===');