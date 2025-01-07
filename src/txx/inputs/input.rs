use super::errors::ReadInputsLengthError;
use crate::txx;

pub type InputsLength = u64;

pub const INPUT_LENGTH_POSITION: usize = 4;

pub fn length(tx_bytes: &txx::TransactionData) -> Result<InputsLength, ReadInputsLengthError> {
    let size = tx_bytes.len();
    if size < INPUT_LENGTH_POSITION + 1 {
        return Ok(0);
    }

    let (compact_size, remaining) = (
        tx_bytes[INPUT_LENGTH_POSITION],
        &tx_bytes[INPUT_LENGTH_POSITION + 1..],
    );

    // Ensure minimum length for the remaining data
    let ensure_min = |min: usize| -> Result<(), ReadInputsLengthError> {
        let size = remaining.len();
        if size < min {
            return Err(ReadInputsLengthError::InsufficientData { length: size });
        }
        Ok(())
    };

    match compact_size {
        0..=252 => {
            // Return the length directly if it's a compact size value
            Ok(compact_size as u64)
        }

        253 => {
            const SIZE: usize = 2;

            ensure_min(SIZE)?;
            let buffer: [u8; SIZE] = remaining[0..SIZE]
                .try_into()
                .map_err(|_| ReadInputsLengthError::DataCorruption)?;

            Ok(u16::from_be_bytes(buffer) as u64)
        }

        254 => {
            const SIZE: usize = 4;

            ensure_min(SIZE)?;
            let buffer: [u8; SIZE] = remaining[0..SIZE]
                .try_into()
                .map_err(|_| ReadInputsLengthError::DataCorruption)?;

            Ok(u32::from_be_bytes(buffer) as u64)
        }

        255 => {
            const SIZE: usize = 8;

            ensure_min(SIZE)?;
            let buffer: [u8; SIZE] = remaining[0..SIZE]
                .try_into()
                .map_err(|_| ReadInputsLengthError::DataCorruption)?;

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
            expected: Result<InputsLength, ReadInputsLengthError>,
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
                expected: Err(ReadInputsLengthError::InsufficientData { length: 0 }),
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
