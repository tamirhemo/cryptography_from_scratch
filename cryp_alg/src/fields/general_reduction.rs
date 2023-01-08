use crate::biginteger::{Limb, LimbInt};
use cryp_std::rand::{Rng, UniformRand};

use super::PrimeFieldOperations;
use cryp_std::fmt::Debug;

/// A trait that allows the implementation of field operations when a good reduction algorithm is available.
///
/// This is useful for e.g. primes with special form that can be reduced efficiently.
pub trait GeneralReduction<const N: usize>: 'static + Debug {
    type Limb: Limb + Debug;
    const MODULUS: LimbInt<Self::Limb, N>;

    /// Reduction mod the prime for a general double-length integer.
    ///
    /// This function is used in the implementation of the field operations.
    fn reduction(
        element: &(LimbInt<Self::Limb, N>, LimbInt<Self::Limb, N>),
    ) -> LimbInt<Self::Limb, N>;
}

#[derive(Debug)]
pub struct GeneralReductionOperations<const N: usize, P: GeneralReduction<N>> {
    _marker: cryp_std::marker::PhantomData<P>,
}

impl<const N: usize, P: GeneralReduction<N>> PrimeFieldOperations
    for GeneralReductionOperations<N, P>
{
    type BigInt = LimbInt<P::Limb, N>;
    const MODULUS: Self::BigInt = P::MODULUS;

    #[inline]
    fn zero() -> Self::BigInt {
        Self::BigInt::zero()
    }

    #[inline]
    fn one() -> Self::BigInt {
        Self::BigInt::one()
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
        *element
    }

    fn reduce(element: &Self::BigInt) -> Self::BigInt {
        // Using the given reduction algorithm
        let padded = (*element, *&Self::BigInt::zero());
        P::reduction(&padded)
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

        let (e, _) = d.carrying_add(P::MODULUS, P::Limb::NO);

        if c_1 == P::Limb::NO {
            *lhs = d;
        } else {
            *lhs = e;
        }
    }

    fn mul_assign(lhs: &mut Self::BigInt, other: &Self::BigInt) {
        let double = lhs.carrying_mul(*other, Self::BigInt::zero());
        *lhs = P::reduction(&double);
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
