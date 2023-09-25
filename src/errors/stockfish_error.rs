#[derive(Clone, Debug)]
pub enum StockfishError {
    LoadEngine,
    Write,
    Read,
    StdIn,
    StdOut,
    GetSubString,
}

impl core::fmt::Display for StockfishError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for StockfishError {}
