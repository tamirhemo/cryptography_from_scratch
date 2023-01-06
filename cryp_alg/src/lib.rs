#![cfg_attr(not(feature = "std"), no_std)]

mod biginteger;
mod fields;
mod groups;

pub use biginteger::Integer;
pub use fields::{Field, PrimeField};
pub use groups::{Group, PrimeGroup};

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}
