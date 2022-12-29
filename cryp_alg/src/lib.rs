#![cfg_attr(not(feature = "std"), no_std)]

mod fields;
mod groups;

pub use fields::Field;
pub use groups::Group;

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}
