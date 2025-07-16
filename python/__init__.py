"""
OrderBook-rs Python bindings

High-performance, lock-free price level implementation for limit order books.
"""

from .orderbook_rs import PyOrderBook

__version__ = "0.2.0"
__author__ = "Joaquin Bejar"
__email__ = "jb@taunais.com"

__all__ = ["PyOrderBook"]