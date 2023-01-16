//! Finite field arithmetic - primitives and implementations
//!
//!
//! This module contains various primitives for defining finite fields of arbitrary characteristic.
//! Elements are represented as sequences of Limbs (u32, u64, etc).
//!
//! The main customization is available is the choice of reduction method.
//!
//!

use cryp_std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    rand::UniformRand,
};

use super::{One, Zero};
use crate::Integer;

use zeroize::Zeroize;

mod abstract_operations;
mod exponentiation;

pub use abstract_operations::{PrimeFieldOperations, F};
pub use abstract_operations::general_reduction::{GeneralReduction, GeneralReductionOperations};
pub use abstract_operations::montgomery::{MontParameters, MontgomeryOperations};
pub use abstract_operations::solinas::{SolinasParameters, SolinasReduction};

/// The interface for a field
pub trait Field:
    'static
    + Copy
    + Clone
    + Eq
    + Display
    + Debug
    + Send
    + Sync
    + Sized
    + Hash
    + UniformRand
    + Zero
    + One
    + PartialEq
    + Eq
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Neg<Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> Div<&'a Self, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> MulAssign<&'a Self>
    + for<'a> DivAssign<&'a Self>
    + iter::Sum<Self>
    + for<'a> iter::Sum<&'a Self>
    + iter::Product<Self>
    + for<'a> iter::Product<&'a Self>
{
    /// Computes the multiplicative inverse, if it exists.
    fn inverse(&self) -> Option<Self>;
    /// Squares the field element in place.
    fn square_in_place(&mut self);

    /// Computes the square of the field element.
    fn square(&self) -> Self {
        let mut result = *self;
        result.square_in_place();
        result
    }

    /// Doubles the field element in place.
    fn double_in_place(&mut self);

    fn double(&self) -> Self {
        let mut result = *self;
        result.double_in_place();
        result
    }

    /// Exponentiation by squaring for a small modulus.
    ///
    /// Does not run in constant time.
    fn pow(&self, exp: u64) -> Self {
        let mut res = Self::one();
        let base = *self;

        for i in (0..64).rev() {
            let bit = (exp >> i) & 1 == 1;
            res = res.square();
            if bit {
                res *= base;
            }
        }
        res
    }

    /// Exponentiation by a general exponent
    fn exp(&self, exp: &impl Integer) -> Self {
        let mut res = Self::one();
        let mut base = *self;

        let bits = super::Bits::into_iter_be(exp);
        for bit in bits {
            if bit {
                res *= base;
                base = base.square();
            } else {
                base *= res;
                res = res.square();
            }
        }
        res
    }
}

/// An interface for field of prime order.
///
/// This is a subtrait of `Field` that adds a few additional properties and is meant to be
/// used as scalars for multiplications in cryptographic groups.
///
/// Elements of this field can be represented as integers of some big integer type.
pub trait PrimeField: Field {
    /// The underlying representation as an integer
    ///
    /// Safety: the number of bits representing each element must be constant.
    type BigInteger: Integer + Debug + PartialEq + Eq;

    const MODULUS: Self::BigInteger;

    fn as_int(&self) -> Self::BigInteger;
    fn from_int(int: &Self::BigInteger) -> Self;
}
