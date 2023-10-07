#[cfg(test)]
mod tests {
    use stockfish_rust::{Stockfish, StockfishError};

    #[test]
    fn go_depth() -> Result<(), StockfishError> {
        let stockfish = Stockfish::new("stockfish");
        let result = stockfish.go_depth(1)?;

        assert_eq!(result, "a2a3");

        Ok(())
    }
}
