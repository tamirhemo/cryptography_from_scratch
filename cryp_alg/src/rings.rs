use cryp_std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    rand::UniformRand,
};

use crate::Integer;
use super::{Zero, One, Field};

use zeroize::Zeroize;


pub trait Ring:
'static
+ Clone
+ Eq
+ Display
+ Debug
+ Send
+ Sized
+ UniformRand
+ Zero
+ One
+ PartialEq
+ Eq
+ Add<Self, Output = Self>
+ Sub<Self, Output = Self>
+ Mul<Self, Output = Self>
+ Neg<Output = Self>
+ AddAssign<Self>
+ SubAssign<Self>
+ MulAssign<Self>
+ for<'a> Add<&'a Self, Output = Self>
+ for<'a> Sub<&'a Self, Output = Self>
+ for<'a> Mul<&'a Self, Output = Self>
+ for<'a> AddAssign<&'a Self>
+ for<'a> SubAssign<&'a Self>
+ for<'a> MulAssign<&'a Self>
+ iter::Sum<Self>
+ for<'a> iter::Sum<&'a Self>
+ iter::Product<Self>
+ for<'a> iter::Product<&'a Self>
{
/// Squares the ring element in place.
fn square_in_place(&mut self);

/// Computes the square of the element
fn square(&self) -> Self {
    let mut result = *self;
    result.square_in_place();
    result
}

/// Doubles the element in place.
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


/// A wrapper to represent the underlying ring of a Field
pub struct AsRing<F: Field>(pub F);