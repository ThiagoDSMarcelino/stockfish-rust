#[cfg(test)]
mod tests {
    use stockfish_rust::Stockfish;

    #[test]
    fn go_depth() {
        let stockfish = Stockfish::new("src/stockfish");
        let result = stockfish.go_depth(1).unwrap();

        assert_eq!(result, "a2a3");
    }
}
