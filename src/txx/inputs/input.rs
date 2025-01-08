use super::{errors::ReadCompactSizeError, ReadInputsError, ReadInputsLengthError};
use crate::txx::{self, TransactionBytesTrait};

pub struct Input {
    _txid: [u8; 32],
    _vout: [u8; 4],
    _script_sig: Vec<u8>,
    _sequence: [u8; 4],
}

pub type InputsLength = u64;

pub const INPUT_LENGTH_POSITION: usize = 4;
pub const INPUT_TXID_POSITION: usize = 5;
pub const INPUT_VOUT_POSITION: usize = INPUT_TXID_POSITION + 32 + 1;
pub const INPUT_SCRIPT_SIG_POSITION: usize = INPUT_VOUT_POSITION + 1;

pub fn from(tx_bytes: &txx::TransactionData) -> Result<Vec<Input>, ReadInputsError> {
    let length: InputsLength = match tx_bytes.length() {
        Ok(length) => length,
        Err(err) => return Err(ReadInputsError::InvalidData(err)),
    };

    let mut inputs = Vec::new();
    for _ in 0..length {
        let txid: [u8; 32] = tx_bytes[INPUT_TXID_POSITION..(INPUT_TXID_POSITION + 32)]
            .try_into()
            .map_err(|_| ReadInputsError::DataCorruption)?;

        let vout: [u8; 4] = tx_bytes[INPUT_VOUT_POSITION..4]
            .try_into()
            .map_err(|_| ReadInputsError::DataCorruption)?;

        let (script_sig_head, script_sig_tail) = (
            tx_bytes[INPUT_SCRIPT_SIG_POSITION],
            &tx_bytes[INPUT_SCRIPT_SIG_POSITION + 1..],
        );

        let script_sig_size = match read_compact_size(script_sig_head, script_sig_tail) {
            Ok(size) => size,
            Err(_) => return Err(ReadInputsError::DataCorruption),
        };

        let script_sig: Vec<u8> =
            tx_bytes[INPUT_SCRIPT_SIG_POSITION..(script_sig_size as usize)].to_vec();

        let sequence: [u8; 4] = tx_bytes[(INPUT_SCRIPT_SIG_POSITION + script_sig_size as usize)..4]
            .try_into()
            .map_err(|_| ReadInputsError::DataCorruption)?;

        inputs.push(Input {
            _txid: txid,
            _vout: vout,
            _script_sig: script_sig,
            _sequence: sequence,
        });
    }

    Ok(vec![])
}

pub fn length(tx_bytes: &txx::TransactionData) -> Result<InputsLength, ReadInputsLengthError> {
    let size = tx_bytes.len();
    if size < INPUT_LENGTH_POSITION + 1 {
        return Ok(0);
    }

    let (compact_size, remaining) = (
        tx_bytes[INPUT_LENGTH_POSITION],
        &tx_bytes[INPUT_LENGTH_POSITION + 1..],
    );

    read_compact_size(compact_size, remaining)
}

fn read_compact_size(head: u8, tail: &[u8]) -> Result<u64, ReadCompactSizeError> {
    // Ensure minimum length for the remaining data
    let ensure_min = |min: usize| -> Result<(), ReadCompactSizeError> {
        let size = tail.len();
        if size < min {
            return Err(ReadCompactSizeError::InsufficientData { length: size });
        }
        Ok(())
    };

    match head {
        0..=252 => {
            // Return the length directly if it's a compact size value
            Ok(head as u64)
        }

        253 => {
            const SIZE: usize = 2;

            ensure_min(SIZE)?;
            let buffer: [u8; SIZE] = tail[0..SIZE]
                .try_into()
                .map_err(|_| ReadCompactSizeError::DataCorruption)?;

            Ok(u16::from_be_bytes(buffer) as u64)
        }

        254 => {
            const SIZE: usize = 4;

            ensure_min(SIZE)?;
            let buffer: [u8; SIZE] = tail[0..SIZE]
                .try_into()
                .map_err(|_| ReadCompactSizeError::DataCorruption)?;

            Ok(u32::from_be_bytes(buffer) as u64)
        }

        255 => {
            const SIZE: usize = 8;

            ensure_min(SIZE)?;
            let buffer: [u8; SIZE] = tail[0..SIZE]
                .try_into()
                .map_err(|_| ReadCompactSizeError::DataCorruption)?;

            Ok(u64::from_be_bytes(buffer))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tx_length() {
        struct TestCase {
            input: Vec<u8>,
            expected: Result<InputsLength, ReadCompactSizeError>,
        }

        let test_cases = vec![
            // Test with inputs of varying sizes
            TestCase {
                input: vec![0, 0, 0, 0, 3], // compact size is 3
                expected: Ok(3),
            },
            TestCase {
                input: vec![0, 0, 0, 0, 253, 0, 1], // compact size is 253, next is 1 (u16)
                expected: Ok(1),
            },
            TestCase {
                input: vec![0, 0, 0, 0, 254, 0, 0, 0, 1], // compact size is 254, next is 1 (u32)
                expected: Ok(1),
            },
            TestCase {
                input: vec![0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 1], // compact size is 255, followed by 8 bytes for u64 (value: 1)
                expected: Ok(1),
            },
            TestCase {
                input: vec![0, 0, 0, 0, 253], // insufficient data for 253
                expected: Err(ReadCompactSizeError::InsufficientData { length: 0 }),
            },
            TestCase {
                input: vec![0, 0, 0, 0], // not enough data to extract anything
                expected: Ok(0),
            },
        ];

        for case in test_cases {
            let result = length(&case.input);
            assert_eq! {result, case.expected, "Failed for input: {:?}", case.input};
        }
    }
}
