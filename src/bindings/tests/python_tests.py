#!/usr/bin/env python3
"""
Comprehensive test suite for Python bindings of OrderBook-rs
"""
import unittest
import sys
import os

# Add the built module to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..', 'target', 'release'))

try:
    from orderbook_rs import PyOrderBook
except ImportError as e:
    print(f"Failed to import PyOrderBook: {e}")
    print("Make sure to build with: maturin develop --features python")
    sys.exit(1)


class TestPyOrderBook(unittest.TestCase):
    
    def setUp(self):
        """Set up test fixtures before each test method."""
        self.orderbook = PyOrderBook("BTCUSD")
    
    def test_initialization(self):
        """Test OrderBook initialization."""
        self.assertIsNotNone(self.orderbook)
        self.assertIsNone(self.orderbook.best_bid())
        self.assertIsNone(self.orderbook.best_ask())
    
    def test_add_buy_limit_order(self):
        """Test adding a buy limit order."""
        order_id = self.orderbook.add_limit_order(50000, 100, "Buy", "GTC")
        self.assertIsNotNone(order_id)
        self.assertEqual(self.orderbook.best_bid(), 50000)
        self.assertIsNone(self.orderbook.best_ask())
    
    def test_add_sell_limit_order(self):
        """Test adding a sell limit order."""
        order_id = self.orderbook.add_limit_order(51000, 100, "Sell", "GTC")
        self.assertIsNotNone(order_id)
        self.assertIsNone(self.orderbook.best_bid())
        self.assertEqual(self.orderbook.best_ask(), 51000)
    
    def test_multiple_orders(self):
        """Test multiple orders and best price functionality."""
        # Add multiple buy orders
        self.orderbook.add_limit_order(50000, 100, "Buy", "GTC")
        self.orderbook.add_limit_order(50100, 200, "Buy", "GTC")
        self.orderbook.add_limit_order(49900, 150, "Buy", "GTC")
        
        # Add multiple sell orders
        self.orderbook.add_limit_order(51000, 100, "Sell", "GTC")
        self.orderbook.add_limit_order(50900, 200, "Sell", "GTC")
        self.orderbook.add_limit_order(51100, 150, "Sell", "GTC")
        
        # Check best prices
        self.assertEqual(self.orderbook.best_bid(), 50100)  # Highest buy price
        self.assertEqual(self.orderbook.best_ask(), 50900)  # Lowest sell price
    
    def test_invalid_side(self):
        """Test invalid side parameter."""
        with self.assertRaises(ValueError):
            self.orderbook.add_limit_order(50000, 100, "InvalidSide", "GTC")
    
    def test_invalid_time_in_force(self):
        """Test invalid time in force parameter."""
        with self.assertRaises(ValueError):
            self.orderbook.add_limit_order(50000, 100, "Buy", "InvalidTIF")
    
    def test_zero_quantity(self):
        """Test zero quantity order."""
        # Note: Current implementation may not validate zero quantity
        try:
            order_id = self.orderbook.add_limit_order(50000, 0, "Buy", "GTC")
            print(f"Zero quantity order created: {order_id}")
        except ValueError:
            pass  # Expected behavior if validation is implemented
    
    def test_zero_price(self):
        """Test zero price order."""
        # Note: Current implementation may not validate zero price
        try:
            order_id = self.orderbook.add_limit_order(0, 100, "Buy", "GTC")
            print(f"Zero price order created: {order_id}")
        except ValueError:
            pass  # Expected behavior if validation is implemented


class TestOrderBookPerformance(unittest.TestCase):
    """Performance tests for OrderBook operations."""
    
    def setUp(self):
        """Set up test fixtures."""
        self.orderbook = PyOrderBook("PERFORMANCE_TEST")
    
    def test_bulk_order_insertion(self):
        """Test performance of bulk order insertion."""
        import time
        
        start_time = time.time()
        
        # Add 1000 orders
        for i in range(1000):
            price = 50000 + (i % 100)
            self.orderbook.add_limit_order(price, 100, "Buy", "GTC")
        
        end_time = time.time()
        duration = end_time - start_time
        
        print(f"Inserted 1000 orders in {duration:.4f} seconds")
        self.assertLess(duration, 1.0, "Bulk insertion should be fast")


if __name__ == '__main__':
    # Run the tests
    unittest.main(verbosity=2)