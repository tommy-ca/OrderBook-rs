#![allow(unsafe_op_in_unsafe_fn)]

use crate::orderbook::OrderBook;
use pricelevel::{OrderId, OrderUpdate, PriceLevelError, Side, TimeInForce};
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct PyOrderBook {
    inner: OrderBook,
}

#[pyclass]
#[derive(Clone)]
pub struct PyMatchResult {
    #[pyo3(get)]
    pub order_id: String,
    #[pyo3(get)]
    pub executed_quantity: u64,
    #[pyo3(get)]
    pub remaining_quantity: u64,
    #[pyo3(get)]
    pub is_complete: bool,
    #[pyo3(get)]
    pub transactions: Vec<PyTransaction>,
    #[pyo3(get)]
    pub filled_order_ids: Vec<String>,
    #[pyo3(get)]
    pub executed_value: u64,
    #[pyo3(get)]
    pub average_price: Option<f64>,
}

#[pyclass]
#[derive(Clone)]
pub struct PyTransaction {
    #[pyo3(get)]
    pub taker_order_id: String,
    #[pyo3(get)]
    pub maker_order_id: String,
    #[pyo3(get)]
    pub price: u64,
    #[pyo3(get)]
    pub quantity: u64,
    #[pyo3(get)]
    pub taker_side: String,
    #[pyo3(get)]
    pub timestamp: u64,
}

#[pyclass]
#[derive(Clone)]
pub struct PyOrderBookSnapshot {
    #[pyo3(get)]
    pub symbol: String,
    #[pyo3(get)]
    pub timestamp: u64,
    #[pyo3(get)]
    pub bids: Vec<PyPriceLevel>,
    #[pyo3(get)]
    pub asks: Vec<PyPriceLevel>,
}

#[pyclass]
#[derive(Clone)]
pub struct PyPriceLevel {
    #[pyo3(get)]
    pub price: u64,
    #[pyo3(get)]
    pub visible_quantity: u64,
    #[pyo3(get)]
    pub hidden_quantity: u64,
    #[pyo3(get)]
    pub order_count: usize,
}

#[pymethods]
impl PyOrderBook {
    #[new]
    pub fn new(symbol: String) -> Self {
        Self {
            inner: OrderBook::new(&symbol),
        }
    }

    pub fn add_limit_order(
        &self,
        price: u64,
        quantity: u64,
        side: &str,
        time_in_force: &str,
    ) -> PyResult<String> {
        let side: Side = side.parse().map_err(|e: PriceLevelError| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        })?;
        let tif: TimeInForce = time_in_force.parse().map_err(|e: PriceLevelError| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        })?;
        let id = OrderId(uuid::Uuid::new_v4());
        self.inner
            .add_limit_order(id, price, quantity, side, tif)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e:?}")))?;
        Ok(id.0.to_string())
    }

    pub fn add_iceberg_order(
        &self,
        price: u64,
        visible_quantity: u64,
        hidden_quantity: u64,
        side: &str,
        time_in_force: &str,
    ) -> PyResult<String> {
        let side: Side = side.parse().map_err(|e: PriceLevelError| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        })?;
        let tif: TimeInForce = time_in_force.parse().map_err(|e: PriceLevelError| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        })?;
        let id = OrderId(uuid::Uuid::new_v4());
        self.inner
            .add_iceberg_order(id, price, visible_quantity, hidden_quantity, side, tif)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e:?}")))?;
        Ok(id.0.to_string())
    }

    pub fn add_post_only_order(
        &self,
        price: u64,
        quantity: u64,
        side: &str,
        time_in_force: &str,
    ) -> PyResult<String> {
        let side: Side = side.parse().map_err(|e: PriceLevelError| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        })?;
        let tif: TimeInForce = time_in_force.parse().map_err(|e: PriceLevelError| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        })?;
        let id = OrderId(uuid::Uuid::new_v4());
        self.inner
            .add_post_only_order(id, price, quantity, side, tif)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e:?}")))?;
        Ok(id.0.to_string())
    }

    pub fn submit_market_order(&self, quantity: u64, side: &str) -> PyResult<PyMatchResult> {
        let side: Side = side.parse().map_err(|e: PriceLevelError| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        })?;
        let id = OrderId(uuid::Uuid::new_v4());
        
        let result = self.inner
            .submit_market_order(id, quantity, side)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e:?}")))?;

        let transactions = result.transactions
            .as_vec()
            .iter()
            .map(|tx| PyTransaction {
                taker_order_id: tx.taker_order_id.0.to_string(),
                maker_order_id: tx.maker_order_id.0.to_string(),
                price: tx.price,
                quantity: tx.quantity,
                taker_side: tx.taker_side.to_string(),
                timestamp: tx.timestamp,
            })
            .collect();

        let filled_order_ids = result.filled_order_ids.clone().into_iter().map(|id| id.0.to_string()).collect();

        Ok(PyMatchResult {
            order_id: result.order_id.0.to_string(),
            executed_quantity: result.executed_quantity(),
            remaining_quantity: result.remaining_quantity,
            is_complete: result.is_complete,
            transactions,
            filled_order_ids,
            executed_value: result.executed_value(),
            average_price: result.average_price(),
        })
    }

    pub fn cancel_order(&self, order_id: &str) -> PyResult<bool> {
        let id = uuid::Uuid::parse_str(order_id)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid order ID: {e}")))?;
        let order_id = OrderId(id);
        
        match self.inner.cancel_order(order_id) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e:?}"))),
        }
    }

    pub fn update_order(&self, order_id: &str, new_quantity: u64) -> PyResult<bool> {
        let id = uuid::Uuid::parse_str(order_id)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid order ID: {e}")))?;
        let order_id = OrderId(id);
        
        let update = OrderUpdate::UpdateQuantity {
            order_id,
            new_quantity,
        };
        
        match self.inner.update_order(update) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e:?}"))),
        }
    }

    pub fn get_order(&self, order_id: &str) -> PyResult<Option<String>> {
        let id = uuid::Uuid::parse_str(order_id)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid order ID: {e}")))?;
        let order_id = OrderId(id);
        
        match self.inner.get_order(order_id) {
            Some(order) => Ok(Some(format!("Order: id={}, price={}, quantity={}, side={:?}", 
                order.id().0, order.price(), order.visible_quantity(), order.side()))),
            None => Ok(None),
        }
    }

    pub fn best_bid(&self) -> Option<u64> {
        self.inner.best_bid()
    }

    pub fn best_ask(&self) -> Option<u64> {
        self.inner.best_ask()
    }

    pub fn spread(&self) -> Option<u64> {
        self.inner.spread()
    }

    pub fn mid_price(&self) -> Option<f64> {
        self.inner.mid_price()
    }

    pub fn last_trade_price(&self) -> Option<u64> {
        self.inner.last_trade_price()
    }

    pub fn symbol(&self) -> String {
        self.inner.symbol().to_string()
    }

    pub fn total_orders(&self) -> usize {
        self.inner.get_all_orders().len()
    }

    pub fn create_snapshot(&self, depth: usize) -> PyOrderBookSnapshot {
        let snapshot = self.inner.create_snapshot(depth);
        
        let bids = snapshot.bids.into_iter().map(|level| PyPriceLevel {
            price: level.price,
            visible_quantity: level.visible_quantity,
            hidden_quantity: level.hidden_quantity,
            order_count: level.order_count,
        }).collect();
        
        let asks = snapshot.asks.into_iter().map(|level| PyPriceLevel {
            price: level.price,
            visible_quantity: level.visible_quantity,
            hidden_quantity: level.hidden_quantity,
            order_count: level.order_count,
        }).collect();
        
        PyOrderBookSnapshot {
            symbol: snapshot.symbol,
            timestamp: snapshot.timestamp,
            bids,
            asks,
        }
    }

    pub fn get_orders_at_price(&self, price: u64, side: &str) -> PyResult<Vec<String>> {
        let side: Side = side.parse().map_err(|e: PriceLevelError| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        })?;
        
        let orders = self.inner.get_orders_at_price(price, side);
        let order_strings = orders.into_iter().map(|order| {
            format!("Order: id={}, price={}, quantity={}", 
                order.id().0, order.price(), order.visible_quantity())
        }).collect();
        
        Ok(order_strings)
    }

    pub fn get_volume_by_price(&self) -> Vec<String> {
        let (bid_volumes, ask_volumes) = self.inner.get_volume_by_price();
        let mut result = Vec::new();
        
        for (price, volume) in bid_volumes {
            result.push(format!("Bid: price={}, volume={}", price, volume));
        }
        
        for (price, volume) in ask_volumes {
            result.push(format!("Ask: price={}, volume={}", price, volume));
        }
        
        result
    }

    pub fn set_market_close_timestamp(&self, timestamp: u64) {
        self.inner.set_market_close_timestamp(timestamp);
    }
}

#[pymodule]
#[allow(deprecated)]
pub fn orderbook_rs(_py: Python<'_>, m: &pyo3::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyOrderBook>()?;
    m.add_class::<PyMatchResult>()?;
    m.add_class::<PyTransaction>()?;
    m.add_class::<PyOrderBookSnapshot>()?;
    m.add_class::<PyPriceLevel>()?;
    Ok(())
}