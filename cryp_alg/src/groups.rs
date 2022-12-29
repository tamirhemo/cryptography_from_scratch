// A group (of prime order)

use cryp_std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
    rand::UniformRand,
};

use zeroize::Zeroize;

pub trait Group:
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
    + Zeroize //  TODO: Consider removing this
    + UniformRand
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<<Self as Group>::ScalarField, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<<Self as Group>::ScalarField>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> Mul<&'a <Self as Group>::ScalarField, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> MulAssign<&'a <Self as Group>::ScalarField>
    + iter::Sum<Self>
    + for<'a> iter::Sum<&'a Self>
{
    type ScalarField;
}
