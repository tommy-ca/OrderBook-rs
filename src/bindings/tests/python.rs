#[cfg(test)]
mod tests {
    use crate::PyOrderBook;
    use pyo3::Python;

    #[test]
    fn py_orderbook_add_limit() {
        Python::with_gil(|_| {
            let ob = PyOrderBook::new("TEST".to_string());
            assert_eq!(ob.best_bid(), None);
            ob.add_limit_order(100, 1, "BUY", "Gtc").unwrap();
            assert_eq!(ob.best_bid(), Some(100));
        });
    }
}
