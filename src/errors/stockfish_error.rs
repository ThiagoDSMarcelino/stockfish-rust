use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum StockfishError {
    LoadEngine { path: PathBuf },
    Write,
    Read,
    StdIn,
    StdOut,
    GetSubString,
}

impl core::fmt::Display for StockfishError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let error_text = match self {
            StockfishError::LoadEngine { path } => {
                format!("Error loading engine. Path: {:?}", path.to_string_lossy())
            }
            StockfishError::Write => String::from("Error writing"),
            StockfishError::Read => String::from("Error reading"),
            StockfishError::StdIn => String::from("Error in standard input"),
            StockfishError::StdOut => String::from("Error in standard output"),
            StockfishError::GetSubString => String::from("Error getting substring"),
        };

        write!(f, "{}", error_text)
    }
}

impl std::error::Error for StockfishError {}
