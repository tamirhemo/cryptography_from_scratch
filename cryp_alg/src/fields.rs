use cryp_std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Neg},
    rand::UniformRand,
};

use zeroize::Zeroize;

use crate::{One, Zero};

mod models;

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
    + Zeroize //  TODO: Consider removing this
    + UniformRand
    + Zero
    + One
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
    + core::iter::Sum<Self>
    + for<'a> iter::Sum<&'a Self>
    + iter::Product<Self>
    + for<'a> iter::Product<&'a Self>
    + From<u8>
    + From<u16>
    + From<u32>
    + From<u64>
    + From<u128>
{

    /// The additive identity of the field.
    const ZERO: Self;
    /// The multiplicative identity of the field.
    const ONE: Self;


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

    fn exp(&self, exp: u64) -> Self {
        let mut result = Self::ONE;
        let mut base = *self;
        let mut exp = exp;

        while exp > 0 {
            if exp & 1 == 1 {
                result *= base;
            }
            exp >>= 1;
            base.square_in_place();
        }

        result
    }
    
    /// Returns `sum([a_i * b_i])`.
    #[inline]
    fn sum_of_products<const T: usize>(a: &[Self; T], b: &[Self; T]) -> Self {
        a.iter()
            .zip(b.iter())
            .map(|(a, b)| *a * *b)
            .sum::<Self>()
    }
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
            assert_eq!(zero, F::ZERO);

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
            assert_eq!(one, F::ONE);

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

        /// Test that the conversion from u128 to F is homomorphic
        fn test_from_u64(num_tests: usize) {
            let mut rng = StepRng::new(0, 1);

            for _ in 0..num_tests {
                let a = u64::rand(&mut rng);
                let b = u64::rand(&mut rng);
                // Test that the conversion from u64 to F is homomorphic
                assert_eq!(F::from(1u64), F::one());
                assert_eq!(F::from(0u64), F::zero());
                assert_eq!(F::from(a) + F::from(b), F::from(a + b));
                assert_eq!(F::from(a) * F::from(b), F::from(a * b));
            }
        }

        fn comparison_with_big_int() {}

        /// Run all tests for a field
        pub fn run_all_tests() {
            let num_tests = 10;
            Self::test_zero(num_tests);
            Self::test_one(num_tests);
            Self::test_additive_commutes(num_tests);
            Self::test_mul_commutes(num_tests);
        }
    }
}
