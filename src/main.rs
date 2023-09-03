// use std::io::{Read, Write};
// use std::process::{Command, Stdio};

// fn main() {
//     let mut stockfish = Command::new("cmd")
//         .current_dir("./src")
//         .args(["/C", "stockfish.exe"])
//         .stdin(Stdio::piped())
//         .stdout(Stdio::piped())
//         .spawn()
//         .expect("Failed to access stockfish program");

//     if let Some(mut stdin) = stockfish.stdin.take() {
//         let input = format!(
//             "position {}\ngo depth {}",
//             "startpos",
//             &10.to_string()
//         );

//         stdin.write_all(input.as_bytes()).expect("Failed to input stockfish");
//     }

//     if let Some(mut stdout) = stockfish.stdout.take() {
//         let mut output = String::new();
//         stdout.read_to_string(&mut output).expect("Failed to get the output stockfish");

//         println!("Stockfish response:\n{:?}", output);
//     }
// }

use stockfish_rust::Stockfish;

fn main() {
    let stockfish = Stockfish::new("stockfish.exe".to_string());
    let result = stockfish.go_depth(1);
    match result {
        Some(output) => {
            println!("{}", output.len());
            println!("{}", output);
        }
        None => print!("Something doesn't works"),
    }
}
