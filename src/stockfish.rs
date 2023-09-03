use std::io::{Read, Write};
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct Stockfish {
    dir: String,
    position: String,
}

impl Stockfish {
    pub fn new(dir: String) -> Self {
        Self {
            dir,
            position: "startpos".to_string(),
        }
    }

    pub fn go_depth(&self, depth: u8) -> Option<String> {
        let mut stockfish = Command::new("cmd")
            .current_dir("./src")
            .args(["/C", &self.dir])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to access stockfish program");

        match stockfish.stdin.take() {
            Some(mut stdin) => {
                let input = format!("position {}\ngo depth {}", "startpos", &depth.to_string());

                stdin
                    .write_all(input.as_bytes())
                    .expect("Failed to input stockfish");
            }
            None => return None,
        }

        stockfish.stdout.take().map(|mut stdout| {
            let mut output = String::new();
            stdout
                .read_to_string(&mut output)
                .expect("Failed to get the output stockfish");

            output
        })
    }

    pub fn set_position(&mut self, fen: String) {
        self.position = fen;
    }

    pub fn reset_position(&mut self) {
        self.position = "startpos".to_string();
    }
}
