use crate::orderbook::OrderBook;
use napi_derive::napi;
use napi::{Error, Result};
use pricelevel::{OrderId, OrderUpdate, PriceLevelError, Side, TimeInForce};

#[napi]
pub struct JsOrderBook {
    inner: OrderBook,
}

#[napi(object)]
pub struct JsMatchResult {
    pub order_id: String,
    pub executed_quantity: f64,
    pub remaining_quantity: f64,
    pub is_complete: bool,
    pub transactions: Vec<JsTransaction>,
    pub filled_order_ids: Vec<String>,
    pub executed_value: f64,
    pub average_price: Option<f64>,
}

#[napi(object)]
pub struct JsTransaction {
    pub taker_order_id: String,
    pub maker_order_id: String,
    pub price: f64,
    pub quantity: f64,
    pub taker_side: String,
    pub timestamp: f64,
}

#[napi(object)]
pub struct JsOrderBookSnapshot {
    pub symbol: String,
    pub timestamp: f64,
    pub bids: Vec<JsPriceLevel>,
    pub asks: Vec<JsPriceLevel>,
}

#[napi(object)]
pub struct JsPriceLevel {
    pub price: f64,
    pub visible_quantity: f64,
    pub hidden_quantity: f64,
    pub order_count: i32,
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
    pub fn add_iceberg_order(
        &self,
        price: f64,
        visible_quantity: f64,
        hidden_quantity: f64,
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
            .add_iceberg_order(id, price as u64, visible_quantity as u64, hidden_quantity as u64, side, tif)
            .map_err(|e| Error::from_reason(format!("{e:?}")))?;
        Ok(id.0.to_string())
    }

    #[napi]
    pub fn add_post_only_order(
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
            .add_post_only_order(id, price as u64, quantity as u64, side, tif)
            .map_err(|e| Error::from_reason(format!("{e:?}")))?;
        Ok(id.0.to_string())
    }

    #[napi]
    pub fn submit_market_order(&self, quantity: f64, side: String) -> Result<JsMatchResult> {
        let side: Side = side
            .parse()
            .map_err(|e: PriceLevelError| Error::from_reason(e.to_string()))?;
        let id = OrderId(uuid::Uuid::new_v4());
        
        let result = self.inner
            .submit_market_order(id, quantity as u64, side)
            .map_err(|e| Error::from_reason(format!("{e:?}")))?;

        let transactions = result.transactions
            .as_vec()
            .iter()
            .map(|tx| JsTransaction {
                taker_order_id: tx.taker_order_id.0.to_string(),
                maker_order_id: tx.maker_order_id.0.to_string(),
                price: tx.price as f64,
                quantity: tx.quantity as f64,
                taker_side: tx.taker_side.to_string(),
                timestamp: tx.timestamp as f64,
            })
            .collect();

        let filled_order_ids = result.filled_order_ids.clone().into_iter().map(|id| id.0.to_string()).collect();

        Ok(JsMatchResult {
            order_id: result.order_id.0.to_string(),
            executed_quantity: result.executed_quantity() as f64,
            remaining_quantity: result.remaining_quantity as f64,
            is_complete: result.is_complete,
            transactions,
            filled_order_ids,
            executed_value: result.executed_value() as f64,
            average_price: result.average_price(),
        })
    }

    #[napi]
    pub fn cancel_order(&self, order_id: String) -> Result<bool> {
        let id = uuid::Uuid::parse_str(&order_id)
            .map_err(|e| Error::from_reason(format!("Invalid order ID: {e}")))?;
        let order_id = OrderId(id);
        
        match self.inner.cancel_order(order_id) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(Error::from_reason(format!("{e:?}"))),
        }
    }

    #[napi]
    pub fn update_order(&self, order_id: String, new_quantity: f64) -> Result<bool> {
        let id = uuid::Uuid::parse_str(&order_id)
            .map_err(|e| Error::from_reason(format!("Invalid order ID: {e}")))?;
        let order_id = OrderId(id);
        
        let update = OrderUpdate::UpdateQuantity {
            order_id,
            new_quantity: new_quantity as u64,
        };
        
        match self.inner.update_order(update) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(Error::from_reason(format!("{e:?}"))),
        }
    }

    #[napi]
    pub fn get_order(&self, order_id: String) -> Result<Option<String>> {
        let id = uuid::Uuid::parse_str(&order_id)
            .map_err(|e| Error::from_reason(format!("Invalid order ID: {e}")))?;
        let order_id = OrderId(id);
        
        match self.inner.get_order(order_id) {
            Some(order) => Ok(Some(format!("Order: id={}, price={}, quantity={}, side={:?}", 
                order.id().0, order.price(), order.visible_quantity(), order.side()))),
            None => Ok(None),
        }
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
    pub fn spread(&self) -> Option<f64> {
        self.inner.spread().map(|v| v as f64)
    }

    #[napi]
    pub fn mid_price(&self) -> Option<f64> {
        self.inner.mid_price()
    }

    #[napi]
    pub fn last_trade_price(&self) -> Option<f64> {
        self.inner.last_trade_price().map(|v| v as f64)
    }

    #[napi]
    pub fn symbol(&self) -> String {
        self.inner.symbol().to_string()
    }

    #[napi]
    pub fn total_orders(&self) -> i32 {
        self.inner.get_all_orders().len() as i32
    }

    #[napi]
    pub fn create_snapshot(&self, depth: i32) -> JsOrderBookSnapshot {
        let snapshot = self.inner.create_snapshot(depth as usize);
        
        let bids = snapshot.bids.into_iter().map(|level| JsPriceLevel {
            price: level.price as f64,
            visible_quantity: level.visible_quantity as f64,
            hidden_quantity: level.hidden_quantity as f64,
            order_count: level.order_count as i32,
        }).collect();
        
        let asks = snapshot.asks.into_iter().map(|level| JsPriceLevel {
            price: level.price as f64,
            visible_quantity: level.visible_quantity as f64,
            hidden_quantity: level.hidden_quantity as f64,
            order_count: level.order_count as i32,
        }).collect();
        
        JsOrderBookSnapshot {
            symbol: snapshot.symbol,
            timestamp: snapshot.timestamp as f64,
            bids,
            asks,
        }
    }

    #[napi]
    pub fn get_orders_at_price(&self, price: f64, side: String) -> Result<Vec<String>> {
        let side: Side = side
            .parse()
            .map_err(|e: PriceLevelError| Error::from_reason(e.to_string()))?;
        
        let orders = self.inner.get_orders_at_price(price as u64, side);
        let order_strings = orders.into_iter().map(|order| {
            format!("Order: id={}, price={}, quantity={}", 
                order.id().0, order.price(), order.visible_quantity())
        }).collect();
        
        Ok(order_strings)
    }

    #[napi]
    pub fn get_volume_by_price(&self) -> Result<Vec<String>> {
        let (bid_volumes, ask_volumes) = self.inner.get_volume_by_price();
        let mut result = Vec::new();
        
        for (price, volume) in bid_volumes {
            result.push(format!("Bid: price={}, volume={}", price, volume));
        }
        
        for (price, volume) in ask_volumes {
            result.push(format!("Ask: price={}, volume={}", price, volume));
        }
        
        Ok(result)
    }

    #[napi]
    pub fn set_market_close_timestamp(&self, timestamp: f64) {
        self.inner.set_market_close_timestamp(timestamp as u64);
    }
}

#[napi]
pub fn hello() -> String {
    "Hello from OrderBook-rs!".to_string()
}