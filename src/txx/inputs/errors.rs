#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadInputsLengthError {
    /// Insufficient data to extract the input count.
    InsufficientData { length: usize },

    /// Corrupted data when parsing bytes.
    DataCorruption,
}

#[cfg(feature = "std")]
impl std::error::Error for ReadInputsLengthError {}

impl std::fmt::Display for ReadInputsLengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadInputsLengthError::InsufficientData { length } => write! {
                f,
                "Transaction data is too short to extract the inputs' length (length: {}, should be at least {})",
                length, super::INPUT_LENGTH_POSITION + 1
            },
            ReadInputsLengthError::DataCorruption => write! {
                f,
                "Transaction data is corrupted or contains invalid byte sequences."
            },
        }
    }
}
