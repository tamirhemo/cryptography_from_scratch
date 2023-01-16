pub(crate) mod montgomery;
pub(crate) mod general_reduction;
pub(crate) mod solinas;

use crate::{biginteger::Bits, One, Zero};

use super::{Field, Integer, PrimeField};
use cryp_std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    iter,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    rand::{Rng, UniformRand},
};

/// An interface for defining operations on a prime field.
///
/// The user may make any assumptions about the inputs to these functions
/// at thier own discretion.
pub trait PrimeFieldOperations: 'static + Debug {
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

    /// Addition of two elements in place.
    fn add_assign(lhs: &mut Self::BigInt, other: &Self::BigInt);

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

    /// The multiplicative inverse of an element, if exists
    /// 
    /// The default implementation is based on Fermat's little theorem. A more
    /// efficient implementation may be provided by the user.
    fn inverse(element: &Self::BigInt) -> Option<Self::BigInt> {
        let mut modulus_minus_two = Self::one();
        Self::add_assign(&mut modulus_minus_two, &Self::one());
        Self::negation_in_place(&mut modulus_minus_two);
        let power = Self::as_int(&modulus_minus_two);

        let res = Self::exp(element, &power);

        if Self::is_zero(&res) {
            None
        } else {
            Some(res)
        }
    }

    /// Exponentiation of an element.
    ///
    ///  Default implementation is based on the Montgomery ladder algorithm and runs
    /// in constant time depending only on the length of exp.to_bits_be().
    /// Thus, the user must make sure all secret exponents have the same bit length.
    fn exp(element: &Self::BigInt, exp: &impl Integer) -> Self::BigInt {
        let mut res = Self::one();
        let mut base = *element;

        let bits = Bits::into_iter_be(exp);
        for bit in bits {
            if bit {
                Self::mul_assign(&mut res, &base);
                Self::square_assign(&mut base);
            } else {
                Self::mul_assign(&mut base, &res);
                Self::square_assign(&mut res);
            }
        }
        res
    }
}

#[derive(Debug)]
pub struct F<S: PrimeFieldOperations> {
    element: S::BigInt,
}

impl<S: PrimeFieldOperations> F<S> {
    /// Allows the assignment of a field element with specific limbs
    ///
    /// This function is meant to be used when the user needs to hard-encode
    /// a field element. The user should make sure that the element is in
    /// the correct representation.
    #[inline]
    #[allow(non_snake_case)]
    pub const fn from_RAW_limbs(element: S::BigInt) -> Self {
        Self { element }
    }
}

//------------------------------------
// Trait implementations
//------------------------------------

impl<S: PrimeFieldOperations> Zero for F<S> {
    fn zero() -> Self {
        Self::from_RAW_limbs(S::zero())
    }
}

impl<S: PrimeFieldOperations> One for F<S> {
    fn one() -> Self {
        Self::from_RAW_limbs(S::one())
    }
}

impl<S: PrimeFieldOperations> Field for F<S> {
    fn inverse(&self) -> Option<Self> {
        S::inverse(&self.element).map(Self::from_RAW_limbs)
    }

    fn square_in_place(&mut self) {
        S::square_assign(&mut self.element);
    }

    fn double_in_place(&mut self) {
        S::double_assign(&mut self.element);
    }

    fn exp(&self, exp: &impl Integer) -> Self {
        Self::from_RAW_limbs(S::exp(&self.element, exp))
    }
}

impl<S: PrimeFieldOperations> PrimeField for F<S> {
    type BigInteger = S::BigInt;

    const MODULUS: Self::BigInteger = S::MODULUS;

    fn as_int(&self) -> Self::BigInteger {
        S::as_int(&self.element)
    }

    fn from_int(int: &Self::BigInteger) -> Self {
        Self::from_RAW_limbs(S::reduce(int))
    }
}

// ------------------------
// Operations
// ------------------------

impl<S: PrimeFieldOperations> AddAssign<&F<S>> for F<S> {
    fn add_assign(&mut self, other: &F<S>) {
        S::add_assign(&mut self.element, &other.element);
    }
}

impl<S: PrimeFieldOperations> AddAssign for F<S> {
    fn add_assign(&mut self, other: F<S>) {
        *self += &other;
    }
}

impl<S: PrimeFieldOperations> Add for F<S> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = self;
        result += other;
        result
    }
}

impl<S: PrimeFieldOperations> Add<&F<S>> for F<S> {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        let mut result = self;
        result += other;
        result
    }
}

impl<S: PrimeFieldOperations> SubAssign<&F<S>> for F<S> {
    fn sub_assign(&mut self, other: &F<S>) {
        S::sub_assign(&mut self.element, &other.element);
    }
}

impl<S: PrimeFieldOperations> SubAssign for F<S> {
    fn sub_assign(&mut self, other: F<S>) {
        *self -= &other;
    }
}

impl<S: PrimeFieldOperations> Sub for F<S> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = self;
        result -= other;
        result
    }
}

impl<S: PrimeFieldOperations> Sub<&F<S>> for F<S> {
    type Output = Self;

    fn sub(self, other: &Self) -> Self {
        let mut result = self;
        result -= other;
        result
    }
}

impl<S: PrimeFieldOperations> MulAssign<&F<S>> for F<S> {
    fn mul_assign(&mut self, other: &F<S>) {
        S::mul_assign(&mut self.element, &other.element);
    }
}

impl<S: PrimeFieldOperations> MulAssign for F<S> {
    fn mul_assign(&mut self, other: F<S>) {
        *self *= &other;
    }
}

impl<S: PrimeFieldOperations> Mul for F<S> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = self;
        result *= other;
        result
    }
}

impl<S: PrimeFieldOperations> Mul<&F<S>> for F<S> {
    type Output = Self;

    fn mul(self, other: &Self) -> Self {
        let mut result = self;
        result *= other;
        result
    }
}

impl<S: PrimeFieldOperations> DivAssign<&F<S>> for F<S> {
    fn div_assign(&mut self, other: &F<S>) {
        let inverse = S::inverse(&other.element).expect("Division by zero");
        S::mul_assign(&mut self.element, &inverse);
    }
}

impl<S: PrimeFieldOperations> DivAssign for F<S> {
    fn div_assign(&mut self, other: F<S>) {
        *self /= &other;
    }
}

impl<S: PrimeFieldOperations> Div for F<S> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let mut result = self;
        result /= other;
        result
    }
}

impl<S: PrimeFieldOperations> Div<&F<S>> for F<S> {
    type Output = Self;

    fn div(self, other: &Self) -> Self {
        let mut result = self;
        result /= other;
        result
    }
}

impl<S: PrimeFieldOperations> Neg for F<S> {
    type Output = Self;

    fn neg(self) -> Self {
        let mut result = self;
        S::negation_in_place(&mut result.element);
        result
    }
}

impl<S: PrimeFieldOperations> iter::Sum for F<S> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(F::from_RAW_limbs(S::zero()), |acc, x| acc + x)
    }
}

impl<'a, S: PrimeFieldOperations> iter::Sum<&'a Self> for F<S> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(F::from_RAW_limbs(S::zero()), |acc, x| acc + x)
    }
}

impl<S: PrimeFieldOperations> iter::Product for F<S> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(F::from_RAW_limbs(S::one()), |acc, x| acc * x)
    }
}

impl<'a, S: PrimeFieldOperations> iter::Product<&'a Self> for F<S> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(F::from_RAW_limbs(S::one()), |acc, x| acc * x)
    }
}

//------------------------------------
// Hashing, Clone, formating traits
//------------------------------------

impl<S: PrimeFieldOperations> PartialEq for F<S> {
    fn eq(&self, other: &Self) -> bool {
        S::equals(&self.element, &other.element)
    }
}

impl<S: PrimeFieldOperations> Eq for F<S> {}

impl<S: PrimeFieldOperations> Hash for F<S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.element.hash(state);
    }
}

impl<S: PrimeFieldOperations> Clone for F<S> {
    fn clone(&self) -> Self {
        Self {
            element: self.element.clone(),
        }
    }
}

impl<S: PrimeFieldOperations> Copy for F<S> {}

impl<S: PrimeFieldOperations> cryp_std::fmt::Display for F<S> {
    fn fmt(&self, f: &mut cryp_std::fmt::Formatter) -> cryp_std::fmt::Result {
        write!(f, "F{}", self.element)
    }
}

// ------------
// random field element
// CAREFUL

impl<S: PrimeFieldOperations> UniformRand for F<S> {
    fn rand<R: Rng + ?Sized>(rng: &mut R) -> Self {
        F::from_RAW_limbs(S::rand(rng))
    }
}

//------------
// From numerical types
//------------

macro_rules! impl_from {
    ($($t:ty),*) => {
        $(
            impl<S: PrimeFieldOperations> From<$t> for F<S> {
                fn from(x: $t) -> Self {
                    Self::new(S::BigInt::from(x))
                }
            }
        )*
    };
}

//impl_from!(u32);
