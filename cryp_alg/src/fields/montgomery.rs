use crate::biginteger::{Limb, LimbInt};
use cryp_std::rand::{Rng, UniformRand};

use super::PrimeFieldOperations;
use cryp_std::fmt::Debug;

pub trait MontParameters<const N: usize>: 'static + Debug {
    // the type of limbs `b` for representing integers
    type Limb: Limb + Debug;
    // the prime modulus `p`
    const MODULUS: [Self::Limb; N];
    /// the constant `m' = -p^(-1) mod b`
    const MP: Self::Limb;
    /// the element `R^2 mod p`
    const R2: [Self::Limb; N];
    // the element `R mod p`
    const R: [Self::Limb; N];
}

/// Montgomery representation of a prime field element
///
/// The element is represented as `x*R mod p`, where `R = b^N`
///
#[derive(Debug)]
pub struct MontgomeryOperations<const N: usize, P: MontParameters<N>> {
    _marker: cryp_std::marker::PhantomData<P>,
}

impl<const N: usize, P: MontParameters<N>> MontgomeryOperations<N, P> {
    /// Montgomery reduction
    ///
    /// Given `x` a double-length integer, the function computes `x*R^-1 mod p`, where `R = b^N`
    pub fn montgomery_reduction(
        element: &(LimbInt<P::Limb, N>, LimbInt<P::Limb, N>),
    ) -> LimbInt<P::Limb, N> {
        // algorithm 14.32 in Handbook of Applied Cryptography

        let (mut a_l, mut a_r) = (element.0, element.1);

        let modulus = LimbInt::from(P::MODULUS);

        for i in 0..N {
            // u = a_i * m′ mod b
            let u = a_l.limbs[i].mul_carry(P::MP, P::Limb::ZERO).0;

            // a = a + u * m * b^i

            // umbi = u * m * b^i = m*(u*b^i)
            //let umbi = modulus.mul_by_limb_shift(u, i);
            let mut ubi = [P::Limb::ZERO; N];
            ubi[i] = u;
            let umbi = modulus.carrying_mul(ubi.into(), LimbInt::zero());

            // add umbi to a
            let (a_0, c) = a_l.carrying_add(umbi.0, P::Limb::NO);
            let (a_1, _) = a_r.carrying_add(umbi.1, c);
            (a_l, a_r) = (a_0, a_1);
        }
        assert_eq!(a_l.limbs, [P::Limb::ZERO; N]);

        // A/b^n = a_r so that's the element we keep

        // if a_r > p, set a_r = a_r - p and return a_r
        // we use checked sub instead of comparison to get constant running time
        let (e, carry) = a_r.carrying_sub(modulus, P::Limb::NO);
        if carry == P::Limb::NO {
            e
        } else {
            a_r
        }
    }

    pub fn montgomery_mul(
        element: &LimbInt<P::Limb, N>,
        other: &LimbInt<P::Limb, N>,
    ) -> LimbInt<P::Limb, N> {
        let multiple = element.carrying_mul(*other, LimbInt::zero());
        Self::montgomery_reduction(&multiple)
    }
}

/// Montgomery representation of a prime field element
impl<const N: usize, P: MontParameters<N>> PrimeFieldOperations for MontgomeryOperations<N, P> {
    type BigInt = LimbInt<P::Limb, N>;
    const MODULUS: Self::BigInt = LimbInt { limbs: P::MODULUS };

    #[inline]
    fn zero() -> Self::BigInt {
        Self::BigInt::zero()
    }

    #[inline]
    fn one() -> Self::BigInt {
        // return `R mod p`
        Self::BigInt::from(P::R)
    }

    /// Checks if the element is zero.
    fn is_zero(element: &Self::BigInt) -> bool {
        let mut flag = false;
        for i in 0..N {
            flag = flag || element.limbs[i] != P::Limb::ZERO;
        }
        !flag
    }

    fn as_int(element: &Self::BigInt) -> Self::BigInt {
        // converts the element from montgomery representation to the integer representation
        // from x*R mod p to x mod p by doing montgomery multiplication with 1.
        let one = Self::BigInt::one();
        Self::montgomery_mul(element, &one)
    }

    fn reduce(element: &Self::BigInt) -> Self::BigInt {
        // Given an integer x, computes x*R mod p by doing multiplication `x*R^2`
        // followed by montgomery reduction
        let xr2 = element.carrying_mul(Self::BigInt::from(P::R2), Self::BigInt::zero());
        Self::montgomery_reduction(&xr2)
    }

    fn rand<R: Rng + ?Sized>(rng: &mut R) -> Self::BigInt {
        // Takes a random collection of limbs and reject it if it is greater than the modulus
        let mut res = [P::Limb::ZERO; N];
        loop {
            for i in 0..N {
                res[i] = P::Limb::rand(rng);
            }
            let element = Self::BigInt::from(res);
            if element.le(&P::MODULUS.into()) {
                break;
            }
        }
        Self::reduce(&res.into())
    }

    fn add_assign(lhs: &mut Self::BigInt, other: &Self::BigInt) {
        let modulus = LimbInt::from(P::MODULUS);
        let (d, c_1) = lhs.carrying_add(*other, P::Limb::NO);

        let (e, c_2) = d.carrying_sub(modulus, P::Limb::NO);

        if c_1 == c_2 {
            *lhs = e;
        } else {
            *lhs = d;
        }
    }

    fn sub_assign(lhs: &mut Self::BigInt, other: &Self::BigInt) {
        let modulus = LimbInt::from(P::MODULUS);
        let (d, c_1) = lhs.carrying_sub(*other, P::Limb::NO);

        let (e, c2) = d.carrying_add(modulus, P::Limb::NO);

        if c_1 == P::Limb::NO {
            *lhs = d;
        } else {
            *lhs = e;
        }
    }

    fn mul_assign(lhs: &mut Self::BigInt, other: &Self::BigInt) {
        *lhs = Self::montgomery_mul(&lhs, other)
    }
}

// =================================================================================================

// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::big_int_from_u64;
    use cryp_std::vec::Vec;
    use num_bigint::BigUint;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct TestParams1;

    impl MontParameters<4> for TestParams1 {
        type Limb = u32;

        // 52765956244737991800116037595123
        const MODULUS: [u32; 4] = [45043, 444, 555, 666];

        const MP: u32 = 1248165573;

        // not needed for reduction
        const R2: [u32; 4] = [1580018471, 1431656072, 715828350, 561]; // NOT VALUE
                                                                       // 52765956244737991800116037595123
        const R: [u32; 4] = [1580018471, 1431656072, 715828350, 561];
    }

    #[test]
    fn test_montgomery_reduction_u32() {
        use rand::thread_rng;
        type Int = LimbInt<u32, 4>;
        let mut rng = thread_rng();
        let a: [u32; 4] = [
            u32::rand(&mut rng),
            u32::rand(&mut rng),
            u32::rand(&mut rng),
            u32::rand(&mut rng),
        ];
        let b: [u32; 4] = [
            u32::rand(&mut rng),
            u32::rand(&mut rng),
            u32::rand(&mut rng),
            u32::rand(&mut rng),
        ];

        // check rng doesn't do anything weird
        assert_ne!(a, b);

        // check reduction is correct
        let modulus = BigUint::new(TestParams1::MODULUS.to_vec());

        let (product_l, product_r) = Int::from(a).carrying_mul(Int::from(b), Int::zero());
        let mont_red =
            MontgomeryOperations::<4, TestParams1>::montgomery_reduction(&(product_l, product_r));

        let product: Vec<u32> = product_l
            .limbs
            .into_iter()
            .chain(product_r.limbs.into_iter())
            .collect();

        let n_a = BigUint::new(a.to_vec());
        let n_b = BigUint::new(b.to_vec());
        let n_product = BigUint::from_slice(product.as_slice());
        assert_eq!(n_product, n_a * n_b);

        let n_mont_red = BigUint::new(mont_red.limbs.to_vec());
        let r = BigUint::new(TestParams1::R.to_vec());
        assert_eq!((n_mont_red * &r) % &modulus, n_product % modulus);
    }

    #[test]
    fn test_montgomery_reduction_u64() {
        use rand::thread_rng;
        type Int = LimbInt<u64, 2>;

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        struct TestParams2;

        impl MontParameters<2> for TestParams2 {
            type Limb = u64;

            // 52765956244737991800116037595123
            const MODULUS: [u64; 2] = [1906965524467, 2860448219691];

            const MP: u64 = 6034914237403725509;

            // not needed for reduction
            const R2: [u64; 2] = [1580018471, 1431656072]; // NOT VALUE

            // 44460203872881598092700617091879
            const R: [u64; 2] = [6148916009939839783, 2410192481406];
        }
        let mut rng = thread_rng();
        let a: [u64; 2] = [u64::rand(&mut rng), u64::rand(&mut rng)];
        let b: [u64; 2] = [u64::rand(&mut rng), u64::rand(&mut rng)];

        // check rng doesn't do anything weird
        assert_ne!(a, b);

        // check reduction is correct
        let modulus = big_int_from_u64([1906965524467, 2860448219691].as_slice());

        let (product_l, product_r) = Int::from(a).carrying_mul(Int::from(b), Int::zero());
        let mont_red =
            MontgomeryOperations::<2, TestParams2>::montgomery_reduction(&(product_l, product_r));

        let product: Vec<u64> = product_l
            .limbs
            .into_iter()
            .chain(product_r.limbs.into_iter())
            .collect();

        let n_a = big_int_from_u64(a.as_slice());
        let n_b = big_int_from_u64(b.as_slice());
        let n_product = big_int_from_u64(product.as_slice());
        assert_eq!(n_product, n_a * n_b);

        let n_mont_red = big_int_from_u64(mont_red.limbs.as_slice());
        let r = big_int_from_u64(TestParams2::R.as_slice());

        // verify montogomery parameters
        let two128 = BigUint::from(2u64).pow(128);
        assert_eq!(&r % &modulus, two128 % &modulus);
        let n_mp = big_int_from_u64(&[TestParams2::MP]);
        let b = BigUint::from(2u64).pow(32);
        assert_eq!((n_mp * &modulus + 1u64) % &b, 0u32 % &b);

        // check reduction
        assert_eq!((n_mont_red * &r) % &modulus, n_product % modulus);
    }

    #[test]
    fn test_montgomery_reduction_25519() {
        use rand::thread_rng;
        type Int = LimbInt<u64, 4>;

        /// Parameters for the prime field Fp25519
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct Fp25519Params;

        impl MontParameters<4usize> for Fp25519Params {
            type Limb = u64;

            const MODULUS: [Self::Limb; 4] = [
                18446744073709551597,
                18446744073709551615,
                18446744073709551615,
                9223372036854775807,
            ];
        
            const R: [Self::Limb; 4] = [38, 0, 0, 0];
        
            const R2: [Self::Limb; 4] = [1444, 0, 0, 0];
            const MP: Self::Limb = 9708812670373448219; 
        }

        let mut rng = thread_rng();
        let a: [u64; 4] = [
            u64::rand(&mut rng),
            u64::rand(&mut rng),
            u64::rand(&mut rng),
            u64::rand(&mut rng),
        ];
        let b: [u64; 4] = [
            u64::rand(&mut rng),
            u64::rand(&mut rng),
            u64::rand(&mut rng),
            u64::rand(&mut rng),
        ];

        // check rng doesn't do anything weird
        assert_ne!(a, b);

        // check reduction is correct
        let modulus = big_int_from_u64(Fp25519Params::MODULUS.as_slice());

        let (product_l, product_r) = Int::from(a).carrying_mul(Int::from(b), Int::zero());
        let mont_red =
            MontgomeryOperations::<4, Fp25519Params>::montgomery_reduction(&(product_l, product_r));

        let product: Vec<u64> = product_l
            .limbs
            .into_iter()
            .chain(product_r.limbs.into_iter())
            .collect();

        let n_a = big_int_from_u64(a.as_slice());
        let n_b = big_int_from_u64(b.as_slice());
        let n_product = big_int_from_u64(product.as_slice());
        assert_eq!(n_product, n_a * n_b);

        let n_mont_red = big_int_from_u64(mont_red.limbs.as_slice());
        let r = big_int_from_u64(Fp25519Params::R.as_slice());

        // verify montogomery parameters
        let two256 = BigUint::from(2u64).pow(256);
        assert_eq!(&r % &modulus, two256 % &modulus);
        let n_mp = big_int_from_u64(&[Fp25519Params::MP]);
        let b = BigUint::from(2u64).pow(64);
        assert_eq!((n_mp * &modulus + 1u64) % &b, 0u64 % &b);
        let r2 = &r * &r;
        assert_eq!(
            r2 % &modulus,
            big_int_from_u64(Fp25519Params::R2.as_slice())
        );

        // check reduction
        assert_eq!((n_mont_red * &r) % &modulus, n_product % modulus);
    }
}
