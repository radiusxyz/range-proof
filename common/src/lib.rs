use num_bigint::BigUint;
use std::str::FromStr;
// MODULUS is 2048 mod / number of sequencers for skde
pub const MODULUS: &str = "109108784166676529682340577929498188950239585527883687884827626040722072371127456712391033422811328348170518576414206624244823392702116014678887602655605057984874271545556188865755301275371611259397284800785551682318694176857633188036311000733221068448165870969366710007572931433736793827320953175136545355129";
pub const BASE: &str = "4";
pub const RANGE: &str =
    "54228695914669666723440166889041962662973721213812451561550491637090461709551";
pub const EXPONENT: &str = "462000193083985684610660351369692616274581519034636217798321";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Input {
    pub base: BigUint,
    pub modulus: BigUint,
    pub exponent: BigUint,
    pub range: BigUint,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub base: BigUint,
    pub modulus: BigUint,
    pub range: BigUint,
    pub u: BigUint,
}

impl Input {
    // Constructor using default constants
    pub fn new_default() -> Self {
        Self::new(BASE, MODULUS, RANGE, EXPONENT)
    }

    // Constructor that allows custom input
    pub fn new(base_str: &str, modulus_str: &str, range_str: &str, exponent_str: &str) -> Self {
        let base = BigUint::from_str(base_str).expect("Invalid number for Base");
        let modulus = BigUint::from_str(modulus_str).expect("Invalid number for Modulus");
        let range = BigUint::from_str(range_str).expect("Invalid number for Range");
        let exponent = BigUint::from_str(exponent_str).expect("Invalid number for Exponent");
        println!("Initial parameter settings");
        println!("Base: {}", base);
        println!("Modulus: {}", modulus);
        println!("Range: {}", range);

        Input {
            base,
            modulus,
            range,
            exponent,
        }
    }
}
