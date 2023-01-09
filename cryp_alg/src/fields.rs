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
    vec::Vec,
};

use crate::Integer;

use zeroize::Zeroize;

mod abstract_operations;
mod general_reduction;
mod montgomery;
mod solinas;

pub use abstract_operations::{PrimeFieldOperations, F};
pub use general_reduction::{GeneralReduction, GeneralReductionOperations};
pub use montgomery::{MontParameters, MontgomeryOperations};
pub use solinas::{SolinasParameters, SolinasReduction};

pub use general_reduction::*;

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
    fn zero() -> Self;

    fn one() -> Self;

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

#[cfg(test)]
mod field_tests {
    use super::*;
    use cryp_std::rand::rngs::mock::StepRng;

    pub struct FieldTests<F: Field>(cryp_std::marker::PhantomData<F>);

    impl<F: Field> FieldTests<F> {
        /// Test that the additive identity is correct, and multiplicative zero is correct
        fn test_zero(num_tests: usize) {
            let zero = F::zero();

            let mut rng = StepRng::new(0, 1);
            for _ in 0..num_tests {
                let element = F::rand(&mut rng);
                // Test that zero is additive identity
                assert_eq!(element + zero, element);
                assert_eq!(zero + element, element);
                // Test zero multiplicative property
                assert_eq!(element * zero, zero);
                assert_eq!(zero * element, zero);
            }
        }

        /// Test that the multiplicative identity is correct
        fn test_one(num_tests: usize) {
            let one = F::one();

            let mut rng = StepRng::new(0, 1);
            for _ in 0..num_tests {
                let element = F::rand(&mut rng);
                // Test that one is multiplicative identity
                assert_eq!(element * one, one);
                assert_eq!(one * element, one);
            }
        }

        /// Test that the addition is commutative
        fn test_additive_commutes(num_tests: usize) {
            let mut rng = StepRng::new(0, 1);

            for _ in 0..num_tests {
                let element = F::rand(&mut rng);
                let other = F::rand(&mut rng);
                // Test that one is multiplicative identity
                assert_eq!(element + other, other + element);
            }
        }

        /// Test that the multiplication is commutative
        fn test_mul_commutes(num_tests: usize) {
            let mut rng = StepRng::new(0, 1);

            for _ in 0..num_tests {
                let element = F::rand(&mut rng);
                let other = F::rand(&mut rng);
                // Test that one is multiplicative identity
                assert_eq!(element * other, other * element);
            }
        }

        /*
        /// Test that the conversion from u128 to F is homomorphic
        fn test_from_u32(num_tests: usize) {
            let mut rng = StepRng::new(0, 1);

            for _ in 0..num_tests {
                let a = u32::rand(&mut rng);
                let b = u32::rand(&mut rng);
                // Test that the conversion from u64 to F is homomorphic
                assert_eq!(F::from(1u32), F::one());
                assert_eq!(F::from(0u32), F::zero());
                assert_eq!(F::from(a) + F::from(b), F::from(a + b));
                assert_eq!(F::from(a) * F::from(b), F::from(a * b));
            }
        }
        */

        fn comparison_with_big_int() {}

        /// Run all tests for a field
        pub fn run_all_tests() {
            let num_tests = 10;
            Self::test_zero(num_tests);
            Self::test_one(num_tests);
            Self::test_additive_commutes(num_tests);
            Self::test_mul_commutes(num_tests);
            //Self::test_from_u32(num_tests);
        }
    }
}

#[cfg(test)]
mod prime_field_tests {
    use super::*;
    use cryp_std::rand::rngs::mock::StepRng;

    pub struct PrimeFieldTests<F: PrimeField>(cryp_std::marker::PhantomData<F>);

    impl<F: PrimeField> PrimeFieldTests<F> {
        /// Test that the multiplicative inverse is correct
        fn test_modulus() {
            //TODO
        }

        /// Run all tests for a field
        pub fn run_all_tests() {
            Self::test_modulus();
        }
    }
}
