// A group (of prime order)

use cryp_std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Neg},
    rand::{Rng, UniformRand},
};

use crate::Field;

//use zeroize::Zeroize;

/// Interface for a group of prime order.
///
pub trait Group:
    'static
    + Copy
    + Clone
    + PartialEq
    + Eq
    + Display
    + Debug
    + Send
    + Sync
    + Sized
    + Hash
    + UniformRand
    + Add<Self, Output = Self>
    + Neg<Output = Self>
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
    type ScalarField: Field;

    const IDENTITY: Self;

    fn identity() -> Self {
        Self::IDENTITY
    }

    fn is_identity(&self) -> bool {
        *self == Self::IDENTITY
    }

    fn generator<R: Rng>(rng: Option<R>) -> Self;

    fn double_in_place(&mut self);

    fn double(&self) -> Self {
        let mut res = *self;
        res.double_in_place();
        res
    }

    /// Performs multi-scalar multiplication of a tuple of bases and scalars.
    ///
    /// The bases and scalars are given as slices of the same length.
    /// the function will panic if the slices are not of the same length.
    fn multiscalar_mul(scalars: &[Self::ScalarField], bases: &[Self]) -> Self {
        assert_eq!(scalars.len(), bases.len());
        scalars
            .iter()
            .zip(bases.into_iter())
            .fold(Self::IDENTITY, |acc, (s, b)| acc + b.mul(s))
    }
}
