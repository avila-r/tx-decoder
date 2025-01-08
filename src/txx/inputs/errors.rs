#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadInputsError {
    /// Invalid data.
    InvalidData(ReadCompactSizeError),

    /// Corrupted data when parsing bytes.
    DataCorruption,
}

#[cfg(feature = "std")]
impl std::error::Error for ReadInputsError {}

impl std::fmt::Display for ReadInputsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadInputsError::InvalidData(err) => write! {
                f,
                "Failed to define input's length: {}",
                err
            },
            ReadInputsError::DataCorruption => write! {
                f,
                "Transaction data is corrupted or contains invalid byte sequences."
            },
        }
    }
}

pub type ReadInputsLengthError = ReadCompactSizeError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadCompactSizeError {
    /// Insufficient data to extract the input count.
    InsufficientData { length: usize },

    /// Corrupted data when parsing bytes.
    DataCorruption,
}

#[cfg(feature = "std")]
impl std::error::Error for ReadCompactSizeError {}

impl std::fmt::Display for ReadCompactSizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadCompactSizeError::InsufficientData { length } => write! {
                f,
                "Transaction data is too short to extract the inputs' length (length: {}, should be at least {})",
                length, super::INPUT_LENGTH_POSITION + 1
            },
            ReadCompactSizeError::DataCorruption => write! {
                f,
                "Transaction data is corrupted or contains invalid byte sequences."
            },
        }
    }
}
