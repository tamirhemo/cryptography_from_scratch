use super::{Field, Integer, PrimeField};
use cryp_std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    iter,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    rand::{
        distributions::{Distribution, Standard},
        Rng,
    },
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
        + From<u16>
        + From<u32>
        + From<u64>
        + From<u128>
        + From<u8>
        + Send
        + Sync
        + 'static;

    const MODULUS : Self::BigInt;

    /// The zero element of the field.
    fn zero() -> Self::BigInt;

    ///The multiplicative identity of the field.
    fn one() -> Self::BigInt;

    /// Returns the reduction of the element modulo the prime.
    fn reduce(element: &Self::BigInt) -> Self::BigInt;

    /// Checks if the element is zero.
    fn is_zero(element: &Self::BigInt) -> bool;

    // Generate a random element of the field.
    fn sample<R: Rng + ?Sized>(rng: &mut R) -> Self::BigInt;

    /// Checks if two elements are equal.
    ///
    /// Default implementations uses the zero comparison and substraction.
    fn equals(lhs: &Self::BigInt, rhs: &Self::BigInt) -> bool {
        let mut res = *lhs;
        Self::negation(&mut res);
        Self::add_assign(&mut res, rhs);
        Self::is_zero(&res)
    }

    /// Addition of two elements in place.
    fn add_assign(lhs: &mut Self::BigInt, other: &Self::BigInt);

    /// Negation of an element.
    fn negation(element: &Self::BigInt) -> Self::BigInt;

    /// Multiplication of two elements in place.
    fn mul_assign(lhs: &mut Self::BigInt, other: &Self::BigInt);

    /// The multiplicative inverse of an element, if exists
    fn inverse(element: &Self::BigInt) -> Option<Self::BigInt>;
}

#[derive(Debug)]
pub struct F<S: PrimeFieldOperations> {
    pub element: S::BigInt,
}

impl<S: PrimeFieldOperations> F<S> {
    fn new(element: S::BigInt) -> Self {
        Self {
            element: S::reduce(&element),
        }
    }
}

//------------------------------------
// Trait implementations
//------------------------------------

impl<S: PrimeFieldOperations> Field for F<S> {
    fn zero() -> Self {
        Self::new(S::zero())
    }

    fn one() -> Self {
        Self::new(S::one())
    }

    fn inverse(&self) -> Option<Self> {
        S::inverse(&self.element).map(Self::new)
    }

    fn square_in_place(&mut self) {
        let other = self.element;
        S::mul_assign(&mut self.element, &other);
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
        let mut negated = other.element;
        S::negation(&mut negated);
        S::add_assign(&mut self.element, &negated);
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
        S::negation(&mut result.element);
        result
    }
}

impl<S: PrimeFieldOperations> iter::Sum for F<S> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(F::new(S::zero()), |acc, x| acc + x)
    }
}

impl<'a, S: PrimeFieldOperations> iter::Sum<&'a Self> for F<S> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(F::new(S::zero()), |acc, x| acc + x)
    }
}

impl<S: PrimeFieldOperations> iter::Product for F<S> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(F::new(S::one()), |acc, x| acc * x)
    }
}

impl<'a, S: PrimeFieldOperations> iter::Product<&'a Self> for F<S> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(F::new(S::one()), |acc, x| acc * x)
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

impl<S: PrimeFieldOperations> Distribution<F<S>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F<S> {
        F::new(S::sample(rng))
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

impl_from!(u8, u16, u32, u64, u128);
