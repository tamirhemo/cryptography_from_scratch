use cryp_std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    rand::UniformRand,
};

use zeroize::Zeroize;

use crate::{One, Zero};

mod models;

/// The interface for a (finite) field
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
    + From<u128>
    + From<u64>
    + From<u32>
    + From<u16>
    + From<u8>
{
    type BasePrimeField: PrimeField;

    /// The additive identity of the field.
    const ZERO: Self;
    /// The multiplicative identity of the field.
    const ONE: Self;
}

pub trait PrimeField: Field<BasePrimeField = Self> {}

#[cfg(test)]
mod field_tests {
    use super::*;
    use cryp_std::rand::rngs::mock::StepRng;

    pub struct FieldTests<F: Field>(cryp_std::marker::PhantomData<F>);

    impl<F: Field> FieldTests<F> {
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

        fn test_additive_commutes(num_tests: usize) {
            let mut rng = StepRng::new(0, 1);

            for _ in 0..num_tests {
                let element = F::rand(&mut rng);
                let other = F::rand(&mut rng);
                // Test that one is multiplicative identity
                assert_eq!(element + other, other + element);
            }
        }

        fn test_mul_commutes(num_tests: usize) {
            let mut rng = StepRng::new(0, 1);

            for _ in 0..num_tests {
                let element = F::rand(&mut rng);
                let other = F::rand(&mut rng);
                // Test that one is multiplicative identity
                assert_eq!(element * other, other * element);
            }
        }

        pub fn run_all_tests() {
            let num_tests = 10;
            Self::test_zero(num_tests);
            Self::test_one(num_tests);
            Self::test_additive_commutes(num_tests);
            Self::test_mul_commutes(num_tests);
        }
    }
}
