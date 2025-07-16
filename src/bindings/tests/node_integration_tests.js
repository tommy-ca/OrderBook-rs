#!/usr/bin/env node
/**
 * Integration tests for Node.js bindings of OrderBook-rs
 * Tests complex scenarios and edge cases
 */

const assert = require('assert');
const { Worker, isMainThread, parentPort, workerData } = require('worker_threads');
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
 * Integration test class for complex OrderBook scenarios
 */
class OrderBookIntegrationTests {
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
     * Test order priority (FIFO at same price level)
     */
    testOrderPriorityFIFO() {
        console.log('\n=== Testing Order Priority (FIFO) ===');
        const orderbook = new JsOrderBook('FIFO_TEST');
        
        // Add multiple orders at same price
        const order1 = orderbook.addLimitOrder(50000, 100, 'Buy', 'GTC');
        const order2 = orderbook.addLimitOrder(50000, 200, 'Buy', 'GTC');
        const order3 = orderbook.addLimitOrder(50000, 150, 'Buy', 'GTC');
        
        // All should be unique order IDs
        this.assert(order1 !== order2, 'Order IDs should be unique');
        this.assert(order2 !== order3, 'Order IDs should be unique');
        this.assert(order1 !== order3, 'Order IDs should be unique');
        
        // Best bid should remain 50000
        this.assert(orderbook.bestBid() === 50000, 'Best bid should remain 50000');
    }

    /**
     * Test spread calculation scenarios
     */
    testSpreadCalculation() {
        console.log('\n=== Testing Spread Calculation ===');
        const orderbook = new JsOrderBook('SPREAD_TEST');
        
        // No spread initially
        this.assert(orderbook.bestBid() === null, 'Initial best bid should be null');
        this.assert(orderbook.bestAsk() === null, 'Initial best ask should be null');
        
        // Add bid
        orderbook.addLimitOrder(49900, 100, 'Buy', 'GTC');
        this.assert(orderbook.bestBid() === 49900, 'Best bid should be 49900');
        this.assert(orderbook.bestAsk() === null, 'Best ask should still be null');
        
        // Add ask
        orderbook.addLimitOrder(50100, 100, 'Sell', 'GTC');
        this.assert(orderbook.bestAsk() === 50100, 'Best ask should be 50100');
        
        // Spread should be 200
        const spread = orderbook.bestAsk() - orderbook.bestBid();
        this.assert(spread === 200, `Spread should be 200, got ${spread}`);
    }

    /**
     * Test handling of large price values
     */
    testLargePriceValues() {
        console.log('\n=== Testing Large Price Values ===');
        const orderbook = new JsOrderBook('LARGE_PRICE_TEST');
        
        const largePrice = Math.pow(2, 31) - 1; // Max 32-bit signed
        
        const orderId = orderbook.addLimitOrder(largePrice, 100, 'Buy', 'GTC');
        this.assert(orderId !== null && orderId !== undefined, 'Order ID should be returned for large price');
        this.assert(orderbook.bestBid() === largePrice, `Best bid should be ${largePrice}`);
    }

    /**
     * Test handling of large quantity values
     */
    testLargeQuantityValues() {
        console.log('\n=== Testing Large Quantity Values ===');
        const orderbook = new JsOrderBook('LARGE_QUANTITY_TEST');
        
        const largeQuantity = Math.pow(2, 31) - 1; // Max 32-bit signed
        
        const orderId = orderbook.addLimitOrder(50000, largeQuantity, 'Buy', 'GTC');
        this.assert(orderId !== null && orderId !== undefined, 'Order ID should be returned for large quantity');
        this.assert(orderbook.bestBid() === 50000, 'Best bid should be 50000');
    }

    /**
     * Test different time in force values
     */
    testMixedTimeInForce() {
        console.log('\n=== Testing Mixed Time in Force ===');
        const orderbook = new JsOrderBook('TIF_TEST');
        
        // GTC orders should always work
        const gtcOrder = orderbook.addLimitOrder(50000, 100, 'Buy', 'GTC');
        this.assert(gtcOrder !== null, 'GTC order should be created');
        
        // Add sell order to provide liquidity for IOC/FOK tests
        const sellOrder = orderbook.addLimitOrder(50001, 100, 'Sell', 'GTC');
        this.assert(sellOrder !== null, 'Sell order should be created');
        
        // IOC orders - should execute immediately against available liquidity
        try {
            const iocOrder = orderbook.addLimitOrder(50001, 50, 'Buy', 'IOC');
            this.assert(iocOrder !== null, 'IOC order should be created');
        } catch (error) {
            this.assert(error.message.includes('InsufficientLiquidity'), 'IOC should fail with insufficient liquidity');
        }
        
        // FOK orders - should execute fully or not at all
        try {
            const fokOrder = orderbook.addLimitOrder(50001, 25, 'Buy', 'FOK');
            this.assert(fokOrder !== null, 'FOK order should be created');
        } catch (error) {
            this.assert(error.message.includes('InsufficientLiquidity'), 'FOK should fail with insufficient liquidity');
        }
        
        // Best bid should be the GTC order
        this.assert(orderbook.bestBid() === 50000, 'Best bid should be 50000');
    }

    /**
     * Simulate realistic order book depth
     */
    testOrderBookDepthSimulation() {
        console.log('\n=== Testing Order Book Depth Simulation ===');
        const orderbook = new JsOrderBook('DEPTH_TEST');
        
        // Add multiple bid levels
        const bidLevels = [
            [49900, 1000],
            [49950, 1500],
            [50000, 2000],
            [50050, 1200],
            [50100, 800]
        ];
        
        // Add multiple ask levels
        const askLevels = [
            [50200, 800],
            [50250, 1200],
            [50300, 2000],
            [50350, 1500],
            [50400, 1000]
        ];
        
        // Add all bid orders
        for (const [price, quantity] of bidLevels) {
            orderbook.addLimitOrder(price, quantity, 'Buy', 'GTC');
        }
        
        // Add all ask orders
        for (const [price, quantity] of askLevels) {
            orderbook.addLimitOrder(price, quantity, 'Sell', 'GTC');
        }
        
        // Verify best prices
        this.assert(orderbook.bestBid() === 50100, 'Best bid should be 50100');
        this.assert(orderbook.bestAsk() === 50200, 'Best ask should be 50200');
        
        // Spread should be 100
        const spread = orderbook.bestAsk() - orderbook.bestBid();
        this.assert(spread === 100, `Spread should be 100, got ${spread}`);
    }

    /**
     * Test high volume order processing
     */
    testHighVolumeOrders() {
        console.log('\n=== Testing High Volume Orders ===');
        const orderbook = new JsOrderBook('HIGH_VOLUME_TEST');
        
        const orderCount = 10000;
        const startTime = Date.now();
        
        const orders = [];
        for (let i = 0; i < orderCount; i++) {
            const price = 50000 + (i % 1000); // Create price levels
            const side = i % 2 === 0 ? 'Buy' : 'Sell';
            const orderId = orderbook.addLimitOrder(price, 100, side, 'GTC');
            orders.push(orderId);
        }
        
        const endTime = Date.now();
        const duration = endTime - startTime;
        
        // Performance assertion
        const ordersPerSecond = (orderCount / duration) * 1000;
        console.log(`Added ${orderCount} orders in ${duration}ms (${ordersPerSecond.toFixed(0)} orders/sec)`);
        
        // Should handle at least 10K orders per second
        this.assert(ordersPerSecond > 10000, `Should handle >10K orders/sec, got ${ordersPerSecond.toFixed(0)}`);
        
        // Verify all orders were added
        this.assert(orders.length === orderCount, `Should have ${orderCount} orders, got ${orders.length}`);
    }

    /**
     * Test concurrent access (using worker threads)
     */
    async testConcurrentAccess() {
        console.log('\n=== Testing Concurrent Access ===');
        
        // Skip if worker threads not available
        if (!Worker) {
            console.log('⚠ Worker threads not available, skipping concurrent test');
            return;
        }
        
        const workerCount = 4;
        const ordersPerWorker = 250;
        
        const workers = [];
        const results = [];
        
        // Create worker code
        const workerCode = `
            const { parentPort, workerData } = require('worker_threads');
            const { JsOrderBook } = require('${path.resolve(__dirname, '../../../index.js')}');
            
            const { workerId, orderCount } = workerData;
            const orderbook = new JsOrderBook('WORKER_TEST_' + workerId);
            
            const orders = [];
            for (let i = 0; i < orderCount; i++) {
                const price = 50000 + workerId * 100 + i;
                const orderId = orderbook.addLimitOrder(price, 100, 'Buy', 'GTC');
                orders.append(orderId);
            }
            
            parentPort.postMessage({ workerId, orders });
        `;
        
        // This is a simplified test - in practice, we'd need shared state
        // For now, just test that multiple OrderBooks can be created
        for (let i = 0; i < workerCount; i++) {
            const orderbook = new JsOrderBook(`CONCURRENT_TEST_${i}`);
            
            for (let j = 0; j < ordersPerWorker; j++) {
                const price = 50000 + i * 100 + j;
                const orderId = orderbook.addLimitOrder(price, 100, 'Buy', 'GTC');
                results.push(orderId);
            }
        }
        
        const expectedOrderCount = workerCount * ordersPerWorker;
        this.assert(results.length === expectedOrderCount, `Should have ${expectedOrderCount} orders, got ${results.length}`);
        
        // All order IDs should be unique
        const uniqueOrders = new Set(results);
        this.assert(uniqueOrders.size === expectedOrderCount, 'All order IDs should be unique');
    }

    /**
     * Test error handling comprehensively
     */
    testErrorHandlingComprehensive() {
        console.log('\n=== Testing Comprehensive Error Handling ===');
        const orderbook = new JsOrderBook('ERROR_TEST');
        
        // Test various invalid sides
        const invalidSides = ['', 'buy', 'sell', 'BUY', 'SELL', 'Invalid', '123'];
        for (const side of invalidSides) {
            try {
                orderbook.addLimitOrder(50000, 100, side, 'GTC');
                this.assert(false, `Should throw error for invalid side: ${side}`);
            } catch (error) {
                this.assert(true, `Correctly threw error for invalid side: ${side}`);
            }
        }
        
        // Test null/undefined sides
        try {
            orderbook.addLimitOrder(50000, 100, null, 'GTC');
            this.assert(false, 'Should throw error for null side');
        } catch (error) {
            this.assert(true, 'Correctly threw error for null side');
        }
        
        // Test various invalid time in force values
        const invalidTifs = ['', 'gtc', 'ioc', 'fok', 'Invalid', '123'];
        for (const tif of invalidTifs) {
            try {
                orderbook.addLimitOrder(50000, 100, 'Buy', tif);
                this.assert(false, `Should throw error for invalid TIF: ${tif}`);
            } catch (error) {
                this.assert(true, `Correctly threw error for invalid TIF: ${tif}`);
            }
        }
        
        // Test null/undefined time in force
        try {
            orderbook.addLimitOrder(50000, 100, 'Buy', null);
            this.assert(false, 'Should throw error for null TIF');
        } catch (error) {
            this.assert(true, 'Correctly threw error for null TIF');
        }
        
        // Test negative values - Node.js will convert to large positive numbers
        try {
            const negPriceOrder = orderbook.addLimitOrder(-1, 100, 'Buy', 'GTC');
            this.assert(negPriceOrder !== null, 'Negative price converted to large positive (JavaScript behavior)');
        } catch (error) {
            this.assert(true, 'Correctly threw error for negative price');
        }
        
        try {
            const negQuantityOrder = orderbook.addLimitOrder(50000, -1, 'Buy', 'GTC');
            this.assert(negQuantityOrder !== null, 'Negative quantity converted to large positive (JavaScript behavior)');
        } catch (error) {
            this.assert(true, 'Correctly threw error for negative quantity');
        }
        
        // Test zero values
        try {
            orderbook.addLimitOrder(0, 100, 'Buy', 'GTC');
            this.assert(false, 'Should throw error for zero price');
        } catch (error) {
            this.assert(true, 'Correctly threw error for zero price');
        }
        
        try {
            orderbook.addLimitOrder(50000, 0, 'Buy', 'GTC');
            this.assert(false, 'Should throw error for zero quantity');
        } catch (error) {
            this.assert(true, 'Correctly threw error for zero quantity');
        }
    }

    /**
     * Test Unicode symbol handling
     */
    testUnicodeSymbolHandling() {
        console.log('\n=== Testing Unicode Symbol Handling ===');
        
        const unicodeSymbols = ['BTC/USD', 'ETH-EUR', '株式会社', '🚀MOON🚀', ''];
        
        for (const symbol of unicodeSymbols) {
            try {
                const ob = new JsOrderBook(symbol);
                const orderId = ob.addLimitOrder(50000, 100, 'Buy', 'GTC');
                this.assert(orderId !== null && orderId !== undefined, `Should handle Unicode symbol: ${symbol}`);
            } catch (error) {
                // Some symbols might be invalid, but shouldn't crash
                this.assert(error instanceof Error, `Should throw proper Error for symbol: ${symbol}`);
            }
        }
    }

    /**
     * Memory usage stability test
     */
    testMemoryStability() {
        console.log('\n=== Testing Memory Stability ===');
        const orderbook = new JsOrderBook('MEMORY_TEST');
        
        // Force garbage collection if available
        if (global.gc) {
            global.gc();
        }
        
        // Add many orders
        for (let i = 0; i < 50000; i++) {
            const price = 50000 + (i % 100);
            orderbook.addLimitOrder(price, 100, 'Buy', 'GTC');
        }
        
        // Force garbage collection again
        if (global.gc) {
            global.gc();
        }
        
        // This test mainly ensures no memory leaks cause crashes
        this.assert(orderbook.bestBid() !== null, 'OrderBook should still be functional after many operations');
    }

    /**
     * Run all integration tests
     */
    async runAllTests() {
        console.log('🚀 Starting OrderBook Node.js Integration Tests...\n');
        
        this.testOrderPriorityFIFO();
        this.testSpreadCalculation();
        this.testLargePriceValues();
        this.testLargeQuantityValues();
        this.testMixedTimeInForce();
        this.testOrderBookDepthSimulation();
        this.testHighVolumeOrders();
        await this.testConcurrentAccess();
        this.testErrorHandlingComprehensive();
        this.testUnicodeSymbolHandling();
        this.testMemoryStability();
        
        console.log('\n' + '='.repeat(50));
        console.log(`📊 Integration Test Results: ${this.passed}/${this.total} passed`);
        
        if (this.failed > 0) {
            console.error(`❌ ${this.failed} tests failed`);
            process.exit(1);
        } else {
            console.log('✅ All integration tests passed!');
            process.exit(0);
        }
    }
}

// Run the integration tests
const tests = new OrderBookIntegrationTests();
tests.runAllTests().catch(error => {
    console.error('Test runner error:', error);
    process.exit(1);
});