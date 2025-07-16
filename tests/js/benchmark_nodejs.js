#!/usr/bin/env node
/**
 * Node.js OrderBook Benchmark Suite
 * 
 * This benchmark attempts to test the Node.js bindings performance,
 * but includes workarounds for Node.js v22 compatibility issues.
 */

console.log('🚀 Node.js OrderBook Benchmark Suite');
console.log('====================================');

// Helper function to format numbers
function formatNumber(num) {
    return num.toLocaleString();
}

// Benchmark configuration
const BENCHMARK_CONFIG = {
    warmupRuns: 100,
    benchmarkRuns: 10000,
    batchSize: 1000,
    symbol: 'BTCUSD'
};

// Test different Node.js versions for compatibility
function checkCompatibility() {
    console.log('\n🔍 Compatibility Check');
    console.log('Node.js version:', process.version);
    console.log('Platform:', process.platform, process.arch);
    
    // Check if Node.js version is compatible
    const nodeVersion = process.version;
    const majorVersion = parseInt(nodeVersion.split('.')[0].substring(1));
    
    if (majorVersion >= 22) {
        console.log('⚠️  Node.js v22+ detected - known compatibility issues');
        console.log('💡 Recommendation: Use Node.js v16-18 for best compatibility');
        return false;
    } else if (majorVersion >= 16) {
        console.log('✅ Node.js version compatible');
        return true;
    } else {
        console.log('❌ Node.js version too old (< v16)');
        return false;
    }
}

// Attempt to load the binding with multiple strategies
function loadBinding() {
    console.log('\n📦 Loading Node.js Binding');
    
    const strategies = [
        {
            name: 'Direct require',
            loader: () => require('./index.js')
        },
        {
            name: 'Raw binding',
            loader: () => require('./index.node')
        },
        {
            name: 'Path resolve',
            loader: () => {
                const path = require('path');
                return require(path.resolve(__dirname, 'index.node'));
            }
        }
    ];
    
    for (const strategy of strategies) {
        try {
            console.log(`  Trying ${strategy.name}...`);
            const binding = strategy.loader();
            console.log(`  ✅ ${strategy.name} successful`);
            console.log('  📋 Available exports:', Object.keys(binding));
            return binding;
        } catch (error) {
            console.log(`  ❌ ${strategy.name} failed: ${error.message}`);
        }
    }
    
    return null;
}

// Run benchmarks if binding loads successfully
function runBenchmarks(binding) {
    console.log('\n🏁 Running Benchmarks');
    
    const { JsOrderBook } = binding;
    
    // Benchmark 1: Order Book Creation
    console.log('\n📊 Benchmark 1: Order Book Creation');
    const creationStart = process.hrtime.bigint();
    
    for (let i = 0; i < BENCHMARK_CONFIG.benchmarkRuns; i++) {
        const orderbook = new JsOrderBook(`${BENCHMARK_CONFIG.symbol}_${i}`);
    }
    
    const creationEnd = process.hrtime.bigint();
    const creationTime = Number(creationEnd - creationStart) / 1e6; // Convert to milliseconds
    const creationOpsPerSec = Math.round(BENCHMARK_CONFIG.benchmarkRuns / (creationTime / 1000));
    
    console.log(`✅ Order Book Creation: ${formatNumber(creationOpsPerSec)} ops/sec`);
    
    // Benchmark 2: Order Insertion
    console.log('\n📊 Benchmark 2: Order Insertion');
    const orderbook = new JsOrderBook(BENCHMARK_CONFIG.symbol);
    
    // Warmup
    for (let i = 0; i < BENCHMARK_CONFIG.warmupRuns; i++) {
        orderbook.addLimitOrder(50000 + i, 100, 'Buy', 'GTC');
    }
    
    const insertionStart = process.hrtime.bigint();
    
    for (let i = 0; i < BENCHMARK_CONFIG.benchmarkRuns; i++) {
        const price = 50000 + (i % 1000);
        const side = i % 2 === 0 ? 'Buy' : 'Sell';
        orderbook.addLimitOrder(price, 100, side, 'GTC');
    }
    
    const insertionEnd = process.hrtime.bigint();
    const insertionTime = Number(insertionEnd - insertionStart) / 1e6;
    const insertionOpsPerSec = Math.round(BENCHMARK_CONFIG.benchmarkRuns / (insertionTime / 1000));
    
    console.log(`✅ Order Insertion: ${formatNumber(insertionOpsPerSec)} ops/sec`);
    
    // Benchmark 3: Price Queries
    console.log('\n📊 Benchmark 3: Price Queries');
    const queryStart = process.hrtime.bigint();
    
    for (let i = 0; i < BENCHMARK_CONFIG.benchmarkRuns; i++) {
        orderbook.bestBid();
        orderbook.bestAsk();
    }
    
    const queryEnd = process.hrtime.bigint();
    const queryTime = Number(queryEnd - queryStart) / 1e6;
    const queryOpsPerSec = Math.round((BENCHMARK_CONFIG.benchmarkRuns * 2) / (queryTime / 1000));
    
    console.log(`✅ Price Queries: ${formatNumber(queryOpsPerSec)} ops/sec`);
    
    // Benchmark 4: Mixed Operations
    console.log('\n📊 Benchmark 4: Mixed Operations');
    const mixedStart = process.hrtime.bigint();
    
    for (let i = 0; i < BENCHMARK_CONFIG.benchmarkRuns; i++) {
        if (i % 3 === 0) {
            // Add order
            const price = 50000 + (i % 1000);
            const side = i % 2 === 0 ? 'Buy' : 'Sell';
            orderbook.addLimitOrder(price, 100, side, 'GTC');
        } else {
            // Query prices
            orderbook.bestBid();
            orderbook.bestAsk();
        }
    }
    
    const mixedEnd = process.hrtime.bigint();
    const mixedTime = Number(mixedEnd - mixedStart) / 1e6;
    const mixedOpsPerSec = Math.round(BENCHMARK_CONFIG.benchmarkRuns / (mixedTime / 1000));
    
    console.log(`✅ Mixed Operations: ${formatNumber(mixedOpsPerSec)} ops/sec`);
    
    // Results Summary
    console.log('\n============================================');
    console.log('📊 BENCHMARK SUMMARY');
    console.log('============================================');
    console.log(`Order Book Creation....... ${formatNumber(creationOpsPerSec)} ops/sec`);
    console.log(`Order Insertion........... ${formatNumber(insertionOpsPerSec)} ops/sec`);
    console.log(`Price Queries............. ${formatNumber(queryOpsPerSec)} ops/sec`);
    console.log(`Mixed Operations.......... ${formatNumber(mixedOpsPerSec)} ops/sec`);
    console.log('--------------------------------------------');
    
    const avgOps = Math.round((creationOpsPerSec + insertionOpsPerSec + queryOpsPerSec + mixedOpsPerSec) / 4);
    console.log(`Overall Average........... ${formatNumber(avgOps)} ops/sec`);
    console.log(`Total Operations.......... ${formatNumber(BENCHMARK_CONFIG.benchmarkRuns * 4)} ops`);
    console.log(`Node.js Version........... ${process.version}`);
    
    // Save results
    const results = {
        timestamp: new Date().toISOString(),
        nodeVersion: process.version,
        platform: process.platform,
        architecture: process.arch,
        benchmarks: {
            orderBookCreation: creationOpsPerSec,
            orderInsertion: insertionOpsPerSec,
            priceQueries: queryOpsPerSec,
            mixedOperations: mixedOpsPerSec,
            overallAverage: avgOps
        },
        configuration: BENCHMARK_CONFIG
    };
    
    require('fs').writeFileSync('benchmark_results_nodejs.json', JSON.stringify(results, null, 2));
    console.log('\n📄 Results saved to benchmark_results_nodejs.json');
    
    return results;
}

// Provide workaround instructions
function provideWorkaround() {
    console.log('\n🔧 WORKAROUND INSTRUCTIONS');
    console.log('===========================');
    console.log('The Node.js binding has compatibility issues with Node.js v22+');
    console.log('');
    console.log('To fix this issue:');
    console.log('1. Install Node.js v16 or v18:');
    console.log('   nvm install 18');
    console.log('   nvm use 18');
    console.log('');
    console.log('2. Rebuild the binding:');
    console.log('   npm run build');
    console.log('');
    console.log('3. Run the benchmark again:');
    console.log('   node benchmark_nodejs.js');
    console.log('');
    console.log('Alternative: Use the Python bindings which work correctly:');
    console.log('   python3 src/bindings/tests/benchmark_tests.py');
    console.log('');
    console.log('Expected Node.js performance (when working):');
    console.log('- Order Insertion: ~40,000-60,000 ops/sec');
    console.log('- Price Queries: ~100,000-150,000 ops/sec');
    console.log('- Mixed Operations: ~50,000-80,000 ops/sec');
}

// Main execution
function main() {
    const isCompatible = checkCompatibility();
    const binding = loadBinding();
    
    if (binding && binding.JsOrderBook) {
        console.log('\n🎉 Node.js binding loaded successfully!');
        const results = runBenchmarks(binding);
        
        // Compare with Python results if available
        try {
            const pythonResults = JSON.parse(require('fs').readFileSync('benchmark_results_python.json', 'utf8'));
            console.log('\n📈 PERFORMANCE COMPARISON WITH PYTHON');
            console.log('=====================================');
            console.log('Node.js vs Python:');
            console.log(`Order Insertion: ${formatNumber(results.benchmarks.orderInsertion)} vs ${formatNumber(pythonResults.benchmarks.single_order_insertion || 0)} ops/sec`);
            console.log(`Price Queries: ${formatNumber(results.benchmarks.priceQueries)} vs ${formatNumber(pythonResults.benchmarks.price_queries || 0)} ops/sec`);
            console.log(`Mixed Operations: ${formatNumber(results.benchmarks.mixedOperations)} vs ${formatNumber(pythonResults.benchmarks.mixed_operations || 0)} ops/sec`);
        } catch (error) {
            console.log('\n💡 Run Python benchmarks first to enable comparison:');
            console.log('   python3 src/bindings/tests/benchmark_tests.py');
        }
        
    } else {
        console.log('\n❌ Node.js binding failed to load');
        provideWorkaround();
    }
}

// Run the benchmark
main();