use cryp_alg::ff::*;
use cryp_alg::{Bits, Bytes};
use cryp_alg::{LimbInt, MontgomeryOperations};

mod test_montgomery {
    use std::ops::Mul;

    use super::*;
    use num_bigint::BigUint;

    fn test_f5() {
        pub type F5 = F<MontgomeryOperations<1, F5Params>>;
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct F5Params;

        impl MontParameters<1usize> for F5Params {
            type Limb = u32;

            const MODULUS: [u32; 1] = [5];

            const R: [u32; 1] = [1];
            const MP: Self::Limb = 858993459u32;
            const R2: [Self::Limb; 1] = [1];
        }

        let one = F5::one();
        let two = one.double();
        let three = two + one;
        let four = two.double();

        assert_eq!(F::zero() - F5::one(), -F5::one());
        assert_eq!(one - one, F5::zero());
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
        assert_eq!(four.pow(4u64), one);
        assert_eq!(four.pow(5u64), four);
        assert_eq!(four.exp(&[5u64]), four);
        assert_eq!(four.exp(&[5u64, 0, 0]), four);
        assert_eq!(four.exp(&LimbInt::from([5u64, 0])), four);

        // test Montgomery reduction and representation
        assert_eq!(one.as_int().limbs, [1]);
        assert_eq!(two.as_int().limbs, [2]);
        assert_eq!(three.as_int().limbs, [3]);
        assert_eq!(four.as_int().limbs, [4]);
    }

    fn test_f26459() {
        pub type F26459 = F<MontgomeryOperations<2, Fp6459Params32>>;
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct Fp6459Params32;

        impl MontParameters<2usize> for Fp6459Params32 {
            type Limb = u32;

            const MODULUS: [u32; 2] = [4294967237, 4294967295];

            const R: [u32; 2] = [59, 0];
            const MP: Self::Limb = 2693454067;
            const R2: [Self::Limb; 2] = [3481, 0];
        }

        let zero = F26459::zero();
        let one = F26459::one();
        let two = one + one;

        let y = F26459::from_int(&[3481, 494389].into());
        assert_eq!(y * y, y.square());
        assert_eq!(y * one, y);

        let mut res = y;
        res = res.square(); // x^2
        res = res.square(); // x^4
        res = res.square(); // x^8
        assert_eq!(res, y.exp(&[8u32, 0]));
        assert_eq!(y.exp(&LimbInt::from([8u32, 0])), y.pow(8));

        let power: [u32; 2] = [4294967236, 4294967295];
        let fp = F26459::from_int(&power.into());
        assert_eq!(fp + one, zero);
        //assert_eq!(x.exp(&[0, 0, 1u32]), x.exp(&[60u32]));
        //assert_eq!(x.exp(&power), x.pow(18446744073709551556));
        //assert_eq!(x.pow(4294967236)*x.pow(18446744069414584320), x.pow(18446744073709551556));
        //assert_eq!(x.exp(&[4294967235, 4294967295u32]), x.inverse().unwrap());
        //assert_eq!(x*x.inverse().unwrap(), one);

        pub type F26459Sol =
            F<GeneralReductionOperations<2, SolinasReduction<2usize, Fp6459SolinasParams32>>>;
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct Fp6459SolinasParams32;

        impl SolinasParameters<2usize> for Fp6459SolinasParams32 {
            type Limb = u32;

            const MODULUS: [u32; 2] = [4294967237, 4294967295];

            const C: u32 = 59;
        }

        let zero = F26459Sol::zero();
        let one = F26459Sol::one();
        let two = one + one;
        assert_eq!(two + zero, two);

        // 2123384586505625
        let x = F26459Sol::from_int(&[3481, 494389].into());
        assert_eq!(x * x, x.square());
        assert_eq!(x * one, x);

        let powerS: [u32; 2] = [4294967236, 4294967295];
        let fp = F26459Sol::from_int(&powerS.into());
        assert_eq!(fp + one, zero);
        assert_eq!(x.exp(&powerS), x.pow(18446744073709551556));
        assert_eq!(
            x.pow(4294967236) * x.pow(18446744069414584320),
            x.pow(18446744073709551556)
        );
        assert_eq!(x, x.exp(&[1u32]));
        assert_eq!(x.exp(&[4294967236, 4294967295u32]), x.exp(&[0u32]));
        assert_eq!(x.exp(&[4294967235, 4294967295u32]) * x, x.exp(&[0u32]));
        assert_eq!(x.exp(&[10, 0, 1u32]), x.exp(&[70u32]));
        assert_eq!(x.exp(&[4294967237, 4294967295u32]), x);
        assert_eq!(x.pow(u64::MAX), x.pow(59));
        assert_eq!(x.exp(&[4294967235, 4294967295u32]), x.inverse().unwrap());
        assert_eq!(x * x.inverse().unwrap(), one);
        //assert_eq!(x.exp(&[4939u32, 23232]), x.exp_non_ct(&[4939u32, 23232]));
        assert_eq!(x.exp(&[1300u64]) * x.exp(&[250u64]), x.exp(&[1550u64]));

        // compare the two implementations
        //assert_eq!(y.exp(&[2,2,2u32]).as_int(), x.exp(&[2,2,2u32]).as_int());
    }

    fn test_fp192() {
        pub type Fp192Sol =
            F<GeneralReductionOperations<3, SolinasReduction<3usize, Fp192SolinasParams>>>;
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct Fp192SolinasParams;

        impl SolinasParameters<3usize> for Fp192SolinasParams {
            type Limb = u64;

            const MODULUS: [u64; 3] = [
                18446744073709551615,
                18446744073709551614,
                18446744073709551615,
            ];

            const C: u64 = u64::MAX;
        }

        let zero = Fp192Sol::zero();
        let one = Fp192Sol::one();
        let two = one + one;
        assert_eq!(two + zero, two);

        let x = Fp192Sol::from_int(&[3481, 494389, 0].into());
        assert_eq!(x * x, x.square());
        assert_eq!(x * one, x);

        let power: [u64; 3] = [
            18446744073709551615,
            18446744073709551614,
            18446744073709551615,
        ];

        assert_eq!(x.inverse().unwrap() * x, one);
    }

    fn test_fp192_mont() {
        // The NIST prime 2^192 - 2^64 - 1
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct Fp192Params;

        pub type Fp192 = F<MontgomeryOperations<3, Fp192Params>>;

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

        let zero = Fp192::zero();
        let one = Fp192::one();
        let two = one + one;
        let three = two + one;
        let four = two.double();

        assert_eq!(
            one.as_int(),
            <Fp192 as PrimeField>::BigInteger::from([1, 0, 0])
        );
        assert_eq!(
            two.as_int(),
            <Fp192 as PrimeField>::BigInteger::from([2, 0, 0])
        );
        assert_eq!(
            three.as_int(),
            <Fp192 as PrimeField>::BigInteger::from([3, 0, 0])
        );
        assert_eq!(
            four.as_int(),
            <Fp192 as PrimeField>::BigInteger::from([4, 0, 0])
        );
    }
}

#[test]
fn test_fp25519() {


    pub type Fp25519Sol =
    F<GeneralReductionOperations<4, SolinasReduction<4usize, Fp25519Params>>>;
    /// Parameters for the prime field of size 2^255 - 19
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Fp25519Params;

    impl SolinasParameters<4usize> for Fp25519Params {
        type Limb = u64;

        // 57896044618658097711785492504343953926634992332820282019728792003956564819949
        const MODULUS: [Self::Limb; 4] = [
            18446744073709551597,
            18446744073709551615,
            18446744073709551615,
            9223372036854775807,
        ];

        const C: Self::Limb = 38;
    }

    let zero = Fp25519Sol::zero();
    let one = Fp25519Sol::one();
    let two = one + one;
    assert_eq!(two + zero, two);

    let x = Fp25519Sol::from_int(&[3481, 494389, 0, 0].into());
    assert_eq!(x * x, x.square());

    let modulus_minus_two : [u64; 4] = [
        18446744073709551595,
        18446744073709551615,
        18446744073709551615,
        9223372036854775807,
    ];

    assert_eq!(Fp25519Sol::from_int(&modulus_minus_two.into()) + two, zero);
    assert_eq!(x.inverse().unwrap(), x.exp(&modulus_minus_two));
    assert_eq!(x * x.inverse().unwrap(), one);

}
