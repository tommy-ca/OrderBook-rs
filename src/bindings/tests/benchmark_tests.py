#!/usr/bin/env python3
"""
Comprehensive benchmark tests for Python bindings of OrderBook-rs
Performance testing and regression detection
"""
import time
import statistics
import sys
import os
import json
import gc
from datetime import datetime
from typing import List, Dict, Any

# Add the built module to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..', 'target', 'release'))

try:
    from orderbook_rs import PyOrderBook
except ImportError as e:
    print(f"Failed to import PyOrderBook: {e}")
    print("Make sure to build with: maturin develop --features python")
    sys.exit(1)


class BenchmarkResult:
    """Container for benchmark results."""
    
    def __init__(self, name: str, operations: int, duration: float, memory_usage: float = 0.0):
        self.name = name
        self.operations = operations
        self.duration = duration
        self.ops_per_second = operations / duration if duration > 0 else 0
        self.memory_usage = memory_usage
        self.timestamp = datetime.now()
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            'name': self.name,
            'operations': self.operations,
            'duration': self.duration,
            'ops_per_second': self.ops_per_second,
            'memory_usage': self.memory_usage,
            'timestamp': self.timestamp.isoformat()
        }


class OrderBookBenchmark:
    """Comprehensive benchmark suite for OrderBook operations."""
    
    def __init__(self):
        self.results: List[BenchmarkResult] = []
        self.warmup_iterations = 1000
        self.benchmark_iterations = 10000
    
    def warmup(self, orderbook: PyOrderBook):
        """Warm up the JIT and caches."""
        print("🔥 Warming up...")
        for i in range(self.warmup_iterations):
            price = 50000 + (i % 100)
            side = "Buy" if i % 2 == 0 else "Sell"
            orderbook.add_limit_order(price, 100, side, "GTC")
    
    def benchmark_single_order_insertion(self) -> BenchmarkResult:
        """Benchmark single order insertion performance."""
        print("📊 Benchmarking single order insertion...")
        
        orderbook = PyOrderBook("BENCHMARK_SINGLE")
        self.warmup(orderbook)
        
        gc.collect()
        start_time = time.perf_counter()
        
        for i in range(self.benchmark_iterations):
            price = 50000 + (i % 1000)
            side = "Buy" if i % 2 == 0 else "Sell"
            orderbook.add_limit_order(price, 100, side, "GTC")
        
        end_time = time.perf_counter()
        duration = end_time - start_time
        
        result = BenchmarkResult("Single Order Insertion", self.benchmark_iterations, duration)
        self.results.append(result)
        return result
    
    def benchmark_batch_order_insertion(self) -> BenchmarkResult:
        """Benchmark batch order insertion performance."""
        print("📊 Benchmarking batch order insertion...")
        
        orderbook = PyOrderBook("BENCHMARK_BATCH")
        batch_size = 1000
        batch_count = 10
        
        gc.collect()
        start_time = time.perf_counter()
        
        for batch in range(batch_count):
            for i in range(batch_size):
                price = 50000 + (i % 100)
                side = "Buy" if i % 2 == 0 else "Sell"
                orderbook.add_limit_order(price, 100, side, "GTC")
        
        end_time = time.perf_counter()
        duration = end_time - start_time
        total_operations = batch_size * batch_count
        
        result = BenchmarkResult("Batch Order Insertion", total_operations, duration)
        self.results.append(result)
        return result
    
    def benchmark_price_queries(self) -> BenchmarkResult:
        """Benchmark price query performance."""
        print("📊 Benchmarking price queries...")
        
        orderbook = PyOrderBook("BENCHMARK_QUERIES")
        
        # Add some orders first
        for i in range(1000):
            price = 50000 + (i % 100)
            side = "Buy" if i % 2 == 0 else "Sell"
            orderbook.add_limit_order(price, 100, side, "GTC")
        
        query_count = 100000
        
        gc.collect()
        start_time = time.perf_counter()
        
        for _ in range(query_count):
            bid = orderbook.best_bid()
            ask = orderbook.best_ask()
        
        end_time = time.perf_counter()
        duration = end_time - start_time
        
        result = BenchmarkResult("Price Queries", query_count * 2, duration)
        self.results.append(result)
        return result
    
    def benchmark_mixed_operations(self) -> BenchmarkResult:
        """Benchmark mixed operations (realistic trading scenario)."""
        print("📊 Benchmarking mixed operations...")
        
        orderbook = PyOrderBook("BENCHMARK_MIXED")
        operation_count = 5000
        
        gc.collect()
        start_time = time.perf_counter()
        
        for i in range(operation_count):
            # 70% order insertion, 30% price queries
            if i % 10 < 7:
                price = 50000 + (i % 100)
                side = "Buy" if i % 2 == 0 else "Sell"
                orderbook.add_limit_order(price, 100, side, "GTC")
            else:
                bid = orderbook.best_bid()
                ask = orderbook.best_ask()
        
        end_time = time.perf_counter()
        duration = end_time - start_time
        
        result = BenchmarkResult("Mixed Operations", operation_count, duration)
        self.results.append(result)
        return result
    
    def benchmark_concurrent_simulation(self) -> BenchmarkResult:
        """Simulate concurrent access patterns."""
        print("📊 Benchmarking concurrent simulation...")
        
        import threading
        import concurrent.futures
        
        orderbook = PyOrderBook("BENCHMARK_CONCURRENT")
        operations_per_thread = 1000
        thread_count = 4
        
        def worker_thread(thread_id: int):
            for i in range(operations_per_thread):
                price = 50000 + thread_id * 1000 + (i % 100)
                side = "Buy" if i % 2 == 0 else "Sell"
                orderbook.add_limit_order(price, 100, side, "GTC")
        
        gc.collect()
        start_time = time.perf_counter()
        
        with concurrent.futures.ThreadPoolExecutor(max_workers=thread_count) as executor:
            futures = [executor.submit(worker_thread, i) for i in range(thread_count)]
            concurrent.futures.wait(futures)
        
        end_time = time.perf_counter()
        duration = end_time - start_time
        total_operations = operations_per_thread * thread_count
        
        result = BenchmarkResult("Concurrent Simulation", total_operations, duration)
        self.results.append(result)
        return result
    
    def benchmark_memory_usage(self) -> BenchmarkResult:
        """Benchmark memory usage patterns."""
        print("📊 Benchmarking memory usage...")
        
        import psutil
        import os
        
        process = psutil.Process(os.getpid())
        initial_memory = process.memory_info().rss / 1024 / 1024  # MB
        
        orderbook = PyOrderBook("BENCHMARK_MEMORY")
        order_count = 50000
        
        gc.collect()
        start_time = time.perf_counter()
        
        for i in range(order_count):
            price = 50000 + (i % 1000)
            side = "Buy" if i % 2 == 0 else "Sell"
            orderbook.add_limit_order(price, 100, side, "GTC")
        
        end_time = time.perf_counter()
        duration = end_time - start_time
        
        final_memory = process.memory_info().rss / 1024 / 1024  # MB
        memory_usage = final_memory - initial_memory
        
        result = BenchmarkResult("Memory Usage", order_count, duration, memory_usage)
        self.results.append(result)
        return result
    
    def benchmark_latency_distribution(self) -> BenchmarkResult:
        """Benchmark latency distribution for individual operations."""
        print("📊 Benchmarking latency distribution...")
        
        orderbook = PyOrderBook("BENCHMARK_LATENCY")
        operation_count = 10000
        latencies = []
        
        # Warm up
        for i in range(100):
            orderbook.add_limit_order(50000 + i, 100, "Buy", "GTC")
        
        gc.collect()
        
        for i in range(operation_count):
            start = time.perf_counter()
            price = 50000 + (i % 1000)
            side = "Buy" if i % 2 == 0 else "Sell"
            orderbook.add_limit_order(price, 100, side, "GTC")
            end = time.perf_counter()
            latencies.append((end - start) * 1000000)  # microseconds
        
        # Calculate statistics
        mean_latency = statistics.mean(latencies)
        median_latency = statistics.median(latencies)
        p95_latency = sorted(latencies)[int(0.95 * len(latencies))]
        p99_latency = sorted(latencies)[int(0.99 * len(latencies))]
        
        print(f"    Mean latency: {mean_latency:.2f} μs")
        print(f"    Median latency: {median_latency:.2f} μs")
        print(f"    95th percentile: {p95_latency:.2f} μs")
        print(f"    99th percentile: {p99_latency:.2f} μs")
        
        total_duration = sum(latencies) / 1000000  # convert back to seconds
        result = BenchmarkResult("Latency Distribution", operation_count, total_duration)
        self.results.append(result)
        return result
    
    def run_all_benchmarks(self):
        """Run all benchmark tests."""
        print("🚀 Starting OrderBook Python Binding Benchmarks")
        print("=" * 60)
        
        benchmarks = [
            self.benchmark_single_order_insertion,
            self.benchmark_batch_order_insertion,
            self.benchmark_price_queries,
            self.benchmark_mixed_operations,
            self.benchmark_concurrent_simulation,
            self.benchmark_memory_usage,
            self.benchmark_latency_distribution
        ]
        
        for benchmark in benchmarks:
            try:
                result = benchmark()
                print(f"✅ {result.name}: {result.ops_per_second:,.0f} ops/sec")
                if result.memory_usage > 0:
                    print(f"    Memory usage: {result.memory_usage:.2f} MB")
            except Exception as e:
                print(f"❌ {benchmark.__name__} failed: {e}")
        
        self.print_summary()
        self.save_results()
    
    def print_summary(self):
        """Print benchmark summary."""
        print("\n" + "=" * 60)
        print("📊 BENCHMARK SUMMARY")
        print("=" * 60)
        
        for result in self.results:
            print(f"{result.name:.<30} {result.ops_per_second:>15,.0f} ops/sec")
        
        if self.results:
            total_ops = sum(r.operations for r in self.results)
            total_time = sum(r.duration for r in self.results)
            average_throughput = total_ops / total_time if total_time > 0 else 0
            
            print("-" * 60)
            print(f"{'Overall Average':.<30} {average_throughput:>15,.0f} ops/sec")
            print(f"{'Total Operations':.<30} {total_ops:>15,} ops")
            print(f"{'Total Time':.<30} {total_time:>15.3f} sec")
    
    def save_results(self):
        """Save benchmark results to JSON file."""
        results_file = "benchmark_results_python.json"
        
        results_data = {
            'timestamp': datetime.now().isoformat(),
            'platform': 'Python',
            'results': [r.to_dict() for r in self.results]
        }
        
        with open(results_file, 'w') as f:
            json.dump(results_data, f, indent=2)
        
        print(f"\n📄 Results saved to {results_file}")
    
    def compare_with_baseline(self, baseline_file: str = "benchmark_baseline.json"):
        """Compare results with baseline performance."""
        if not os.path.exists(baseline_file):
            print(f"⚠️  Baseline file {baseline_file} not found")
            return
        
        try:
            with open(baseline_file, 'r') as f:
                baseline_data = json.load(f)
            
            print(f"\n📈 PERFORMANCE COMPARISON vs {baseline_file}")
            print("=" * 60)
            
            baseline_results = {r['name']: r for r in baseline_data['results']}
            
            for result in self.results:
                if result.name in baseline_results:
                    baseline_ops = baseline_results[result.name]['ops_per_second']
                    current_ops = result.ops_per_second
                    
                    if baseline_ops > 0:
                        improvement = ((current_ops - baseline_ops) / baseline_ops) * 100
                        status = "📈" if improvement > 0 else "📉" if improvement < -5 else "📊"
                        print(f"{result.name:.<30} {status} {improvement:>8.1f}%")
                    else:
                        print(f"{result.name:.<30} ❓ No baseline data")
                else:
                    print(f"{result.name:.<30} 🆕 New benchmark")
                    
        except Exception as e:
            print(f"❌ Error comparing with baseline: {e}")


def main():
    """Main benchmark execution."""
    try:
        benchmark = OrderBookBenchmark()
        benchmark.run_all_benchmarks()
        benchmark.compare_with_baseline()
        
        print("\n🎉 Python binding benchmarks completed successfully!")
        return 0
        
    except KeyboardInterrupt:
        print("\n⏹️  Benchmark interrupted by user")
        return 1
    except Exception as e:
        print(f"❌ Benchmark failed: {e}")
        return 1


if __name__ == '__main__':
    sys.exit(main())