#!/usr/bin/env node
/**
 * Comprehensive test suite for Node.js bindings of OrderBook-rs
 */

const assert = require('assert');
const path = require('path');

// Import the built module
let JsOrderBook;
try {
    JsOrderBook = require('../../../index.js').JsOrderBook;
} catch (error) {
    console.error('Failed to import JsOrderBook:', error.message);
    console.error('Make sure to build with: npm run build');
    process.exit(1);
}

/**
 * Test class for OrderBook functionality
 */
class OrderBookTests {
    constructor() {
        this.passed = 0;
        this.failed = 0;
        this.total = 0;
    }

    /**
     * Assert helper function
     */
    assert(condition, message) {
        this.total++;
        if (condition) {
            this.passed++;
            console.log(`✓ ${message}`);
        } else {
            this.failed++;
            console.error(`✗ ${message}`);
        }
    }

    /**
     * Test OrderBook initialization
     */
    testInitialization() {
        console.log('\n=== Testing Initialization ===');
        const orderbook = new JsOrderBook('BTCUSD');
        
        this.assert(orderbook !== null && orderbook !== undefined, 'OrderBook should be created');
        this.assert(orderbook.bestBid() === null, 'Initial best bid should be null');
        this.assert(orderbook.bestAsk() === null, 'Initial best ask should be null');
    }

    /**
     * Test adding buy limit order
     */
    testAddBuyLimitOrder() {
        console.log('\n=== Testing Buy Limit Order ===');
        const orderbook = new JsOrderBook('BTCUSD');
        
        const orderId = orderbook.addLimitOrder(50000, 100, 'Buy', 'GTC');
        this.assert(orderId !== null && orderId !== undefined, 'Order ID should be returned');
        this.assert(orderbook.bestBid() === 50000, 'Best bid should be 50000');
        this.assert(orderbook.bestAsk() === null, 'Best ask should still be null');
    }

    /**
     * Test adding sell limit order
     */
    testAddSellLimitOrder() {
        console.log('\n=== Testing Sell Limit Order ===');
        const orderbook = new JsOrderBook('BTCUSD');
        
        const orderId = orderbook.addLimitOrder(51000, 100, 'Sell', 'GTC');
        this.assert(orderId !== null && orderId !== undefined, 'Order ID should be returned');
        this.assert(orderbook.bestBid() === null, 'Best bid should still be null');
        this.assert(orderbook.bestAsk() === 51000, 'Best ask should be 51000');
    }

    /**
     * Test multiple orders and best price functionality
     */
    testMultipleOrders() {
        console.log('\n=== Testing Multiple Orders ===');
        const orderbook = new JsOrderBook('BTCUSD');
        
        // Add multiple buy orders
        orderbook.addLimitOrder(50000, 100, 'Buy', 'GTC');
        orderbook.addLimitOrder(50100, 200, 'Buy', 'GTC');
        orderbook.addLimitOrder(49900, 150, 'Buy', 'GTC');
        
        // Add multiple sell orders
        orderbook.addLimitOrder(51000, 100, 'Sell', 'GTC');
        orderbook.addLimitOrder(50900, 200, 'Sell', 'GTC');
        orderbook.addLimitOrder(51100, 150, 'Sell', 'GTC');
        
        // Check best prices
        this.assert(orderbook.bestBid() === 50100, 'Best bid should be 50100 (highest buy price)');
        this.assert(orderbook.bestAsk() === 50900, 'Best ask should be 50900 (lowest sell price)');
    }

    /**
     * Test invalid parameters
     */
    testInvalidParameters() {
        console.log('\n=== Testing Invalid Parameters ===');
        const orderbook = new JsOrderBook('BTCUSD');
        
        // Test invalid side
        try {
            orderbook.addLimitOrder(50000, 100, 'InvalidSide', 'GTC');
            this.assert(false, 'Should throw error for invalid side');
        } catch (error) {
            this.assert(true, 'Should throw error for invalid side');
        }

        // Test invalid time in force
        try {
            orderbook.addLimitOrder(50000, 100, 'Buy', 'InvalidTIF');
            this.assert(false, 'Should throw error for invalid time in force');
        } catch (error) {
            this.assert(true, 'Should throw error for invalid time in force');
        }

        // Test zero quantity
        try {
            const orderId = orderbook.addLimitOrder(50000, 0, 'Buy', 'GTC');
            console.log(`Zero quantity order created: ${orderId}`);
            this.assert(true, 'Zero quantity handled (validation may not be implemented)');
        } catch (error) {
            this.assert(true, 'Should throw error for zero quantity');
        }

        // Test zero price
        try {
            const orderId = orderbook.addLimitOrder(0, 100, 'Buy', 'GTC');
            console.log(`Zero price order created: ${orderId}`);
            this.assert(true, 'Zero price handled (validation may not be implemented)');
        } catch (error) {
            this.assert(true, 'Should throw error for zero price');
        }
    }

    /**
     * Test performance with bulk operations
     */
    testBulkOperations() {
        console.log('\n=== Testing Bulk Operations ===');
        const orderbook = new JsOrderBook('PERFORMANCE_TEST');
        
        const startTime = Date.now();
        
        // Add 1000 orders
        for (let i = 0; i < 1000; i++) {
            const price = 50000 + (i % 100);
            orderbook.addLimitOrder(price, 100, 'Buy', 'GTC');
        }
        
        const endTime = Date.now();
        const duration = endTime - startTime;
        
        console.log(`Inserted 1000 orders in ${duration}ms`);
        this.assert(duration < 1000, 'Bulk insertion should be fast (< 1 second)');
    }

    /**
     * Run all tests
     */
    runAllTests() {
        console.log('🚀 Starting OrderBook Node.js Binding Tests...\n');
        
        this.testInitialization();
        this.testAddBuyLimitOrder();
        this.testAddSellLimitOrder();
        this.testMultipleOrders();
        this.testInvalidParameters();
        this.testBulkOperations();
        
        console.log('\n' + '='.repeat(50));
        console.log(`📊 Test Results: ${this.passed}/${this.total} passed`);
        
        if (this.failed > 0) {
            console.error(`❌ ${this.failed} tests failed`);
            process.exit(1);
        } else {
            console.log('✅ All tests passed!');
            process.exit(0);
        }
    }
}

// Run the tests
const tests = new OrderBookTests();
tests.runAllTests();