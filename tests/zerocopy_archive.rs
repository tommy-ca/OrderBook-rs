use std::error::Error;

use orderbook_rs::TradeResult;
use orderbook_rs::orderbook::{OrderBook, OrderBookSnapshotPackage};
use orderbook_rs::zerocopy::{
    deserialize_snapshot_archive, deserialize_trade_event_archive, serialize_snapshot_archive,
    serialize_trade_event_archive,
};
use pricelevel::{MatchResult, OrderId, Side};
use uuid::Uuid;

#[test]
fn snapshot_archive_roundtrip() -> Result<(), Box<dyn Error>> {
    let orderbook = OrderBook::<()>::new("ETH-USD");
    let snapshot = orderbook.create_snapshot(5);
    let package = OrderBookSnapshotPackage::new(snapshot)?;

    let bytes = serialize_snapshot_archive(&package)?;
    let archive = deserialize_snapshot_archive(&bytes)?;

    assert_eq!(archive.symbol, package.snapshot.symbol);
    assert_eq!(archive.version, package.version);
    assert_eq!(archive.checksum, package.checksum);
    assert!(archive.bids.is_empty());
    assert!(archive.asks.is_empty());

    Ok(())
}

#[test]
fn trade_event_archive_preserves_identifiers() -> Result<(), Box<dyn Error>> {
    let symbol = "SOL-USD".to_string();
    let taker_id = OrderId::new();
    let maker_id = OrderId::new();
    let mut match_result = MatchResult::new(taker_id, 10);
    let transaction =
        pricelevel::Transaction::new(Uuid::new_v4(), taker_id, maker_id, 42, 10, Side::Buy);
    match_result.add_transaction(transaction);
    let trade_result = TradeResult::new(symbol.clone(), match_result.clone());

    let event = orderbook_rs::orderbook::trade::TradeEvent {
        symbol: symbol.clone(),
        trade_result,
        timestamp: 1_234,
    };

    let bytes = serialize_trade_event_archive(&event)?;
    let archive = deserialize_trade_event_archive(&bytes)?;

    assert_eq!(archive.symbol, symbol);
    assert_eq!(
        archive.order_id.bytes,
        order_id_to_bytes(&match_result.order_id)
    );
    assert_eq!(archive.transactions.len(), 1);
    let archived_tx = &archive.transactions[0];
    assert_eq!(archived_tx.price, 42);
    assert_eq!(archived_tx.quantity, 10);
    assert_eq!(
        archived_tx.taker_order.bytes,
        order_id_to_bytes(&match_result.order_id)
    );
    assert_eq!(archived_tx.price, 42);
    assert_eq!(archived_tx.quantity, 10);
    assert_eq!(archived_tx.taker_side, 0); // 0 => Buy
    assert_eq!(archive.remaining_quantity, match_result.remaining_quantity);
    assert_eq!(archive.is_complete, match_result.is_complete);
    assert_eq!(
        archive.filled_order_ids.len(),
        match_result.filled_order_ids.len()
    );

    Ok(())
}

fn order_id_to_bytes(id: &OrderId) -> [u8; 16] {
    match id {
        OrderId::Uuid(uuid) => *uuid.as_bytes(),
        OrderId::Ulid(ulid) => ulid.to_bytes(),
    }
}
