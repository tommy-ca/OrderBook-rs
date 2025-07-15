#[cfg(test)]
mod tests {
    use crate::bindings::python::PyOrderBook;

    #[test]
    fn basic_python_binding() {
        let book = PyOrderBook::new("BTC".to_string());
        assert_eq!(book.best_bid(), None);
        let _id = book.add_limit_order(1000, 1, "BUY", "GTC").unwrap();
        assert_eq!(book.best_bid(), Some(1000));
    }
}
