use crate::orderbook::OrderBook;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use pricelevel::{OrderId, Side, TimeInForce};

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
            .parse::<Side>()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let tif: TimeInForce = time_in_force
            .parse::<TimeInForce>()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let id = OrderId(uuid::Uuid::new_v4());
        self.inner
            .add_limit_order(id, price as u64, quantity as u64, side, tif)
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
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

#[cfg(all(test, feature = "nodejs"))]
mod tests {
    use super::*;

    #[test]
    fn node_binding_basic() {
        let ob = JsOrderBook::new("BTCUSD".to_string());
        let _id = ob
            .add_limit_order(1000, 1, "Buy".to_string(), "Gtc".to_string())
            .expect("order should be added");
        assert_eq!(ob.best_bid(), Some(1000));
        assert!(ob.best_ask().is_none());
    }
}
