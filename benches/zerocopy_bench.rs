use criterion::{Criterion, criterion_group, criterion_main};
use orderbook_rs::TradeResult;
use orderbook_rs::orderbook::{OrderBook, OrderBookSnapshotPackage};
use orderbook_rs::zerocopy::{
    deserialize_snapshot_archive, serialize_snapshot_archive, serialize_trade_event_archive,
};
use pricelevel::{MatchResult, OrderId, Side};
use uuid::Uuid;

fn snapshot_round_trip(c: &mut Criterion) {
    let orderbook = OrderBook::<()>::new("BENCH");
    let package = orderbook
        .create_snapshot_package(128)
        .expect("snapshot package");

    let json = package.to_json().expect("json");

    c.bench_function("snapshot_json_serialize", |b| {
        b.iter(|| std::hint::black_box(&package).to_json().unwrap())
    });

    c.bench_function("snapshot_json_deserialize", |b| {
        b.iter(|| OrderBookSnapshotPackage::from_json(std::hint::black_box(&json)).unwrap())
    });

    c.bench_function("snapshot_serialize", |b| {
        b.iter(|| serialize_snapshot_archive(std::hint::black_box(&package)))
    });

    let bytes = serialize_snapshot_archive(&package).expect("serialize");

    c.bench_function("snapshot_deserialize", |b| {
        b.iter(|| deserialize_snapshot_archive(std::hint::black_box(&bytes)))
    });
}

fn trade_event_serialize(c: &mut Criterion) {
    let taker = OrderId::new();
    let maker = OrderId::new();
    let mut match_result = MatchResult::new(taker, 100);
    match_result.add_transaction(pricelevel::Transaction::new(
        Uuid::new_v4(),
        taker,
        maker,
        101,
        100,
        Side::Buy,
    ));

    let trade_result = TradeResult::new("BENCH".to_string(), match_result.clone());
    let event = orderbook_rs::orderbook::trade::TradeEvent {
        symbol: "BENCH".to_string(),
        trade_result,
        timestamp: 42,
    };

    c.bench_function("trade_event_serialize", |b| {
        b.iter(|| serialize_trade_event_archive(std::hint::black_box(&event)))
    });
}

fn criterion_benches(c: &mut Criterion) {
    snapshot_round_trip(c);
    trade_event_serialize(c);
}

criterion_group!(benches, criterion_benches);
criterion_main!(benches);
