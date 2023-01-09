use cryp_std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    rand::{Rng, UniformRand},
    vec::Vec,
};

use crate::{Bits, Integer};

use core::borrow::Borrow;

use crate::PrimeField;

//use zeroize::Zeroize;

/// Interface for a group
pub trait Group:
    'static
    + Copy
    + Clone
    + PartialEq
    + Eq
    //+ Display 
    + Debug
    + Send
    + Sync
    + Sized
    + Hash
    + Add<Self, Output = Self>
    + Neg<Output = Self>
    + Sub<Self, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
    + iter::Sum<Self>
    + for<'a> iter::Sum<&'a Self>
{

    fn identity() -> Self;

    fn is_identity(&self) -> bool {
        *self == Self::identity()
    }

    fn double_in_place(&mut self);

    fn double(&self) -> Self {
        let mut res = *self;
        res.double_in_place();
        res
    }

    fn mul_int(&self, scalar: & impl Integer) -> Self {
        let mut res = Self::identity();
        let mut base = *self;

        let bits = Bits::into_iter_be(scalar);
        for bit in bits {
            if bit {
                res += base;
                base.double();
            } else {
                base += res;
                res.double();
            }
        }
        res
    }
}

/// Interface for a group of prime order, this is a group used in cryptographic protocols.
///
pub trait PrimeGroup:
    Group
    + for<'a> Mul<&'a <Self as PrimeGroup>::ScalarField, Output = Self>
    + for<'a> MulAssign<&'a <Self as PrimeGroup>::ScalarField>
    + Add<Self::Public, Output = Self>
    + AddAssign<Self::Public>
    + Sub<Self::Public, Output = Self>
    + SubAssign<Self::Public>
    + for<'a> Add<&'a Self::Public, Output = Self>
    + for<'a> AddAssign<&'a Self::Public>
    + for<'a> Sub<&'a Self::Public, Output = Self>
    + for<'a> SubAssign<&'a Self::Public>
{
    type ScalarField: PrimeField;

    /// The presentation of the group element that is safe to send publictly.
    ///
    /// This is useful in e.g. elliptic curves since sending projective coordinates may leak
    /// information about the private key.
    ///
    /// An element in the public representation can be turned into an element in the internal representation.
    type Public: Clone
        + Copy
        + PartialEq
        + Eq
        //+ Debug
        + Hash
        + Send
        + Sync
        + Sized
        + 'static
        + Into<Self>
        + for<'a> Mul<&'a <Self as PrimeGroup>::ScalarField, Output = Self>
        + Add<Self::Public, Output = Self>
        + for<'a> Add<&'a Self::Public, Output = Self>;

    /// Gives a generator for the group.
    fn generator<R: Rng>(rng: Option<R>) -> Self::Public;

    /// Verifies that a given `Public` type is a valid element of the group
    fn is_valid(input: &Self::Public) -> bool;

    /// Attempts to convert an an element of the group into the `Public` type.
    fn as_public(&self) -> Option<Self::Public>;

    /// Gives a vector of generators of the group of size `n`.
    ///
    /// The generators should be independent in the sense that the mutual
    /// discrete logarithms are not known.
    fn batch_generators(n: usize, rng: Option<impl Rng>) -> Vec<Self::Public>;

    /// Multi-scalar multiplication with a vector of secret scalars.
    ///
    /// The iteretors should be of the same length (this is not checked).
    ///
    /// Users should transform the output of this function into a `Self::Public` type before
    /// sending it to other parties.
    fn msm<I, J>(bases: I, scalars: J) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<Self::Public>,
        J: IntoIterator,
        J::Item: Borrow<<Self as PrimeGroup>::ScalarField>;
}
