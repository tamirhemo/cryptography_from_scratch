use super::general_reduction::{GeneralReduction, GeneralReductionOperations};
use crate::biginteger::{Limb, LimbInt};
use cryp_std::fmt::Debug;

/// Primes with a special form
///
/// Assumes N > 1
pub trait SolinasParameters<const N: usize>: 'static + Debug {
    /// The limb type b
    type Limb: Limb + Debug;

    /// b^N - C, hard-coded
    const MODULUS: [Self::Limb; N];

    /// The constant C so that b^N = C mod p
    const C: Self::Limb;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolinasReduction<const N: usize, P: SolinasParameters<N>> {
    _marker: cryp_std::marker::PhantomData<P>,
}

impl<const N: usize, P: SolinasParameters<N>> GeneralReduction<N> for SolinasReduction<N, P> {
    type Limb = P::Limb;

    const MODULUS: [Self::Limb; N] = P::MODULUS;

    fn reduction(element: &([Self::Limb; N], [Self::Limb; N])) -> [Self::Limb; N] {
        let (mut a_l, mut a_h) = (LimbInt::from(element.0), LimbInt::from(element.1));

        // Note that a = a_l + 2^N * a_h = a_l + R * a_h mod p
        //
        // First compute a_new = a_l + C * a_h  as (a, carry)
        // and repeadedly reduce the new a_h

        // c_vec = [C, 0, 0, .., 0]
        let mut c_vec = LimbInt::from([Self::Limb::ZERO; N]);
        c_vec.limbs[0] = P::C;

        // a = a_h * c_vec + a_l
        (a_l, a_h) = a_h.carrying_mul(c_vec, a_l);
        // deal with carry
        while a_h.limbs != [Self::Limb::ZERO; N] {
            (a_l, a_h) = a_h.carrying_mul(c_vec, a_l);
        }
        let modules = LimbInt::from(Self::MODULUS);

        while modules.le(&a_l) {
            a_l = a_l.carrying_sub(modules, Self::Limb::NO).0;
        }
        a_l.limbs
    }
}

// ================================

// tests

#[cfg(test)]
mod tests {
    use super::GeneralReduction;
    use super::*;
    use crate::helper::big_int_from_u64;
    use cryp_std::rand::UniformRand;
    use cryp_std::vec::Vec;
    use num_bigint::BigUint;

    #[test]
    fn test_solinas_reduction_25519() {
        use rand::thread_rng;
        type Int = LimbInt<u64, 4>;

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
        let two255 = BigUint::from(2u64).pow(255);
        assert_eq!(modulus, &two255 - 19u32);

        let (product_l, product_r) = Int::from(a).carrying_mul(Int::from(b), Int::zero());
        let reduced =
            SolinasReduction::<4usize, Fp25519Params>::reduction_limbint(&(product_l, product_r));

        let product: Vec<u64> = product_l
            .limbs
            .into_iter()
            .chain(product_r.limbs.into_iter())
            .collect();

        let n_a = big_int_from_u64(a.as_slice());
        let n_b = big_int_from_u64(b.as_slice());
        let n_product = big_int_from_u64(product.as_slice());
        assert_eq!(n_product, &n_a * &n_b);

        let n_red = big_int_from_u64(reduced.limbs.as_slice());

        // check c_vec multiplication
        let mut c_vec = LimbInt::from([0; 4]);
        c_vec.limbs[0] = Fp25519Params::C;

        let (r, t) = Int::from(a).carrying_mul(Int::from(c_vec), Int::from(b));
        let n_r = big_int_from_u64(r.limbs.as_slice());
        let n_t = big_int_from_u64(t.limbs.as_slice());
        let n_c = BigUint::from(38u64);
        assert_eq!(
            n_r + &BigUint::from(2u64).pow(256) * &n_t,
            &n_b + &n_a * n_c
        );

        // check reduction
        assert_eq!(n_red % &modulus, n_product % modulus);
    }
}
