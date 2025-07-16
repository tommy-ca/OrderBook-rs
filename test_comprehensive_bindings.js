#!/usr/bin/env node

const { JsOrderBook, hello } = require('./index.js');

console.log('=== OrderBook-rs Comprehensive Binding Test ===\n');

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
console.log('   Is crossed:', book.isCrossed());

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
    
    if (marketResult.transactions.length > 0) {
        const tx = marketResult.transactions[0];
        console.log('     First transaction:');
        console.log('       Taker:', tx.takerOrderId);
        console.log('       Maker:', tx.makerOrderId);
        console.log('       Price:', tx.price);
        console.log('       Quantity:', tx.quantity);
        console.log('       Taker side:', tx.takerSide);
        console.log('       Timestamp:', tx.timestamp);
    }
} catch (error) {
    console.error('   Error with market order:', error);
}

// Test order management
console.log('\n8. Testing order management:');
try {
    const testOrderId = book.addLimitOrder(97.0, 12.0, 'buy', 'gtc');
    console.log('   Added test order:', testOrderId);
    
    const orderInfo = book.getOrder(testOrderId);
    console.log('   Order info:', orderInfo);
    
    const updateResult = book.updateOrder(testOrderId, 15.0);
    console.log('   Update result:', updateResult);
    
    const updatedOrderInfo = book.getOrder(testOrderId);
    console.log('   Updated order info:', updatedOrderInfo);
    
    const cancelResult = book.cancelOrder(testOrderId);
    console.log('   Cancel result:', cancelResult);
    
    const canceledOrderInfo = book.getOrder(testOrderId);
    console.log('   Canceled order info:', canceledOrderInfo);
} catch (error) {
    console.error('   Error with order management:', error);
}

// Test snapshot functionality
console.log('\n9. Testing snapshot functionality:');
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

// Test volume analysis
console.log('\n10. Testing volume analysis:');
try {
    const volumeData = book.getVolumeByPrice();
    console.log('   Volume by price:', volumeData.length, 'entries');
    volumeData.slice(0, 5).forEach((entry, i) => {
        console.log('     Entry', i + 1, ':', entry);
    });
    
    console.log('   Total volume:', book.totalVolume());
} catch (error) {
    console.error('   Error with volume analysis:', error);
}

// Test price level queries
console.log('\n11. Testing price level queries:');
try {
    const bidOrders = book.getOrdersAtPrice(100.0, 'buy');
    console.log('   Orders at bid price 100:', bidOrders.length);
    bidOrders.forEach((order, i) => {
        console.log('     Order', i + 1, ':', order);
    });
    
    const askOrders = book.getOrdersAtPrice(101.0, 'sell');
    console.log('   Orders at ask price 101:', askOrders.length);
    askOrders.forEach((order, i) => {
        console.log('     Order', i + 1, ':', order);
    });
} catch (error) {
    console.error('   Error with price level queries:', error);
}

// Test market state
console.log('\n12. Final market state:');
console.log('   Symbol:', book.symbol());
console.log('   Best bid:', book.bestBid());
console.log('   Best ask:', book.bestAsk());
console.log('   Spread:', book.spread());
console.log('   Mid price:', book.midPrice());
console.log('   Last trade price:', book.lastTradePrice());
console.log('   Total orders:', book.totalOrders());
console.log('   Total volume:', book.totalVolume());
console.log('   Is crossed:', book.isCrossed());

// Test utility functions
console.log('\n13. Testing utility functions:');
try {
    book.setMarketCloseTimestamp(Date.now() + 8 * 60 * 60 * 1000); // 8 hours from now
    console.log('   Set market close timestamp');
    
    // Don't clear the book in the test to preserve state
    // book.clear();
    // console.log('   Cleared orderbook');
} catch (error) {
    console.error('   Error with utility functions:', error);
}

console.log('\n=== Test Complete ===');