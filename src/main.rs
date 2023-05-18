// Suppress the flurry of warnings caused by using "C" naming conventions
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// This is not needed, if we are linking the wasm library
// extern crate link_cplusplus;

// This matches bindgen::Builder output
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
mod bindings;
use bindings::*;

pub mod blake2s;
pub mod pedersen;
// pub mod composer;
// pub mod pippenger;
// pub mod schnorr;

fn main() {
    let input = vec![0; 64];
    blake2s::hash_to_field(&input);

    let f_zero = [0_u8; 32];
    let mut f_one = [0_u8; 32];
    f_one[31] = 1;
    let got = pedersen::compress_native(&f_zero, &f_one);
    println!("{:?}", got)
}
