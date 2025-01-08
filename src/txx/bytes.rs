use crate::txx::{
    inputs,
    inputs::{Input, InputsLength, ReadCompactSizeError, ReadInputsError},
    version,
    version::ReadVersionError,
};

pub type TransactionData = Vec<u8>;

#[allow(dead_code)]
pub trait TransactionBytesTrait {
    fn version(&self) -> Result<u32, ReadVersionError>;
    fn length(&self) -> Result<InputsLength, ReadCompactSizeError>;
    fn inputs(&self) -> Result<Vec<Input>, ReadInputsError>;
}

impl TransactionBytesTrait for TransactionData {
    fn version(&self) -> Result<u32, ReadVersionError> {
        version::from(self)
    }

    fn length(&self) -> Result<InputsLength, ReadCompactSizeError> {
        inputs::length(self)
    }

    fn inputs(&self) -> Result<Vec<Input>, ReadInputsError> {
        inputs::from(self)
    }
}
