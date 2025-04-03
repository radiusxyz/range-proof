use common::{Input, EXPONENT};
use num_bigint::BigUint;
use risc0_zkvm::guest::env;
use std::str::FromStr;
use std::{io::Read, vec::Vec};

pub fn main() {
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    let input: Input = bincode::deserialize(&input_bytes).unwrap();
    let base = input.base;
    let modulus = input.modulus;
    let range = input.range;

    let exponent = BigUint::from_str(EXPONENT).expect("Guest::Invalid number for Exponent");
    env::write(&exponent);

    if exponent > range {
        panic!(
            "Guest::Range proof generation failed: Exponent ({}) is out of range ({})",
            exponent, range
        );
    }

    let calculation = base.modpow(&exponent, &modulus);

    println!("Guest::Calculation: {:?}", calculation);

    env::commit(&(calculation));
}
