#[cfg(test)]
mod tests {
    use crate::JsOrderBook;

    #[test]
    fn js_orderbook_add_limit() {
        let ob = JsOrderBook::new("TEST".to_string());
        assert_eq!(ob.best_bid(), None);
        ob.add_limit_order(100, 1, "BUY".into(), "Gtc".into())
            .unwrap();
        assert_eq!(ob.best_bid(), Some(100));
    }
}
