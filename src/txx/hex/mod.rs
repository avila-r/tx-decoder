use super::{errors, bytes::TransactionData};

pub fn decode(hex: &str) -> Result<TransactionData, errors::TransactionHexError> {
    let bytes = hex::decode(hex)
        .map_err(|err| errors::TransactionHexError::InvalidHexData(err.to_string()));

    bytes
}
