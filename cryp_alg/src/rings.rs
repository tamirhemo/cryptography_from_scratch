use cryp_std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    rand::UniformRand,
};

use super::{Field, One, Zero};
use crate::Integer;

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
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> MulAssign<&'a Self>
    + iter::Sum<Self>
    + for<'a> iter::Sum<&'a Self>
    + iter::Product<Self>
    + for<'a> iter::Product<&'a Self>
{
}

/// A wrapper to represent the underlying ring of a Field
pub struct AsRing<F: Field>(pub F);
