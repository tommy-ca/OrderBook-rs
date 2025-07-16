#!/usr/bin/env python3

import sys
sys.path.append('./python')

try:
    from orderbook_rs import PyOrderBook
    print("Successfully imported PyOrderBook")
except ImportError as e:
    print(f"Failed to import PyOrderBook: {e}")
    sys.exit(1)

def test_basic_functionality():
    print("=== Basic Python Binding Test ===\n")
    
    # Create orderbook
    book = PyOrderBook('BTC/USD')
    print(f"1. Created orderbook for symbol: {book.symbol()}")
    
    # Test basic functionality
    print("\n2. Testing basic functionality:")
    print(f"   Best bid: {book.best_bid()}")
    print(f"   Best ask: {book.best_ask()}")
    print(f"   Spread: {book.spread()}")
    print(f"   Mid price: {book.mid_price()}")
    print(f"   Total orders: {book.total_orders()}")
    
    # Test adding limit orders
    print("\n3. Testing limit orders:")
    try:
        buy_order_id = book.add_limit_order(100, 10, 'buy', 'gtc')
        print(f"   Added buy order: {buy_order_id}")
        
        sell_order_id = book.add_limit_order(101, 10, 'sell', 'gtc')
        print(f"   Added sell order: {sell_order_id}")
        
        print(f"   Best bid after orders: {book.best_bid()}")
        print(f"   Best ask after orders: {book.best_ask()}")
        print(f"   Spread after orders: {book.spread()}")
        print(f"   Mid price after orders: {book.mid_price()}")
        print(f"   Total orders after orders: {book.total_orders()}")
    except Exception as e:
        print(f"   Error adding limit orders: {e}")
    
    # Test iceberg orders
    print("\n4. Testing iceberg orders:")
    try:
        iceberg_order_id = book.add_iceberg_order(99, 5, 15, 'buy', 'gtc')
        print(f"   Added iceberg order: {iceberg_order_id}")
        print(f"   Total orders after iceberg: {book.total_orders()}")
    except Exception as e:
        print(f"   Error adding iceberg order: {e}")
    
    # Test post-only orders
    print("\n5. Testing post-only orders:")
    try:
        post_only_order_id = book.add_post_only_order(98, 8, 'buy', 'gtc')
        print(f"   Added post-only order: {post_only_order_id}")
        print(f"   Total orders after post-only: {book.total_orders()}")
    except Exception as e:
        print(f"   Error adding post-only order: {e}")
    
    # Test market orders
    print("\n6. Testing market orders:")
    try:
        market_result = book.submit_market_order(5, 'buy')
        print("   Market order result:")
        print(f"     Order ID: {market_result.order_id}")
        print(f"     Executed quantity: {market_result.executed_quantity}")
        print(f"     Remaining quantity: {market_result.remaining_quantity}")
        print(f"     Is complete: {market_result.is_complete}")
        print(f"     Transactions: {len(market_result.transactions)}")
        print(f"     Filled order IDs: {market_result.filled_order_ids}")
        print(f"     Executed value: {market_result.executed_value}")
        print(f"     Average price: {market_result.average_price}")
    except Exception as e:
        print(f"   Error with market order: {e}")
    
    # Test snapshot functionality
    print("\n7. Testing snapshot functionality:")
    try:
        snapshot = book.create_snapshot(3)
        print("   Snapshot:")
        print(f"     Symbol: {snapshot.symbol}")
        print(f"     Timestamp: {snapshot.timestamp}")
        print(f"     Bids: {len(snapshot.bids)} levels")
        print(f"     Asks: {len(snapshot.asks)} levels")
        
        if snapshot.bids:
            top_bid = snapshot.bids[0]
            print(f"     Top bid: price={top_bid.price}, visible={top_bid.visible_quantity}, hidden={top_bid.hidden_quantity}, orders={top_bid.order_count}")
        
        if snapshot.asks:
            top_ask = snapshot.asks[0]
            print(f"     Top ask: price={top_ask.price}, visible={top_ask.visible_quantity}, hidden={top_ask.hidden_quantity}, orders={top_ask.order_count}")
    except Exception as e:
        print(f"   Error creating snapshot: {e}")
    
    print("\n=== Test Complete ===")

if __name__ == "__main__":
    test_basic_functionality()