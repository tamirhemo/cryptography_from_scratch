use cryp_alg::ff::*;
use cryp_alg::{Bits, Bytes};
use cryp_std::rand::thread_rng;
use num_bigint::BigUint;

mod test_fields;

use test_fields::{Fp25519Mont, Fp25519Sol};

#[test]
fn test_fp25519_mont() {
    FieldTests::<Fp25519Mont>::run_all_tests(100);
    PrimeFieldTests::<Fp25519Mont>::run_all_tests(100);
}

#[test]
fn test_fp25519_solinas() {
    FieldTests::<Fp25519Sol>::run_all_tests(100);
    PrimeFieldTests::<Fp25519Sol>::run_all_tests(100);
}

pub struct FieldTests<F: Field>(cryp_std::marker::PhantomData<F>);

impl<F: Field> FieldTests<F> {
    /// Test that the additive and multiplicative identity is correct
    fn test_one_zero(num_tests: usize) {
        let one = F::one();
        let zero = F::zero();

        let mut rng = thread_rng();
        for _ in 0..num_tests {
            let element = F::rand(&mut rng);
            // Test that one is multiplicative identity
            assert_eq!(element * one, element);
            assert_eq!(one * element, element);
            assert_eq!(element + zero, element);
            assert_eq!(zero + element, element);
            assert_eq!(zero * element, zero);
        }
    }

    /// Test that the addition is commutative
    fn test_addition(num_tests: usize) {
        let mut rng = thread_rng();

        for _ in 0..num_tests {
            let element = F::rand(&mut rng);
            let other = F::rand(&mut rng);
            assert_eq!(element + other, other + element);
            assert_eq!(element - other, -(other - element));
        }
    }

    /// Test that the multiplication is commutative
    fn test_multiplication(num_tests: usize) {
        let mut rng = thread_rng();

        for _ in 0..num_tests {
            let element = F::rand(&mut rng);
            let other = F::rand(&mut rng);

            assert_eq!(element * other, other * element);

            // test multiplicative inverse
            if element != F::zero() {
                assert_eq!(element * element.inverse().unwrap(), F::one());
            }
        }
    }

    /// Run all tests for a field
    pub fn run_all_tests(num_tests: usize) {
        Self::test_one_zero(num_tests);
        Self::test_addition(num_tests);
        Self::test_multiplication(num_tests);
    }
}

pub struct PrimeFieldTests<F: PrimeField>(cryp_std::marker::PhantomData<F>);

impl<F: PrimeField> PrimeFieldTests<F> {
    fn test_modulus() {
        assert_eq!(F::from_int(&F::MODULUS), F::zero());
    }

    fn test_as_bigint(num_tests: usize) {
        let mut rng = thread_rng();

        let to_bigint = |x: F| {
            let bytes_be: Vec<u8> = Bytes::into_iter_be(&x.as_int()).collect();
            BigUint::from_bytes_be(&bytes_be)
        };

        let modulus =
            BigUint::from_bytes_be(&Bytes::into_iter_be(&F::MODULUS).collect::<Vec<u8>>());

        for _ in 0..num_tests {
            let a = F::rand(&mut rng);
            let b = F::rand(&mut rng);

            let n_a = to_bigint(a);
            let n_b = to_bigint(b);

            // addition
            let n_sum = to_bigint(a + b);
            assert_eq!(n_sum, (&n_a + &n_b) % &modulus);

            // subtraction
            let n_sub = to_bigint(a - b);
            assert_eq!(n_sub, (&n_a - &n_b) % &modulus);

            // multiplication
            let n_mul = to_bigint(a * b);
            assert_eq!(n_mul, (&n_a * &n_b) % &modulus);

            // division
            if b != F::zero() {
                let n_div = to_bigint(a / b);
                assert_eq!((&n_a * n_div) % &modulus, &n_b % &modulus);
            }
            // inverse and exponentiation
            let mod_minus_two =
                BigUint::iter_u32_digits(&(&modulus - &BigUint::from(2u8))).collect::<Vec<u32>>();

            assert_eq!(a.exp(&mod_minus_two) * a, F::one());
        }
    }

    /// Run all tests for a field
    pub fn run_all_tests(num_tests: usize) {
        //Self::test_modulus();
        //Self::test_as_bigint(num_tests);
    }
}
