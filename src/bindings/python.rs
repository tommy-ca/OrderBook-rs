use crate::orderbook::OrderBook;
use pricelevel::{OrderId, Side, TimeInForce};
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
        let side: Side = side
            .parse::<Side>()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let tif: TimeInForce = time_in_force
            .parse::<TimeInForce>()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let id = OrderId(uuid::Uuid::new_v4());
        self.inner
            .add_limit_order(id, price, quantity, side, tif)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{:?}", e)))?;
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
pub fn orderbook_rs_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyOrderBook>()?;
    Ok(())
}

#[cfg(all(test, feature = "python"))]
mod tests {
    use super::*;

    #[test]
    fn python_binding_basic() {
        Python::with_gil(|_py| {
            let ob = PyOrderBook::new("BTCUSD".to_string());
            let _id = ob
                .add_limit_order(1000, 1, "Buy", "Gtc")
                .expect("order should be added");
            assert_eq!(ob.best_bid(), Some(1000));
            assert!(ob.best_ask().is_none());
        });
    }
}
