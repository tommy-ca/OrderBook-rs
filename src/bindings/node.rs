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
        price: i64,
        quantity: i64,
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
    pub fn best_bid(&self) -> Option<i64> {
        self.inner.best_bid().map(|v| v as i64)
    }

    #[napi]
    pub fn best_ask(&self) -> Option<i64> {
        self.inner.best_ask().map(|v| v as i64)
    }
}

// Add a simple standalone function to help with module registration
#[napi]
pub fn hello() -> String {
    "Hello from OrderBook-rs NAPI!".to_string()
}
