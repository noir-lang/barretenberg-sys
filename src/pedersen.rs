use crate::*;
use std::convert::TryInto;
// TODO: Behaviour has changed since the old barretenberg
// TODO: so test vectors wont work.
//
// TODO: We should check what we need
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
                expected_hex: "11831f49876c313f2a9ec6d8d521c7ce0b6311c852117e340bfe27fd1ac096ef",
            },
            Test {
                input_left: f_one,
                input_right: f_one,
                expected_hex: "1044a769e185fcdf077c8289a6bf87c5c77ff9561cab69d39fadd90a07ee4af4",
            },
            Test {
                input_left: f_one,
                input_right: f_zero,
                expected_hex: "17d213c8fe83e89a2f3190933d437a3e231124e0383e6dc6a7b6e6358833e427",
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
        let expected_x = "11831f49876c313f2a9ec6d8d521c7ce0b6311c852117e340bfe27fd1ac096ef";
        let expected_y = "0ecf9d98be4597a88c46a7e0fa8836b57a7dcb41ee30f8d8787b11cc259c83fa";
        assert_eq!(expected_x, hex::encode(x));
        assert_eq!(expected_y, hex::encode(y));
    }
}
