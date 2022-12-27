#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
pub use core::*;

#[cfg(feature = "std")]
#[doc(hidden)]
pub use std::*;

