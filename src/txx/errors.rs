#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionHexError {
    /// Error decoding the hexadecimal string.
    InvalidHexData(String),
}

#[cfg(feature = "std")]
impl std::error::Error for TransactionHexError {}

impl std::fmt::Display for TransactionHexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionHexError::InvalidHexData(err) => {
                write!(f, "Failed to decode transaction's data: {}", err)
            }
        }
    }
}
