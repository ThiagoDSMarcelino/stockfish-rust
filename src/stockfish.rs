use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::errors::StockfishError;
use crate::string_useful::Useful;

#[derive(Debug)]
pub struct Stockfish {
    dir: PathBuf,
    position: String,
}

impl Stockfish {
    pub fn new(dir: &str) -> Self {
        Self {
            dir: PathBuf::from(dir),
            position: String::from("startpos"),
        }
    }

    pub fn go_depth(&self, depth: u8) -> Result<String, StockfishError> {
        let mut stockfish = Command::new(&self.dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|_| StockfishError::LoadEngine {
                path: self.dir.clone(),
            })?;

        let mut stdin = stockfish.stdin.take().ok_or(StockfishError::StdIn)?;
        let mut stdout = stockfish.stdout.take().ok_or(StockfishError::StdOut)?;

        let input = format!(
            "uci\nisready\nucinewgame\nposition {}\ngo depth {}\nquit\n",
            self.position,
            &depth.to_string()
        );

        stdin
            .write_all(input.as_bytes())
            .map_err(|_| StockfishError::Write)?;

        let mut output = String::new();
        stdout
            .read_to_string(&mut output)
            .map_err(|_| StockfishError::Read)?;

        let result = output
            .get_substring("bestmove ")
            .ok_or(StockfishError::GetSubString)?
            .trim();

        Ok(String::from(result))
    }

    pub fn set_position(&mut self, fen: &str) {
        self.position = String::from(fen);
    }

    pub fn reset_position(&mut self) {
        self.position = String::from("startpos");
    }
}
