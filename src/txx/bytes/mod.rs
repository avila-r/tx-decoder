use crate::txx::{
    inputs,
    inputs::{InputsLength, ReadInputsLengthError},
    version, ReadVersionError,
};

pub type TransactionData = Vec<u8>;

#[allow(dead_code)]
pub trait TransactionBytesTrait {
    fn length(&self) -> Result<InputsLength, ReadInputsLengthError>;
    fn version(&self) -> Result<u32, ReadVersionError>;
}

impl TransactionBytesTrait for TransactionData {
    fn length(&self) -> Result<InputsLength, ReadInputsLengthError> {
        inputs::length(self)
    }

    fn version(&self) -> Result<u32, ReadVersionError> {
        version::from(self)
    }
}
