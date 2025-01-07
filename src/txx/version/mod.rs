pub fn from(tx_bytes: &Vec<u8>) -> Result<u32, ReadVersionError> {
    let size = tx_bytes.len();
    if size < 4 {
        return Err(ReadVersionError::InsufficientData { length: size });
    }

    let version_bytes = <[u8; 4]>::try_from(&tx_bytes[0..=3])
        .map_err(|err| ReadVersionError::InvalidVersionBytes(err.to_string()))?;

    let version = u32::from_le_bytes(version_bytes);

    Ok(version)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadVersionError {
    /// Insufficient data to extract the version.
    InsufficientData { length: usize },

    /// Failed to convert the version bytes into a u32.
    InvalidVersionBytes(String),
}

#[cfg(feature = "std")]
impl std::error::Error for ReadVersionError {}

impl std::fmt::Display for ReadVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadVersionError::InsufficientData { length } => write!(
                f,
                "Transaction data is too short to extract the version (length: {})",
                length
            ),
            ReadVersionError::InvalidVersionBytes(err) => write!(
                f,
                "Failed to decode version from transaction's bytes: {}",
                err
            ),
        }
    }
}
