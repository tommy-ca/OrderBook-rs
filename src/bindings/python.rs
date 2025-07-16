#![allow(unsafe_op_in_unsafe_fn)]

use crate::orderbook::OrderBook;
use pricelevel::{OrderId, PriceLevelError, Side, TimeInForce};
use pyo3::prelude::*;

#[pyclass]
pub struct PyOrderBook {
    inner: OrderBook,
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

    pub fn best_bid(&self) -> Option<u64> {
        self.inner.best_bid()
    }

    pub fn best_ask(&self) -> Option<u64> {
        self.inner.best_ask()
    }
}

#[pymodule]
#[allow(deprecated)]
pub fn orderbook_rs(_py: Python<'_>, m: &pyo3::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyOrderBook>()?;
    Ok(())
}
