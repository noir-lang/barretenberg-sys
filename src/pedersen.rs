use crate::*;
use std::convert::TryInto;

pub fn compress_native(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    // unsafe { pedersen__init() };
    let mut result = [0_u8; 32];

    unsafe {
        pedersen_plookup_compress_fields(
            left.as_ptr() as *const u8,
            right.as_ptr() as *const u8,
            result.as_mut_ptr(),
        );
    }
    result
}

pub fn compress_many(inputs: &[[u8; 32]]) -> [u8; 32] {
    // unsafe { pedersen__init() };
    //convert inputs into one buffer: length + data
    let mut buffer = Vec::new();
    let witness_len = inputs.len() as u32;
    buffer.extend_from_slice(&witness_len.to_be_bytes());
    for assignment in inputs {
        buffer.extend_from_slice(assignment);
    }

    let mut result = [0_u8; 32];
    unsafe {
        pedersen_plookup_compress(buffer.as_ptr() as *const u8, result.as_mut_ptr());
    }
    result
}

pub fn encrypt(inputs_buffer: &[[u8; 32]]) -> ([u8; 32], [u8; 32]) {
    let mut buffer = Vec::new();
    let buffer_len = inputs_buffer.len() as u32;
    let mut result = [0_u8; 64];
    buffer.extend_from_slice(&buffer_len.to_be_bytes());
    for e in inputs_buffer {
        buffer.extend_from_slice(e);
    }

    unsafe {
        pedersen_plookup_commit(buffer.as_ptr() as *const u8, result.as_mut_ptr());
    }
    let s: [u8; 32] = (result[0..32]).try_into().unwrap();
    let e: [u8; 32] = (result[32..64]).try_into().unwrap();
    (s, e)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_interop() {
        // Expected values were taken from Barretenberg by running `crypto::pedersen::compress_native`
        // printing the result in hex to `std::cout` and copying
        struct Test<'a> {
            input_left: [u8; 32],
            input_right: [u8; 32],
            expected_hex: &'a str,
        }
        let f_zero = [0_u8; 32];
        let mut f_one = [0_u8; 32];
        f_one[31] = 1;

        let tests = vec![
            Test {
                input_left: f_zero,
                input_right: f_one,
                expected_hex: "0c5e1ddecd49de44ed5e5798d3f6fb7c71fe3d37f5bee8664cf88a445b5ba0af",
            },
            Test {
                input_left: f_one,
                input_right: f_one,
                expected_hex: "0e1793a0c122887bcb53c84776f4704c26bc093b25eaa9c7847a672c65e314ae",
            },
            Test {
                input_left: f_one,
                input_right: f_zero,
                expected_hex: "0c93b3f27730b2e331e634af15bc9d5a769688921f30b36ca926b35a96b3306c",
            },
        ];

        for test in tests {
            let got = compress_native(&test.input_left, &test.input_right);
            let many_intputs: Vec<[u8; 32]> = vec![test.input_left, test.input_right];
            let got_many = compress_many(&many_intputs);
            assert_eq!(hex::encode(got), test.expected_hex);
            assert_eq!(got, got_many);
        }
    }

    #[test]
    fn pedersen_hash_to_point() {
        let f_zero = [0_u8; 32];
        let mut f_one = [0_u8; 32];
        f_one[31] = 1;
        let inputs: Vec<[u8; 32]> = vec![f_zero, f_one];

        let (x, y) = encrypt(&inputs);
        let expected_x = "0c5e1ddecd49de44ed5e5798d3f6fb7c71fe3d37f5bee8664cf88a445b5ba0af";
        let expected_y = "230294a041e26fe80b827c2ef5cb8784642bbaa83842da2714d62b1f3c4f9752";
        assert_eq!(expected_x, hex::encode(x));
        assert_eq!(expected_y, hex::encode(y));
    }
}
