#[cfg(test)]
mod tests {
    use crate::bindings::node::JsOrderBook;

    #[test]
    fn basic_node_binding() {
        let book = JsOrderBook::new("BTC".to_string());
        assert_eq!(book.best_bid(), None);
        let _id = book
            .add_limit_order(1000, 1, "BUY".to_string(), "GTC".to_string())
            .unwrap();
        assert_eq!(book.best_bid(), Some(1000));
    }
}
