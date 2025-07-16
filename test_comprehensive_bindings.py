#!/usr/bin/env python3

import sys
sys.path.append('./python')

try:
    from orderbook_rs import PyOrderBook
    print("Successfully imported PyOrderBook")
except ImportError as e:
    print(f"Failed to import PyOrderBook: {e}")
    sys.exit(1)

def test_comprehensive_bindings():
    print("=== OrderBook-rs Comprehensive Python Binding Test ===\n")
    
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
    print(f"   Is crossed: {book.is_crossed()}")
    
    # Test adding limit orders
    print("\n3. Testing limit orders:")
    try:
        buy_order_id = book.add_limit_order(100.0, 10.0, 'buy', 'gtc')
        print(f"   Added buy order: {buy_order_id}")
        
        sell_order_id = book.add_limit_order(101.0, 10.0, 'sell', 'gtc')
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
        iceberg_order_id = book.add_iceberg_order(99.0, 5.0, 15.0, 'buy', 'gtc')
        print(f"   Added iceberg order: {iceberg_order_id}")
        print(f"   Total orders after iceberg: {book.total_orders()}")
    except Exception as e:
        print(f"   Error adding iceberg order: {e}")
    
    # Test post-only orders
    print("\n5. Testing post-only orders:")
    try:
        post_only_order_id = book.add_post_only_order(98.0, 8.0, 'buy', 'gtc')
        print(f"   Added post-only order: {post_only_order_id}")
        print(f"   Total orders after post-only: {book.total_orders()}")
    except Exception as e:
        print(f"   Error adding post-only order: {e}")
    
    # Test market orders
    print("\n6. Testing market orders:")
    try:
        market_result = book.submit_market_order(5.0, 'buy')
        print("   Market order result:")
        print(f"     Order ID: {market_result.order_id}")
        print(f"     Executed quantity: {market_result.executed_quantity}")
        print(f"     Remaining quantity: {market_result.remaining_quantity}")
        print(f"     Is complete: {market_result.is_complete}")
        print(f"     Transactions: {len(market_result.transactions)}")
        print(f"     Filled order IDs: {market_result.filled_order_ids}")
        print(f"     Executed value: {market_result.executed_value}")
        print(f"     Average price: {market_result.average_price}")
        
        if market_result.transactions:
            tx = market_result.transactions[0]
            print("     First transaction:")
            print(f"       Taker: {tx.taker_order_id}")
            print(f"       Maker: {tx.maker_order_id}")
            print(f"       Price: {tx.price}")
            print(f"       Quantity: {tx.quantity}")
            print(f"       Taker side: {tx.taker_side}")
            print(f"       Timestamp: {tx.timestamp}")
    except Exception as e:
        print(f"   Error with market order: {e}")
    
    # Test order management
    print("\n7. Testing order management:")
    try:
        test_order_id = book.add_limit_order(97.0, 12.0, 'buy', 'gtc')
        print(f"   Added test order: {test_order_id}")
        
        order_info = book.get_order(test_order_id)
        print(f"   Order info: {order_info}")
        
        update_result = book.update_order(test_order_id, 15.0)
        print(f"   Update result: {update_result}")
        
        updated_order_info = book.get_order(test_order_id)
        print(f"   Updated order info: {updated_order_info}")
        
        cancel_result = book.cancel_order(test_order_id)
        print(f"   Cancel result: {cancel_result}")
        
        canceled_order_info = book.get_order(test_order_id)
        print(f"   Canceled order info: {canceled_order_info}")
    except Exception as e:
        print(f"   Error with order management: {e}")
    
    # Test snapshot functionality
    print("\n8. Testing snapshot functionality:")
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
    
    # Test volume analysis
    print("\n9. Testing volume analysis:")
    try:
        volume_data = book.get_volume_by_price()
        print(f"   Volume by price: {len(volume_data)} entries")
        for i, entry in enumerate(volume_data[:5]):
            print(f"     Entry {i+1}: {entry}")
        
        print(f"   Total volume: {book.total_volume()}")
    except Exception as e:
        print(f"   Error with volume analysis: {e}")
    
    # Test price level queries
    print("\n10. Testing price level queries:")
    try:
        bid_orders = book.get_orders_at_price(100.0, 'buy')
        print(f"   Orders at bid price 100: {len(bid_orders)}")
        for i, order in enumerate(bid_orders):
            print(f"     Order {i+1}: {order}")
        
        ask_orders = book.get_orders_at_price(101.0, 'sell')
        print(f"   Orders at ask price 101: {len(ask_orders)}")
        for i, order in enumerate(ask_orders):
            print(f"     Order {i+1}: {order}")
    except Exception as e:
        print(f"   Error with price level queries: {e}")
    
    # Test market state
    print("\n11. Final market state:")
    print(f"   Symbol: {book.symbol()}")
    print(f"   Best bid: {book.best_bid()}")
    print(f"   Best ask: {book.best_ask()}")
    print(f"   Spread: {book.spread()}")
    print(f"   Mid price: {book.mid_price()}")
    print(f"   Last trade price: {book.last_trade_price()}")
    print(f"   Total orders: {book.total_orders()}")
    print(f"   Total volume: {book.total_volume()}")
    print(f"   Is crossed: {book.is_crossed()}")
    
    # Test utility functions
    print("\n12. Testing utility functions:")
    try:
        import time
        book.set_market_close_timestamp(time.time() * 1000 + 8 * 60 * 60 * 1000)  # 8 hours from now
        print("   Set market close timestamp")
        
        # Don't clear the book in the test to preserve state
        # book.clear()
        # print("   Cleared orderbook")
    except Exception as e:
        print(f"   Error with utility functions: {e}")
    
    print("\n=== Test Complete ===")

if __name__ == "__main__":
    test_comprehensive_bindings()