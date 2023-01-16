//!

#![cfg_attr(not(feature = "std"), no_std)]

mod biginteger;
mod fields;
mod groups;
mod rings;

pub use biginteger::{Bits, Bytes, Integer, LimbInt};
pub use fields::{
    Field, MontParameters, MontgomeryOperations, PrimeField, PrimeFieldOperations, F,
};
pub use groups::{Group, PrimeGroup};

pub use rings::Ring;

pub mod ff {
    pub use crate::biginteger::{Bits, Bytes, Integer};
    pub use crate::fields::{
        Field, GeneralReduction, GeneralReductionOperations, MontParameters, MontgomeryOperations,
        PrimeField, PrimeFieldOperations, SolinasParameters, SolinasReduction, F,
    };
    pub use crate::{One, Zero};
    pub use cryp_std::rand::UniformRand;

    #[cfg(test)]
    pub use crate::fields::test_fields;
}

#[cfg(test)]
pub(crate) mod helper {
    use cryp_std::vec::Vec;
    use num_bigint::BigUint;
    pub fn big_int_from_u64(v: &[u64]) -> BigUint {
        use cryp_std::vec;

        let v_u32: Vec<u32> = v
            .iter()
            .flat_map(|&x| vec![x as u32, (x >> 32) as u32])
            .collect();
        BigUint::from_slice(v_u32.as_slice())
    }
}

// ===========================================================================
// General trais

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}
