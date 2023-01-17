
use super:: {Integer};

use cryp_std::{
    fmt::{Debug, Display},
    hash::Hash,
    rand::Rng,
};


/// An interface for defining operations on a prime field.
///
/// The user may make any assumptions about the inputs to these functions
/// at thier own discretion.
pub trait ArithmeticOperations: 'static + Debug {
    type BigInt: Integer
        + Clone
        + Copy
        + Hash
        + Debug
        + Display
        + PartialEq
        + Eq
        + Send
        + Sync
        + 'static;

    const MODULUS: Self::BigInt;

    /// The zero element of the field.
    fn zero() -> Self::BigInt;

    ///The multiplicative identity of the field.
    fn one() -> Self::BigInt;

    // Allows for different internal representation of field elements
    fn as_int(element: &Self::BigInt) -> Self::BigInt;

    /// Returns the reduction of the element modulo the prime.
    fn reduce(element: &Self::BigInt) -> Self::BigInt;

    /// Checks if the element is zero.
    fn is_zero(element: &Self::BigInt) -> bool;

    /// A random element of the field.
    fn rand<R: Rng + ?Sized>(rng: &mut R) -> Self::BigInt;

    /// Checks if two elements are equal.
    ///
    /// Note that we need to check equality in the field, so this is checking
    /// equality mod p
    ///
    /// Default implementations uses the zero comparison and substraction.
    fn equals(lhs: &Self::BigInt, rhs: &Self::BigInt) -> bool {
        let mut res = *lhs;
        Self::sub_assign(&mut res, &rhs);
        Self::is_zero(&res)
    }

    /// Addition of an element rhs to the element lhs in place.
    fn add_assign(lhs: &mut Self::BigInt, other: &Self::BigInt);

    /// Subtraction of the element rhs to the element lhs in place.
    fn sub_assign(lhs: &mut Self::BigInt, other: &Self::BigInt);

    /// Negation of an element.
    fn negation_in_place(element: &mut Self::BigInt) {
        let mut res = Self::zero();
        Self::sub_assign(&mut res, &element);
        *element = res;
    }

    /// Multiplication of two elements in place.
    fn mul_assign(lhs: &mut Self::BigInt, other: &Self::BigInt);

    /// Squaring the element in place
    ///
    /// Default implementation uses the multiplication but users may want
    /// to override this function for performance reasons.
    ///
    /// Users should **not** assume squaring has the same time cost as
    /// a multiplication.
    fn square_assign(element: &mut Self::BigInt) {
        let other = *element;
        Self::mul_assign(element, &other);
    }

    /// Doubling the element in place
    ///
    /// Default implementation uses the addition but users may want
    /// to override this function for performance reasons.
    ///
    /// Users should **not** assume squaring has the same time cost as
    /// a multiplication.
    fn double_assign(element: &mut Self::BigInt) {
        let other = *element;
        Self::add_assign(element, &other);
    }

}