use crate::biginteger::{Limb, LimbInt};
use cryp_std::rand::{Rng, UniformRand};

use super::PrimeFieldOperations;
use cryp_std::fmt::Debug;

pub trait MontParameters<const N: usize>: 'static + Debug {
    // the type of limbs `b` for representing integers
    type Limb: Limb + Debug;
    // the prime modulus `p`
    const MODULUS: LimbInt<Self::Limb, N>;
    /// the constant `m' = -p^(-1) mod b`
    const MP: Self::Limb;
    /// the element `R^2 mod p`
    const R2 : LimbInt<Self::Limb, N>;
    // the element `R mod p`
    const R : LimbInt<Self::Limb, N>;
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

    /// Montgomery multiplication
    /// 
    /// Given `x, y`, the function computs x*y*R^-1 mod p, where R = b^N
    pub fn montgomery_mul(element: &LimbInt<P::Limb, N>, other: &LimbInt<P::Limb, N>) -> LimbInt<P::Limb, N> {
        // algorithm 14.36 in Handbook of Applied Cryptography

        // a = 0
        let mut a : LimbInt<P::Limb, N> = [P::Limb::ZERO; N].into();
        
        for i in 0..N {
            // u = (a_l[0] + element.limbs[i] * other.limbs[0]) * m' mod b
            let xiyo = element.limbs[i].mul_carry(other.limbs[0], P::Limb::ZERO).0;
            let s = a.limbs[0].add_carry(xiyo, P::Limb::NO).0;
            let u = s.mul_carry(P::MP, P::Limb::ZERO).0;

            // a = (a + x_i*y + u*m) / b
            let xiy = other.mul_by_limb(u);
            let um = P::MODULUS.mul_by_limb(u);

            // v = x_i*y + u*m
            let (v_0, c) = um.0.carrying_add(xiy.0, P::Limb::NO);
            let (v_1, _) = um.1.add_carry(xiy.1, c);

            // add v to a
            let (mut a_0, c) = a.carrying_add(v_0, P::Limb::NO);
            let (a_1, _) = v_1.add_carry(P::Limb::ZERO, c);

            // compute (a_0 + a_1 b^N)/b and assign to a
            a_0.limbs.copy_within(1..N, 0);
            a_0.limbs[N-1] = a_1;
            a = a_0;
        }

        // if a >= p, set a = a - p and return a
        // we use checked sub instead of comparison to get constant running time
        let (e, carry) = a.carrying_sub(P::MODULUS, P::Limb::NO);

        // carry = NO means a >=m
        if carry == P::Limb::NO {
            e
        } else {
            a
        }
    }

    /// Montgomery reduction
    /// 
    /// Given `x` a double-length integer, the function computes `x*R^-1 mod p`, where `R = b^N`
    pub fn montgomery_reduction(element: &(LimbInt<P::Limb, N>, LimbInt<P::Limb, N>)) -> LimbInt<P::Limb, N> {
        // algorithm 14.32 in Handbook of Applied Cryptography

        let (mut a_l, mut a_r) = (element.0, element.1);

        for i in 0..N {
            // u = a_i * m′ mod b
            let (u, _) = a_l.limbs[i].mul_carry(P::MP, P::Limb::ZERO);

            // a = a + u * m * b^i

            // umbi = u * m * b^i = m*(u*b^i)
            let umbi = P::MODULUS.mul_by_limb_shift(u, i);

            // add umbi to a
            let (a_0, c) = a_l.carrying_add(umbi.0, P::Limb::NO);
            let (a_1, _) = a_r.carrying_add(umbi.1, c);
            (a_l, a_r) = (a_0, a_1);
        }

        // A/b^n = a_r so that's the element we keep 

        // if a_r > p, set a_r = a_r - p and return a_r
        // we use checked sub instead of comparison to get constant running time
        let (e, carry) = a_r.carrying_sub(P::MODULUS, P::Limb::NO);
        if carry == P::Limb::NO {
            e
        } else {
            a_r
        }
    }
}

/// Montgomery representation of a prime field element
impl<const N: usize, P: MontParameters<N>> PrimeFieldOperations for MontgomeryOperations<N,  P> {
    type BigInt = LimbInt<P::Limb, N>;
    const MODULUS: Self::BigInt = P::MODULUS;

    #[inline]
    fn zero() -> Self::BigInt {
        Self::BigInt::zero()
    }

    #[inline]
    fn one() -> Self::BigInt {
        // return `R mod p`
        P::R
    }

    /// Checks if the element is zero.
    fn is_zero(element: &Self::BigInt) -> bool {
        let mut flag = false;
        for i in 0..N {
            flag = flag || element.limbs[i] != P::Limb::ZERO;
        }
        flag
    }

    fn as_int(element: &Self::BigInt) -> Self::BigInt {
        // converts the element from montgomery representation to the integer representation
        // from x*R mod p to x mod p by doing montgomery multiplication with 1.
        let one = Self::BigInt::one();
        Self::montgomery_mul(element, &one)
    }


    fn reduce(element: &Self::BigInt) -> Self::BigInt {
        // Given an integer x, computes x*R mod p by doing montgomery multiplication 
        // of `x*R^-1 mod p` with `R^2`
        let padded = (*element, *&Self::BigInt::zero());
        let mont_red = Self::montgomery_reduction(&padded);

        Self::montgomery_mul(&mont_red, &P::R2)
    }

    fn rand<R: Rng + ?Sized>(rng: &mut R) -> Self::BigInt {
        // Takes a random collection of limbs and reduces it modulo the prime
        let mut res = [P::Limb::ZERO; N];

        unimplemented!()
    }

    fn add_assign(lhs: &mut Self::BigInt, other: &Self::BigInt) {
        let (d, c_1) = lhs.carrying_add(*other, P::Limb::NO);

        let (e, c_2) = d.carrying_sub(P::MODULUS, P::Limb::NO);

        if c_1 == c_2 {
            *lhs = e;
        } else {
            *lhs = d;
        }
    }

    fn sub_assign(lhs: &mut Self::BigInt, other: &Self::BigInt) {
        let (d, c_1) = lhs.carrying_sub(*other, P::Limb::NO);

        let (e, c_2) = d.carrying_add(P::MODULUS, P::Limb::NO);

        if c_1 == c_2 {
            *lhs = e;
        } else {
            *lhs = d;
        }
    }

    fn mul_assign(lhs: &mut Self::BigInt, other: &Self::BigInt) {
        *lhs = Self::montgomery_mul(&lhs, other)
    }

    fn inverse(element: &Self::BigInt) -> Option<Self::BigInt> {
        // Compute element^(p-2)

        // Set power to p-2
        let two = Self::one().carrying_add(Self::one(), P::Limb::NO).0;
        let field_power = P::MODULUS.carrying_sub(two, P::Limb::NO).0;

        let power = Self::as_int(&field_power);

        //Compute element^power
        let res = Self::exp(element, &power);

        if Self::is_zero(element) {
            None
        } else {
            Some(res)
        }
    }
}
