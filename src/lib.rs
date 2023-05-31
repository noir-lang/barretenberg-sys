// Suppress the flurry of warnings caused by using "C" naming conventions
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate link_cplusplus;

// This matches bindgen::Builder output
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod blake2s;
pub mod composer;
pub mod pedersen;
pub mod pippenger;
pub mod schnorr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn pedersen() {
        let input = vec![0; 64];
        blake2s::hash_to_field(&input);

        let f_zero = [0_u8; 32];
        let mut f_one = [0_u8; 32];
        f_one[31] = 1;
        let got = pedersen::compress_native(&f_zero, &f_one);
        assert_eq!(
            "0c5e1ddecd49de44ed5e5798d3f6fb7c71fe3d37f5bee8664cf88a445b5ba0af",
            hex::encode(got)
        );
    }
}
