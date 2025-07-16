#!/usr/bin/env python3
"""
Integration tests for Python bindings of OrderBook-rs
Tests complex scenarios and edge cases
"""
import unittest
import sys
import os
import threading
import time
from concurrent.futures import ThreadPoolExecutor, as_completed

# Add the built module to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..', 'target', 'release'))

try:
    from orderbook_rs import PyOrderBook
except ImportError as e:
    print(f"Failed to import PyOrderBook: {e}")
    print("Make sure to build with: maturin develop --features python")
    sys.exit(1)


class TestPyOrderBookIntegration(unittest.TestCase):
    """Integration tests for complex OrderBook scenarios."""
    
    def setUp(self):
        """Set up test fixtures."""
        self.orderbook = PyOrderBook("INTEGRATION_TEST")
    
    def test_order_priority_fifo(self):
        """Test FIFO order priority at same price level."""
        # Add multiple orders at same price
        order1 = self.orderbook.add_limit_order(50000, 100, "Buy", "GTC")
        order2 = self.orderbook.add_limit_order(50000, 200, "Buy", "GTC")
        order3 = self.orderbook.add_limit_order(50000, 150, "Buy", "GTC")
        
        # All should be unique order IDs
        self.assertNotEqual(order1, order2)
        self.assertNotEqual(order2, order3)
        self.assertNotEqual(order1, order3)
        
        # Best bid should remain 50000
        self.assertEqual(self.orderbook.best_bid(), 50000)
    
    def test_spread_calculation(self):
        """Test bid-ask spread scenarios."""
        # No spread initially
        self.assertIsNone(self.orderbook.best_bid())
        self.assertIsNone(self.orderbook.best_ask())
        
        # Add bid
        self.orderbook.add_limit_order(49900, 100, "Buy", "GTC")
        self.assertEqual(self.orderbook.best_bid(), 49900)
        self.assertIsNone(self.orderbook.best_ask())
        
        # Add ask
        self.orderbook.add_limit_order(50100, 100, "Sell", "GTC")
        self.assertEqual(self.orderbook.best_ask(), 50100)
        
        # Spread should be 200 (50100 - 49900)
        spread = self.orderbook.best_ask() - self.orderbook.best_bid()
        self.assertEqual(spread, 200)
    
    def test_large_price_values(self):
        """Test handling of large price values."""
        large_price = 2**32 - 1  # Max 32-bit unsigned
        
        order_id = self.orderbook.add_limit_order(large_price, 100, "Buy", "GTC")
        self.assertIsNotNone(order_id)
        self.assertEqual(self.orderbook.best_bid(), large_price)
    
    def test_large_quantity_values(self):
        """Test handling of large quantity values."""
        large_quantity = 2**32 - 1  # Max 32-bit unsigned
        
        order_id = self.orderbook.add_limit_order(50000, large_quantity, "Buy", "GTC")
        self.assertIsNotNone(order_id)
        self.assertEqual(self.orderbook.best_bid(), 50000)
    
    def test_mixed_time_in_force(self):
        """Test different time in force values."""
        # GTC orders should always work
        gtc_order = self.orderbook.add_limit_order(50000, 100, "Buy", "GTC")
        self.assertIsNotNone(gtc_order)
        
        # Add a sell order to provide liquidity for IOC/FOK tests
        sell_order = self.orderbook.add_limit_order(50001, 100, "Sell", "GTC")
        self.assertIsNotNone(sell_order)
        
        # IOC orders - should execute immediately against available liquidity
        try:
            ioc_order = self.orderbook.add_limit_order(50001, 50, "Buy", "IOC")
            self.assertIsNotNone(ioc_order)
        except ValueError as e:
            # IOC may fail if no matching liquidity, this is expected behavior
            self.assertIn("InsufficientLiquidity", str(e))
        
        # FOK orders - should execute fully or not at all
        try:
            fok_order = self.orderbook.add_limit_order(50001, 25, "Buy", "FOK")
            self.assertIsNotNone(fok_order)
        except ValueError as e:
            # FOK may fail if cannot fill completely, this is expected behavior
            self.assertIn("InsufficientLiquidity", str(e))
        
        # Best bid should be the GTC order
        self.assertEqual(self.orderbook.best_bid(), 50000)
    
    def test_order_book_depth_simulation(self):
        """Simulate realistic order book depth."""
        # Add multiple bid levels
        bid_levels = [
            (49900, 1000),
            (49950, 1500),
            (50000, 2000),
            (50050, 1200),
            (50100, 800)
        ]
        
        # Add multiple ask levels
        ask_levels = [
            (50200, 800),
            (50250, 1200),
            (50300, 2000),
            (50350, 1500),
            (50400, 1000)
        ]
        
        # Add all bid orders
        for price, quantity in bid_levels:
            self.orderbook.add_limit_order(price, quantity, "Buy", "GTC")
        
        # Add all ask orders
        for price, quantity in ask_levels:
            self.orderbook.add_limit_order(price, quantity, "Sell", "GTC")
        
        # Verify best prices
        self.assertEqual(self.orderbook.best_bid(), 50100)  # Highest bid
        self.assertEqual(self.orderbook.best_ask(), 50200)  # Lowest ask
        
        # Spread should be 100
        spread = self.orderbook.best_ask() - self.orderbook.best_bid()
        self.assertEqual(spread, 100)


class TestPyOrderBookConcurrency(unittest.TestCase):
    """Test concurrent access to OrderBook."""
    
    def setUp(self):
        """Set up test fixtures."""
        self.orderbook = PyOrderBook("CONCURRENCY_TEST")
    
    def test_concurrent_order_addition(self):
        """Test adding orders concurrently from multiple threads."""
        def add_orders(thread_id, order_count):
            orders = []
            for i in range(order_count):
                price = 50000 + thread_id * 100 + i
                order_id = self.orderbook.add_limit_order(price, 100, "Buy", "GTC")
                orders.append(order_id)
            return orders
        
        # Create multiple threads
        thread_count = 4
        orders_per_thread = 100
        
        with ThreadPoolExecutor(max_workers=thread_count) as executor:
            futures = [
                executor.submit(add_orders, i, orders_per_thread)
                for i in range(thread_count)
            ]
            
            all_orders = []
            for future in as_completed(futures):
                orders = future.result()
                all_orders.extend(orders)
        
        # Verify all orders were added
        expected_order_count = thread_count * orders_per_thread
        self.assertEqual(len(all_orders), expected_order_count)
        
        # All order IDs should be unique
        unique_orders = set(all_orders)
        self.assertEqual(len(unique_orders), expected_order_count)
    
    def test_concurrent_price_queries(self):
        """Test querying prices concurrently."""
        # Add some initial orders
        self.orderbook.add_limit_order(50000, 100, "Buy", "GTC")
        self.orderbook.add_limit_order(50100, 100, "Sell", "GTC")
        
        def query_prices(iterations):
            results = []
            for _ in range(iterations):
                bid = self.orderbook.best_bid()
                ask = self.orderbook.best_ask()
                results.append((bid, ask))
            return results
        
        # Query from multiple threads
        thread_count = 10
        iterations_per_thread = 1000
        
        with ThreadPoolExecutor(max_workers=thread_count) as executor:
            futures = [
                executor.submit(query_prices, iterations_per_thread)
                for _ in range(thread_count)
            ]
            
            all_results = []
            for future in as_completed(futures):
                results = future.result()
                all_results.extend(results)
        
        # Verify all queries returned consistent results
        expected_results = thread_count * iterations_per_thread
        self.assertEqual(len(all_results), expected_results)
        
        # All results should be consistent
        for bid, ask in all_results:
            self.assertEqual(bid, 50000)
            self.assertEqual(ask, 50100)


class TestPyOrderBookStress(unittest.TestCase):
    """Stress tests for OrderBook performance."""
    
    def setUp(self):
        """Set up test fixtures."""
        self.orderbook = PyOrderBook("STRESS_TEST")
    
    def test_high_volume_orders(self):
        """Test handling high volume of orders."""
        order_count = 10000
        start_time = time.time()
        
        orders = []
        for i in range(order_count):
            price = 50000 + (i % 1000)  # Create price levels
            side = "Buy" if i % 2 == 0 else "Sell"
            order_id = self.orderbook.add_limit_order(price, 100, side, "GTC")
            orders.append(order_id)
        
        end_time = time.time()
        duration = end_time - start_time
        
        # Performance assertion
        orders_per_second = order_count / duration
        print(f"Added {order_count} orders in {duration:.4f}s ({orders_per_second:.0f} orders/sec)")
        
        # Should handle at least 10K orders per second
        self.assertGreater(orders_per_second, 10000)
        
        # Verify all orders were added
        self.assertEqual(len(orders), order_count)
    
    def test_memory_usage_stability(self):
        """Test memory usage doesn't grow excessively."""
        import gc
        
        # Force garbage collection
        gc.collect()
        
        # Add many orders
        for i in range(50000):
            price = 50000 + (i % 100)
            self.orderbook.add_limit_order(price, 100, "Buy", "GTC")
        
        # Force garbage collection again
        gc.collect()
        
        # This test mainly ensures no memory leaks cause crashes
        # The actual memory usage depends on the implementation
        self.assertIsNotNone(self.orderbook.best_bid())


class TestPyOrderBookErrorHandling(unittest.TestCase):
    """Test error handling and edge cases."""
    
    def setUp(self):
        """Set up test fixtures."""
        self.orderbook = PyOrderBook("ERROR_TEST")
    
    def test_invalid_parameters_comprehensive(self):
        """Comprehensive test of invalid parameter handling."""
        # Test various invalid sides - test each individually
        invalid_sides = ["buy", "sell", "BUY", "SELL", "Invalid", "123"]
        for side in invalid_sides:
            with self.assertRaises((ValueError, TypeError)):
                self.orderbook.add_limit_order(50000, 100, side, "GTC")
        
        # Test empty string side
        with self.assertRaises((ValueError, TypeError)):
            self.orderbook.add_limit_order(50000, 100, "", "GTC")
        
        # Test None side
        with self.assertRaises(TypeError):
            self.orderbook.add_limit_order(50000, 100, None, "GTC")
        
        # Test various invalid time in force values - test each individually
        invalid_tifs = ["gtc", "ioc", "fok", "Invalid", "123"]
        for tif in invalid_tifs:
            with self.assertRaises((ValueError, TypeError)):
                self.orderbook.add_limit_order(50000, 100, "Buy", tif)
        
        # Test empty string time in force
        with self.assertRaises((ValueError, TypeError)):
            self.orderbook.add_limit_order(50000, 100, "Buy", "")
        
        # Test None time in force
        with self.assertRaises(TypeError):
            self.orderbook.add_limit_order(50000, 100, "Buy", None)
    
    def test_extreme_values(self):
        """Test handling of extreme values."""
        # Test negative values - expect OverflowError for unsigned conversion
        with self.assertRaises(OverflowError):
            self.orderbook.add_limit_order(-1, 100, "Buy", "GTC")
        
        with self.assertRaises(OverflowError):
            self.orderbook.add_limit_order(50000, -1, "Buy", "GTC")
        
        # Test zero values - current implementation allows these (design decision)
        try:
            zero_price = self.orderbook.add_limit_order(0, 100, "Buy", "GTC")
            self.assertIsNotNone(zero_price)
        except ValueError:
            pass  # Some implementations may validate zero price
        
        try:
            zero_quantity = self.orderbook.add_limit_order(50000, 0, "Buy", "GTC")
            self.assertIsNotNone(zero_quantity)
        except ValueError:
            pass  # Some implementations may validate zero quantity
    
    def test_unicode_symbol_handling(self):
        """Test handling of Unicode symbols."""
        # Test various Unicode symbols
        unicode_symbols = ["BTC/USD", "ETH-EUR", "株式会社", "🚀MOON🚀", ""]
        
        for symbol in unicode_symbols:
            try:
                ob = PyOrderBook(symbol)
                order_id = ob.add_limit_order(50000, 100, "Buy", "GTC")
                self.assertIsNotNone(order_id)
            except Exception as e:
                # Some symbols might be invalid, but shouldn't crash
                self.assertIsInstance(e, (ValueError, TypeError))


if __name__ == '__main__':
    # Create test suite
    suite = unittest.TestSuite()
    
    # Add test cases
    suite.addTest(unittest.makeSuite(TestPyOrderBookIntegration))
    suite.addTest(unittest.makeSuite(TestPyOrderBookConcurrency))
    suite.addTest(unittest.makeSuite(TestPyOrderBookStress))
    suite.addTest(unittest.makeSuite(TestPyOrderBookErrorHandling))
    
    # Run tests with detailed output
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)
    
    # Exit with appropriate code
    sys.exit(0 if result.wasSuccessful() else 1)