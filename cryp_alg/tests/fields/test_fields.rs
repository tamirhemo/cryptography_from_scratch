use cryp_alg::ff::*;

pub type F5 = F<MontgomeryOperations<1, F5Params>>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct F5Params;

impl MontParameters<1usize> for F5Params {
    type Limb = u32;

    const MODULUS: [u32; 1] = [5];

    const R: [u32; 1] = [1];
    const MP: Self::Limb = 858993459u32;
    const R2: [Self::Limb; 1] = [1];
}

pub type Fp25519Sol = F<GeneralReductionOperations<4, SolinasReduction<4, Fp25519Params>>>;
pub type Fp25519Mont = F<MontgomeryOperations<4, Fp25519Params>>;

/// Parameters for the prime field Fp25519
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fp25519Params;

impl MontParameters<4usize> for Fp25519Params {
    type Limb = u64;

    const MODULUS: [Self::Limb; 4] = [
        18446744073709551597,
        18446744073709551615,
        18446744073709551615,
        9223372036854775807,
    ];

    const R: [Self::Limb; 4] = [38, 0, 0, 0];

    const R2: [Self::Limb; 4] = [1444, 0, 0, 0];
    const MP: Self::Limb = 9708812670373448219;
}

impl SolinasParameters<4usize> for Fp25519Params {
    type Limb = u64;

    // 2^255-19 =  57896044618658097711785492504343953926634992332820282019728792003956564819949
    const MODULUS: [Self::Limb; 4] = [
        18446744073709551597,
        18446744073709551615,
        18446744073709551615,
        9223372036854775807,
    ];

    const C: [u64; 4] = [38, 0, 0, 0];
}
