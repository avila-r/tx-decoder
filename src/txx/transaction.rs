use super::{bytes, inputs, version};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub version: u32,
    pub inputs_count: inputs::InputsLength,
    pub inputs: Vec<inputs::Input>,
}

impl Transaction {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

pub fn from(tx_bytes: &bytes::TransactionData) -> Result<Transaction, Box<dyn std::error::Error>> {
    let (version, inputs_count, inputs) = (
        version::from(&tx_bytes)?,
        inputs::length(&tx_bytes)?,
        inputs::from(&tx_bytes)?,
    );

    Ok(Transaction {
        version,
        inputs_count,
        inputs,
    })
}

pub fn from_hex(hex: &str) -> Result<Transaction, Box<dyn std::error::Error>> {
    Ok(from(&super::hex::decode(hex)?)?)
}
