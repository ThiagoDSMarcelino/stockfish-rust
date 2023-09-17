use stockfish_rust::Stockfish;

fn main() {
    let stockfish = Stockfish::new("src/stockfish");
    let result = stockfish.go_depth(1);
    match result {
        Ok(output) => {
            println!("Best move {}", output);
        }
        Err(error) => print!("Something doesn't works {:?}", error),
    }
}
