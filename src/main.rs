use std::io::{Read, Result, Write};
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    let mut stockfish = Command::new("cmd")
        .args(["/C", "stockfish.exe"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = stockfish.stdin.take() {
        stdin.write_all(b"position startpos\n")?;
        stdin.write_all(b"go depth 10\n")?;
    }

    if let Some(mut stdout) = stockfish.stdout.take() {
        let mut output = Vec::new();
        stdout.read_to_end(&mut output)?;

        let response = String::from_utf8_lossy(&output);

        println!("Stockfish response:\n{:?}", response);
    }

    let status = stockfish.wait()?;
    println!("Status: {:?}", status);

    Ok(())
}
