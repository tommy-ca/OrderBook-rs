use crate::orderbook::OrderBook;
use napi_derive::napi;
use napi::{Error, Result};
use pricelevel::{OrderId, PriceLevelError, Side, TimeInForce};

#[napi]
pub struct JsOrderBook {
    inner: OrderBook,
}

#[napi]
impl JsOrderBook {
    #[napi(constructor)]
    pub fn new(symbol: String) -> Self {
        Self {
            inner: OrderBook::new(&symbol),
        }
    }

    #[napi]
    pub fn add_limit_order(
        &self,
        price: f64,
        quantity: f64,
        side: String,
        time_in_force: String,
    ) -> Result<String> {
        let side: Side = side
            .parse()
            .map_err(|e: PriceLevelError| Error::from_reason(e.to_string()))?;
        let tif: TimeInForce = time_in_force
            .parse()
            .map_err(|e: PriceLevelError| Error::from_reason(e.to_string()))?;
        let id = OrderId(uuid::Uuid::new_v4());
        self.inner
            .add_limit_order(id, price as u64, quantity as u64, side, tif)
            .map_err(|e| Error::from_reason(format!("{e:?}")))?;
        Ok(id.0.to_string())
    }

    #[napi]
    pub fn best_bid(&self) -> Option<f64> {
        self.inner.best_bid().map(|v| v as f64)
    }

    #[napi]
    pub fn best_ask(&self) -> Option<f64> {
        self.inner.best_ask().map(|v| v as f64)
    }

    #[napi]
    pub fn symbol(&self) -> String {
        self.inner.symbol().to_string()
    }
}

#[napi]
pub fn hello() -> String {
    "Hello from OrderBook-rs!".to_string()
}