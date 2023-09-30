# Stockfish with Rust

The objective of this project is to integrate the [Stockfish](https://stockfishchess.org) Chess Engine with Rust, that way making it easier to consume their features in projects.

![GitHub](https://img.shields.io/github/license/ThiagoDSMarcelino/stockfish-rust?color=blue)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/ThiagoDSMarcelino/stockfish-rust)

## Methods

### Go Depth

```Rust

fn go_depth() -> Result<(), StockfishError> {
    let stockfish = Stockfish::new("stockfish"); // Directory to Stockfish Engine in your computer
    let result = stockfish.go_depth(1)?;

    assert_eq!(result, "a2a3");

    Ok(())
}
```
