use cryp_alg::ff::*;

mod test_montgomery {
    use std::ops::Mul;

    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn test_f5() {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct F5Params;

        impl MontParameters<1usize> for F5Params {
            type Limb = u32;

            const MODULUS: [u32; 1] = [5];

            const R: [u32; 1] = [1];
            const MP: Self::Limb = 858993459u32;
            const R2: [Self::Limb; 1] = [1];
        }

        pub type F5 = F<MontgomeryOperations<1, F5Params>>;

        let one = F5::one();
        let two = one.double();
        let three = two + one;
        let four = two.double();

        assert_eq!(F::zero() - F5::one(), -F5::one());
        assert_eq!(F5::one() - F5::one(), F5::zero());
        assert_eq!(F5::one() + F5::one(), F5::one().double());

        let zero = F5::zero();
        let one = F5::one();
        let two = one + one;
        let three = two + one;
        let four = three + one;
        let five = four + one;

        assert_eq!(one, F5::one());
        assert_eq!(five, F5::zero());
        assert_eq!(two, F5::one().double());
        assert_eq!(two - one, one);
        assert_eq!(two - four, three);
        assert_eq!(three - four, four);

        assert_eq!(two * two, four);
        assert_eq!(two * three, one);
        assert_eq!(three * three, four);
        assert_eq!(three * four, two);

        // test inversion
        assert_eq!(zero.inverse(), None);
        assert_eq!(one.inverse().unwrap(), one);
        assert_eq!(two.inverse().unwrap(), three);
        assert_eq!(three.inverse().unwrap(), two);

        // test exponentiation
        assert_eq!(four.pow(5), four);
        assert_eq!(
            four.pow(6),
            four.exp(&<F5 as PrimeField>::BigInteger::from([6]))
        );
    }

    fn test_fp192() {
        // The NIST prime 2^192 - 2^64 - 1
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        struct Fp192Params;

        impl MontParameters<3usize> for Fp192Params {
            type Limb = u64;

            const MODULUS: [u64; 3] = [
                18446744073709551615,
                18446744073709551614,
                18446744073709551615,
            ];

            const R: [u64; 3] = [1, 1, 0];
            const R2: [Self::Limb; 3] = [1, 2, 1];
            const MP: Self::Limb = 1;
        }

        pub type Fp192 = F<MontgomeryOperations<3, Fp192Params>>;

        let one = Fp192::one();
        let two = one + one;

        assert_eq!(Fp192::zero() - Fp192::one(), -Fp192::one());

        let modulus = Fp192::from_int(&Fp192Params::MODULUS.into());
        assert_eq!(modulus, Fp192::zero());

        let element = Fp192::from_int(&[0u64, 0u64, 1u64].into());
        assert_eq!(element, Fp192::from_int(&[1, 1, 0].into()));

        let square = element.square();
        assert_eq!(square, element * element);
        assert_eq!(square, Fp192::from_int(&[1, 2, 1].into()));

        let square_again = square.square();
        assert_eq!(square_again, square * element * element);
        assert_eq!(square_again, Fp192::from_int(&[5, 9, 7].into()));
    }
}
