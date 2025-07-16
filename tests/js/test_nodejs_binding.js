#!/usr/bin/env node
/**
 * Node.js Binding Test for OrderBook-rs
 * Tests basic functionality and performance
 */

console.log('🚀 Starting Node.js OrderBook Binding Tests');
console.log('==============================================');

// Test 1: Module Loading
console.log('\n📦 Test 1: Module Loading');
try {
    const binding = require('./index.node');
    console.log('✅ Raw binding loaded successfully');
    console.log('📋 Available exports:', Object.keys(binding));
} catch (error) {
    console.log('❌ Raw binding failed:', error.message);
    
    // Try alternative loading approaches
    console.log('\n🔄 Trying alternative loading methods...');
    
    try {
        // Method 1: Using require with full path
        const path = require('path');
        const fullPath = path.resolve(__dirname, 'index.node');
        console.log('📍 Full path:', fullPath);
        const binding2 = require(fullPath);
        console.log('✅ Alternative loading successful');
        console.log('📋 Available exports:', Object.keys(binding2));
    } catch (error2) {
        console.log('❌ Alternative loading failed:', error2.message);
        
        // Method 2: Check if it's a Node.js version issue
        console.log('\n🔍 Diagnostics:');
        console.log('  Node.js version:', process.version);
        console.log('  Platform:', process.platform);
        console.log('  Architecture:', process.arch);
        
        // Method 3: Try loading through index.js
        try {
            const indexBinding = require('./index.js');
            console.log('✅ Index.js loading successful');
            console.log('📋 Available exports:', Object.keys(indexBinding));
        } catch (error3) {
            console.log('❌ Index.js loading failed:', error3.message);
            console.log('\n💡 Recommendation: Use Node.js v16-18 for better compatibility');
            process.exit(1);
        }
    }
}

// Test 2: If loading was successful, test functionality
console.log('\n🧪 Test 2: Basic Functionality');
try {
    const { JsOrderBook } = require('./index.js');
    
    // Create OrderBook instance
    const orderbook = new JsOrderBook('BTCUSD');
    console.log('✅ OrderBook created successfully');
    
    // Test initial state
    const initialBid = orderbook.bestBid();
    const initialAsk = orderbook.bestAsk();
    console.log('✅ Initial state - Bid:', initialBid, 'Ask:', initialAsk);
    
    // Test adding orders
    const orderId1 = orderbook.addLimitOrder(50000, 100, 'Buy', 'GTC');
    const orderId2 = orderbook.addLimitOrder(50100, 100, 'Sell', 'GTC');
    console.log('✅ Orders added - Buy:', orderId1, 'Sell:', orderId2);
    
    // Test price queries
    const bestBid = orderbook.bestBid();
    const bestAsk = orderbook.bestAsk();
    console.log('✅ Price queries - Bid:', bestBid, 'Ask:', bestAsk);
    
    console.log('\n🎉 All basic functionality tests passed!');
    return true;
    
} catch (error) {
    console.log('❌ Functionality test failed:', error.message);
    return false;
}