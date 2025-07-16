"""
Type stubs for orderbook_rs Python bindings
"""

from typing import Optional, Literal

OrderSide = Literal["Buy", "Sell"]
TimeInForce = Literal["GTC", "IOC", "FOK"]

class PyOrderBook:
    """
    High-performance OrderBook implementation with lock-free operations
    """
    
    def __init__(self, symbol: str) -> None:
        """
        Create a new OrderBook for the specified symbol
        
        Args:
            symbol: Trading symbol (e.g., "BTCUSD")
        """
        ...
    
    def add_limit_order(
        self, 
        price: int, 
        quantity: int, 
        side: OrderSide, 
        time_in_force: TimeInForce
    ) -> str:
        """
        Add a limit order to the order book
        
        Args:
            price: Order price in smallest unit (e.g., cents)
            quantity: Order quantity
            side: Order side ("Buy" or "Sell")
            time_in_force: Time in force ("GTC", "IOC", or "FOK")
            
        Returns:
            Order ID (UUID string)
            
        Raises:
            ValueError: If parameters are invalid or order cannot be placed
            TypeError: If parameter types are incorrect
        """
        ...
    
    def best_bid(self) -> Optional[int]:
        """
        Get the best bid price
        
        Returns:
            Best bid price or None if no bids exist
        """
        ...
    
    def best_ask(self) -> Optional[int]:
        """
        Get the best ask price
        
        Returns:
            Best ask price or None if no asks exist
        """
        ...