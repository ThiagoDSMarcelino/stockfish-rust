use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct Stockfish {
    dir: PathBuf,
    position: String,
}

impl Stockfish {
    pub fn new(dir: &'static str) -> Self {
        Self {
            dir: PathBuf::from(dir),
            position: String::from("startpos"),
        }
    }

    pub fn go_depth(&self, depth: u8) -> Result<String, String> {
        let mut stockfish = Command::new(&self.dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|err| format!("Failed to access Stockfish program: {}", err))?;

        let mut stdin = stockfish.stdin.take().ok_or("Error in input".to_string())?;
        let mut stdout = stockfish
            .stdout
            .take()
            .ok_or("Error in output".to_string())?;

        let input = format!(
            "uci\nisready\nucinewgame\nposition {}\ngo depth {}\nquit\n",
            self.position,
            &depth.to_string()
        );
        stdin
            .write_all(input.as_bytes())
            .map_err(|err| format!("Failed to input Stockfish program: {}", err))?;

        let mut output = String::new();
        stdout
            .read_to_string(&mut output)
            .map_err(|err| format!("Failed to read Stockfish output: {}", err))?;

        let result = self
            .get_substring(&output, "bestmove ")
            .ok_or("Internal error")?
            .trim();

        Ok(String::from(result))
    }

    pub fn set_position(&mut self, fen: &str) {
        self.position = String::from(fen);
    }

    pub fn reset_position(&mut self) {
        self.position = String::from("startpos");
    }

    fn get_substring<'a>(&self, data: &'a str, separator: &'a str) -> Option<&'a str> {
        if let Some(index) = data.find(separator) {
            Some(&data[index + separator.len()..])
        } else {
            None
        }
    }
}
