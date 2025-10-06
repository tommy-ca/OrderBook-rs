use crate::orderbook::snapshot::OrderBookSnapshotPackage;
use crate::orderbook::trade::TradeEvent;
use pricelevel::{OrderId, PriceLevelSnapshot, Side, Transaction};
use rkyv::{Archive, Deserialize, Infallible, Serialize};
use std::convert::TryFrom;
use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur while archiving or restoring zero-copy payloads.
#[derive(Debug, Error)]
pub enum ZeroCopyArchiveError {
    /// Serialization failure when creating the archive buffer.
    #[error("serialization error: {0}")]
    Serialization(String),
    /// Deserialization failure when reading an archive buffer.
    #[error("deserialization error: {0}")]
    Deserialization(String),
    /// Numeric overflow during conversion to archive-friendly primitives.
    #[error("numeric overflow while encoding {0}")]
    NumericOverflow(&'static str),
}

pub type ArchiveResult<T> = Result<T, ZeroCopyArchiveError>;

/// Archived representation of an order book snapshot.
#[derive(Archive, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SnapshotArchive {
    pub version: u32,
    pub symbol: String,
    pub timestamp: u64,
    pub checksum: String,
    pub bids: Vec<LevelArchive>,
    pub asks: Vec<LevelArchive>,
}

/// Archived representation of a single price level summary.
#[derive(Archive, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LevelArchive {
    pub price: u64,
    pub visible_quantity: u64,
    pub hidden_quantity: u64,
    pub order_count: u32,
}

/// Archived representation of a trade event.
#[derive(Archive, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TradeEventArchive {
    pub symbol: String,
    pub timestamp: u64,
    pub order_id: OrderIdArchive,
    pub remaining_quantity: u64,
    pub is_complete: bool,
    pub transactions: Vec<TradeTransactionArchive>,
    pub filled_order_ids: Vec<OrderIdArchive>,
}

/// Archived representation of an individual trade transaction.
#[derive(Archive, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TradeTransactionArchive {
    pub transaction_id: [u8; 16],
    pub taker_order: OrderIdArchive,
    pub maker_order: OrderIdArchive,
    pub price: u64,
    pub quantity: u64,
    pub taker_side: u8,
    pub timestamp: u64,
}

/// Archived order identifier wrapper (UUID or ULID).
#[derive(Archive, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OrderIdArchive {
    pub bytes: [u8; 16],
    pub encoding: u8,
}

/// Serialize an `OrderBookSnapshotPackage` into a rkyv archive buffer.
pub fn serialize_snapshot_archive(package: &OrderBookSnapshotPackage) -> ArchiveResult<Vec<u8>> {
    let snapshot = &package.snapshot;
    let bids = snapshot
        .bids
        .iter()
        .map(level_to_archive)
        .collect::<ArchiveResult<Vec<_>>>()?;
    let asks = snapshot
        .asks
        .iter()
        .map(level_to_archive)
        .collect::<ArchiveResult<Vec<_>>>()?;

    let archive = SnapshotArchive {
        version: package.version,
        symbol: snapshot.symbol.clone(),
        timestamp: snapshot.timestamp,
        checksum: package.checksum.clone(),
        bids,
        asks,
    };

    rkyv::to_bytes::<_, 256>(&archive)
        .map(|bytes| bytes.into_vec())
        .map_err(|err| ZeroCopyArchiveError::Serialization(err.to_string()))
}

/// Deserialize a snapshot archive buffer into its structured representation.
pub fn deserialize_snapshot_archive(bytes: &[u8]) -> ArchiveResult<SnapshotArchive> {
    unsafe { rkyv::archived_root::<SnapshotArchive>(bytes) }
        .deserialize(&mut Infallible)
        .map_err(|err| ZeroCopyArchiveError::Deserialization(err.to_string()))
}

/// Serialize a `TradeEvent` into a rkyv archive buffer.
pub fn serialize_trade_event_archive(event: &TradeEvent) -> ArchiveResult<Vec<u8>> {
    let match_result = &event.trade_result.match_result;
    let transactions = match_result
        .transactions
        .as_vec()
        .iter()
        .copied()
        .map(transaction_to_archive)
        .collect::<ArchiveResult<Vec<_>>>()?;

    let filled_order_ids = match_result
        .filled_order_ids
        .iter()
        .map(order_id_to_archive)
        .collect::<Vec<_>>();

    let archive = TradeEventArchive {
        symbol: event.symbol.clone(),
        timestamp: event.timestamp,
        order_id: order_id_to_archive(&match_result.order_id),
        remaining_quantity: match_result.remaining_quantity,
        is_complete: match_result.is_complete,
        transactions,
        filled_order_ids,
    };

    rkyv::to_bytes::<_, 256>(&archive)
        .map(|bytes| bytes.into_vec())
        .map_err(|err| ZeroCopyArchiveError::Serialization(err.to_string()))
}

/// Deserialize a trade event archive buffer into its structured representation.
pub fn deserialize_trade_event_archive(bytes: &[u8]) -> ArchiveResult<TradeEventArchive> {
    unsafe { rkyv::archived_root::<TradeEventArchive>(bytes) }
        .deserialize(&mut Infallible)
        .map_err(|err| ZeroCopyArchiveError::Deserialization(err.to_string()))
}

fn level_to_archive(level: &PriceLevelSnapshot) -> ArchiveResult<LevelArchive> {
    let order_count = u32::try_from(level.order_count)
        .map_err(|_| ZeroCopyArchiveError::NumericOverflow("order_count"))?;
    Ok(LevelArchive {
        price: level.price,
        visible_quantity: level.visible_quantity,
        hidden_quantity: level.hidden_quantity,
        order_count,
    })
}

fn transaction_to_archive(transaction: Transaction) -> ArchiveResult<TradeTransactionArchive> {
    Ok(TradeTransactionArchive {
        transaction_id: transaction.transaction_id.as_bytes().to_owned(),
        taker_order: order_id_to_archive(&transaction.taker_order_id),
        maker_order: order_id_to_archive(&transaction.maker_order_id),
        price: transaction.price,
        quantity: transaction.quantity,
        taker_side: side_to_u8(transaction.taker_side),
        timestamp: transaction.timestamp,
    })
}

fn order_id_to_archive(id: &OrderId) -> OrderIdArchive {
    match id {
        OrderId::Uuid(uuid) => OrderIdArchive {
            bytes: *uuid.as_bytes(),
            encoding: 0,
        },
        OrderId::Ulid(ulid) => OrderIdArchive {
            bytes: ulid.to_bytes(),
            encoding: 1,
        },
    }
}

fn side_to_u8(side: Side) -> u8 {
    match side {
        Side::Buy => 0,
        Side::Sell => 1,
    }
}

/// Convert little-endian UUID bytes back into a `Uuid`.
pub fn uuid_from_bytes(bytes: [u8; 16]) -> Uuid {
    Uuid::from_bytes(bytes)
}
