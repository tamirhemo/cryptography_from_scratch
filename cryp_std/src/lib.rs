#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
pub use core::*;

#[cfg(not(feature = "std"))]
#[doc(hidden)]
extern crate alloc;

#[cfg(not(feature = "std"))]
pub use alloc::*;

#[cfg(not(feature = "std"))]
pub mod fmt {
    pub use alloc::fmt::*;
    pub use core::fmt::*;
}

#[cfg(feature = "std")]
#[doc(hidden)]
pub use std::*;

#[doc(hidden)]
pub use rand;

pub trait UniformRand: Sized {
    fn rand<R: rand::RngCore + ?Sized>(rng: &mut R) -> Self;
}
