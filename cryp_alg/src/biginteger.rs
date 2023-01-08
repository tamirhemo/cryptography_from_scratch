//! Methods and types for big precision integers
//!
//! This module provides a type for big precision integers and methods for converting
//! integers into iterators of bits.
//!
//!

mod limb;
mod limbint;

pub use limb::Limb;
pub use limbint::LimbInt;

pub trait Integer: Sized {
    type Limb: Limb;

    fn into_limbs_le(&self) -> &[Self::Limb];

    fn from_bytes_be(bytes: &[u8]) -> Result<Self, BytesConversionError>;
    fn from_bytes_le(bytes: &[u8]) -> Result<Self, BytesConversionError>;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BytesConversionError {
    LengthTooBig,
    LengthNotMultipleOfLimbSize,
}

/// Provides a namespace for converting an integer type into
/// an iterator of bits.
///
/// *remark* This is a workaround for the lack of impl as return values in trait method
pub struct Bits;

impl Bits {
    /// Converts an integer into an iterator of bits
    ///
    /// The function iterates over limbs, turns every limb into a bit array
    /// (most significant bit first)
    /// and chains all these iterators together.
    #[inline]
    pub fn into_iter_be(element: &impl Integer) -> impl Iterator<Item = bool> + '_ {
        Bytes::into_iter_be(element).flat_map(|b| (0..8).rev().map(move |i| (b >> i) & 1 == 1))
    }
}

pub struct Bytes;

impl Bytes {
    /// Converts an integer into an iterator of bits
    ///
    /// The function iterates over limbs, turns every limb into a bit array
    /// (most significant bit first)
    /// and chains all these iterators together.
    #[inline]
    pub fn into_iter_be(element: &impl Integer) -> impl Iterator<Item = u8> + '_ {
        element
            .into_limbs_le()
            .iter()
            .rev()
            .flat_map(|l| l.into_bytes_be())
    }
}
