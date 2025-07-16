#!/usr/bin/env node
/**
 * Comprehensive benchmark tests for Node.js bindings of OrderBook-rs
 * Performance testing and regression detection
 */

const fs = require('fs');
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
 * Container for benchmark results
 */
class BenchmarkResult {
    constructor(name, operations, duration, memoryUsage = 0) {
        this.name = name;
        this.operations = operations;
        this.duration = duration;
        this.opsPerSecond = operations / duration;
        this.memoryUsage = memoryUsage;
        this.timestamp = new Date().toISOString();
    }

    toObject() {
        return {
            name: this.name,
            operations: this.operations,
            duration: this.duration,
            opsPerSecond: this.opsPerSecond,
            memoryUsage: this.memoryUsage,
            timestamp: this.timestamp
        };
    }
}

/**
 * Comprehensive benchmark suite for OrderBook operations
 */
class OrderBookBenchmark {
    constructor() {
        this.results = [];
        this.warmupIterations = 1000;
        this.benchmarkIterations = 10000;
    }

    /**
     * Warm up the V8 engine and caches
     */
    warmup(orderbook) {
        console.log('🔥 Warming up...');
        for (let i = 0; i < this.warmupIterations; i++) {
            const price = 50000 + (i % 100);
            const side = i % 2 === 0 ? 'Buy' : 'Sell';
            orderbook.addLimitOrder(price, 100, side, 'GTC');
        }
    }

    /**
     * Benchmark single order insertion performance
     */
    benchmarkSingleOrderInsertion() {
        console.log('📊 Benchmarking single order insertion...');
        
        const orderbook = new JsOrderBook('BENCHMARK_SINGLE');
        this.warmup(orderbook);
        
        if (global.gc) global.gc();
        
        const startTime = process.hrtime.bigint();
        
        for (let i = 0; i < this.benchmarkIterations; i++) {
            const price = 50000 + (i % 1000);
            const side = i % 2 === 0 ? 'Buy' : 'Sell';
            orderbook.addLimitOrder(price, 100, side, 'GTC');
        }
        
        const endTime = process.hrtime.bigint();
        const duration = Number(endTime - startTime) / 1000000000; // Convert to seconds
        
        const result = new BenchmarkResult('Single Order Insertion', this.benchmarkIterations, duration);
        this.results.push(result);
        return result;
    }

    /**
     * Benchmark batch order insertion performance
     */
    benchmarkBatchOrderInsertion() {
        console.log('📊 Benchmarking batch order insertion...');
        
        const orderbook = new JsOrderBook('BENCHMARK_BATCH');
        const batchSize = 1000;
        const batchCount = 10;
        
        if (global.gc) global.gc();
        
        const startTime = process.hrtime.bigint();
        
        for (let batch = 0; batch < batchCount; batch++) {
            for (let i = 0; i < batchSize; i++) {
                const price = 50000 + (i % 100);
                const side = i % 2 === 0 ? 'Buy' : 'Sell';
                orderbook.addLimitOrder(price, 100, side, 'GTC');
            }
        }
        
        const endTime = process.hrtime.bigint();
        const duration = Number(endTime - startTime) / 1000000000;
        const totalOperations = batchSize * batchCount;
        
        const result = new BenchmarkResult('Batch Order Insertion', totalOperations, duration);
        this.results.push(result);
        return result;
    }

    /**
     * Benchmark price query performance
     */
    benchmarkPriceQueries() {
        console.log('📊 Benchmarking price queries...');
        
        const orderbook = new JsOrderBook('BENCHMARK_QUERIES');
        
        // Add some orders first
        for (let i = 0; i < 1000; i++) {
            const price = 50000 + (i % 100);
            const side = i % 2 === 0 ? 'Buy' : 'Sell';
            orderbook.addLimitOrder(price, 100, side, 'GTC');
        }
        
        const queryCount = 100000;
        
        if (global.gc) global.gc();
        
        const startTime = process.hrtime.bigint();
        
        for (let i = 0; i < queryCount; i++) {
            const bid = orderbook.bestBid();
            const ask = orderbook.bestAsk();
        }
        
        const endTime = process.hrtime.bigint();
        const duration = Number(endTime - startTime) / 1000000000;
        
        const result = new BenchmarkResult('Price Queries', queryCount * 2, duration);
        this.results.push(result);
        return result;
    }

    /**
     * Benchmark mixed operations (realistic trading scenario)
     */
    benchmarkMixedOperations() {
        console.log('📊 Benchmarking mixed operations...');
        
        const orderbook = new JsOrderBook('BENCHMARK_MIXED');
        const operationCount = 5000;
        
        if (global.gc) global.gc();
        
        const startTime = process.hrtime.bigint();
        
        for (let i = 0; i < operationCount; i++) {
            // 70% order insertion, 30% price queries
            if (i % 10 < 7) {
                const price = 50000 + (i % 100);
                const side = i % 2 === 0 ? 'Buy' : 'Sell';
                orderbook.addLimitOrder(price, 100, side, 'GTC');
            } else {
                const bid = orderbook.bestBid();
                const ask = orderbook.bestAsk();
            }
        }
        
        const endTime = process.hrtime.bigint();
        const duration = Number(endTime - startTime) / 1000000000;
        
        const result = new BenchmarkResult('Mixed Operations', operationCount, duration);
        this.results.push(result);
        return result;
    }

    /**
     * Simulate concurrent access patterns
     */
    benchmarkConcurrentSimulation() {
        console.log('📊 Benchmarking concurrent simulation...');
        
        const orderbook = new JsOrderBook('BENCHMARK_CONCURRENT');
        const operationsPerWorker = 1000;
        const workerCount = 4;
        
        if (global.gc) global.gc();
        
        const startTime = process.hrtime.bigint();
        
        // Simulate concurrent access with interleaved operations
        for (let round = 0; round < operationsPerWorker; round++) {
            for (let worker = 0; worker < workerCount; worker++) {
                const price = 50000 + worker * 1000 + (round % 100);
                const side = round % 2 === 0 ? 'Buy' : 'Sell';
                orderbook.addLimitOrder(price, 100, side, 'GTC');
            }
        }
        
        const endTime = process.hrtime.bigint();
        const duration = Number(endTime - startTime) / 1000000000;
        const totalOperations = operationsPerWorker * workerCount;
        
        const result = new BenchmarkResult('Concurrent Simulation', totalOperations, duration);
        this.results.push(result);
        return result;
    }

    /**
     * Benchmark memory usage patterns
     */
    benchmarkMemoryUsage() {
        console.log('📊 Benchmarking memory usage...');
        
        const initialMemory = process.memoryUsage();
        
        const orderbook = new JsOrderBook('BENCHMARK_MEMORY');
        const orderCount = 50000;
        
        if (global.gc) global.gc();
        
        const startTime = process.hrtime.bigint();
        
        for (let i = 0; i < orderCount; i++) {
            const price = 50000 + (i % 1000);
            const side = i % 2 === 0 ? 'Buy' : 'Sell';
            orderbook.addLimitOrder(price, 100, side, 'GTC');
        }
        
        const endTime = process.hrtime.bigint();
        const duration = Number(endTime - startTime) / 1000000000;
        
        if (global.gc) global.gc();
        
        const finalMemory = process.memoryUsage();
        const memoryUsage = (finalMemory.heapUsed - initialMemory.heapUsed) / 1024 / 1024; // MB
        
        const result = new BenchmarkResult('Memory Usage', orderCount, duration, memoryUsage);
        this.results.push(result);
        return result;
    }

    /**
     * Benchmark latency distribution for individual operations
     */
    benchmarkLatencyDistribution() {
        console.log('📊 Benchmarking latency distribution...');
        
        const orderbook = new JsOrderBook('BENCHMARK_LATENCY');
        const operationCount = 10000;
        const latencies = [];
        
        // Warm up
        for (let i = 0; i < 100; i++) {
            orderbook.addLimitOrder(50000 + i, 100, 'Buy', 'GTC');
        }
        
        if (global.gc) global.gc();
        
        for (let i = 0; i < operationCount; i++) {
            const start = process.hrtime.bigint();
            const price = 50000 + (i % 1000);
            const side = i % 2 === 0 ? 'Buy' : 'Sell';
            orderbook.addLimitOrder(price, 100, side, 'GTC');
            const end = process.hrtime.bigint();
            latencies.push(Number(end - start) / 1000); // microseconds
        }
        
        // Calculate statistics
        latencies.sort((a, b) => a - b);
        const meanLatency = latencies.reduce((a, b) => a + b, 0) / latencies.length;
        const medianLatency = latencies[Math.floor(latencies.length / 2)];
        const p95Latency = latencies[Math.floor(0.95 * latencies.length)];
        const p99Latency = latencies[Math.floor(0.99 * latencies.length)];
        
        console.log(`    Mean latency: ${meanLatency.toFixed(2)} μs`);
        console.log(`    Median latency: ${medianLatency.toFixed(2)} μs`);
        console.log(`    95th percentile: ${p95Latency.toFixed(2)} μs`);
        console.log(`    99th percentile: ${p99Latency.toFixed(2)} μs`);
        
        const totalDuration = latencies.reduce((a, b) => a + b, 0) / 1000000; // convert to seconds
        const result = new BenchmarkResult('Latency Distribution', operationCount, totalDuration);
        this.results.push(result);
        return result;
    }

    /**
     * Run all benchmark tests
     */
    runAllBenchmarks() {
        console.log('🚀 Starting OrderBook Node.js Binding Benchmarks');
        console.log('='.repeat(60));
        
        const benchmarks = [
            this.benchmarkSingleOrderInsertion,
            this.benchmarkBatchOrderInsertion,
            this.benchmarkPriceQueries,
            this.benchmarkMixedOperations,
            this.benchmarkConcurrentSimulation,
            this.benchmarkMemoryUsage,
            this.benchmarkLatencyDistribution
        ];
        
        for (const benchmark of benchmarks) {
            try {
                const result = benchmark.call(this);
                console.log(`✅ ${result.name}: ${result.opsPerSecond.toLocaleString()} ops/sec`);
                if (result.memoryUsage !== 0) {
                    console.log(`    Memory usage: ${result.memoryUsage.toFixed(2)} MB`);
                }
            } catch (error) {
                console.log(`❌ ${benchmark.name} failed: ${error.message}`);
            }
        }
        
        this.printSummary();
        this.saveResults();
    }

    /**
     * Print benchmark summary
     */
    printSummary() {
        console.log('\n' + '='.repeat(60));
        console.log('📊 BENCHMARK SUMMARY');
        console.log('='.repeat(60));
        
        for (const result of this.results) {
            const name = result.name.padEnd(30, '.');
            const ops = result.opsPerSecond.toLocaleString().padStart(15);
            console.log(`${name} ${ops} ops/sec`);
        }
        
        if (this.results.length > 0) {
            const totalOps = this.results.reduce((sum, r) => sum + r.operations, 0);
            const totalTime = this.results.reduce((sum, r) => sum + r.duration, 0);
            const averageThroughput = totalOps / totalTime;
            
            console.log('-'.repeat(60));
            const avgName = 'Overall Average'.padEnd(30, '.');
            const avgOps = averageThroughput.toLocaleString().padStart(15);
            console.log(`${avgName} ${avgOps} ops/sec`);
            console.log(`${'Total Operations'.padEnd(30, '.')} ${totalOps.toLocaleString().padStart(15)} ops`);
            console.log(`${'Total Time'.padEnd(30, '.')} ${totalTime.toFixed(3).padStart(15)} sec`);
        }
    }

    /**
     * Save benchmark results to JSON file
     */
    saveResults() {
        const resultsFile = 'benchmark_results_nodejs.json';
        
        const resultsData = {
            timestamp: new Date().toISOString(),
            platform: 'Node.js',
            nodeVersion: process.version,
            results: this.results.map(r => r.toObject())
        };
        
        fs.writeFileSync(resultsFile, JSON.stringify(resultsData, null, 2));
        console.log(`\n📄 Results saved to ${resultsFile}`);
    }

    /**
     * Compare results with baseline performance
     */
    compareWithBaseline(baselineFile = 'benchmark_baseline.json') {
        if (!fs.existsSync(baselineFile)) {
            console.log(`⚠️  Baseline file ${baselineFile} not found`);
            return;
        }

        try {
            const baselineData = JSON.parse(fs.readFileSync(baselineFile, 'utf8'));
            
            console.log(`\n📈 PERFORMANCE COMPARISON vs ${baselineFile}`);
            console.log('='.repeat(60));
            
            const baselineResults = {};
            baselineData.results.forEach(r => {
                baselineResults[r.name] = r;
            });
            
            for (const result of this.results) {
                if (baselineResults[result.name]) {
                    const baselineOps = baselineResults[result.name].opsPerSecond;
                    const currentOps = result.opsPerSecond;
                    
                    if (baselineOps > 0) {
                        const improvement = ((currentOps - baselineOps) / baselineOps) * 100;
                        const status = improvement > 0 ? '📈' : improvement < -5 ? '📉' : '📊';
                        const name = result.name.padEnd(30, '.');
                        const perf = `${improvement.toFixed(1)}%`.padStart(8);
                        console.log(`${name} ${status} ${perf}`);
                    } else {
                        console.log(`${result.name.padEnd(30, '.')} ❓ No baseline data`);
                    }
                } else {
                    console.log(`${result.name.padEnd(30, '.')} 🆕 New benchmark`);
                }
            }
        } catch (error) {
            console.log(`❌ Error comparing with baseline: ${error.message}`);
        }
    }
}

/**
 * Main benchmark execution
 */
function main() {
    try {
        // Enable garbage collection if --expose-gc flag is used
        if (typeof global.gc === 'function') {
            console.log('🗑️  Garbage collection enabled for accurate memory measurements');
        } else {
            console.log('⚠️  Run with --expose-gc flag for accurate memory measurements');
        }
        
        const benchmark = new OrderBookBenchmark();
        benchmark.runAllBenchmarks();
        benchmark.compareWithBaseline();
        
        console.log('\n🎉 Node.js binding benchmarks completed successfully!');
        return 0;
        
    } catch (error) {
        console.log(`❌ Benchmark failed: ${error.message}`);
        return 1;
    }
}

// Run if this file is executed directly
if (require.main === module) {
    process.exit(main());
}

module.exports = { OrderBookBenchmark, BenchmarkResult };