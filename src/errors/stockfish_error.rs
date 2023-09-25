#[derive(Clone, Debug)]
pub enum StockfishError {
    LoadEngineError,
    WriteError,
    ReadError,
    StdInError,
    StdOutError,
    GetSubStringError
}

impl core::fmt::Display for StockfishError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for StockfishError {}