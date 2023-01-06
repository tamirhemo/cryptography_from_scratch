use crate::biginteger::{Limb, LimbInt};
use cryp_std::rand::{Rng, UniformRand};

use super::PrimeFieldOperations;
use cryp_std::fmt::Debug;

pub trait Modulus<const N: usize>: 'static + Debug {
    type Limb: Limb + Debug;
    const MODULUS: LimbInt<Self::Limb, N>;
}

#[derive(Debug)]
pub struct Montgomery<const N: usize, P: Modulus<N>> {
    element: LimbInt<P::Limb, N>,
    _marker: cryp_std::marker::PhantomData<P>,
}

// Implement montgomery multiplication, addition, and modular reduction
impl<const N: usize, P: Modulus<N>> PrimeFieldOperations for Montgomery<N, P> {
    type BigInt = LimbInt<P::Limb, N>;
    const MODULUS: Self::BigInt = P::MODULUS;

    fn zero() -> Self::BigInt {
        [P::Limb::ZERO; N].into()
    }

    fn one() -> Self::BigInt {
        let mut limbs = [P::Limb::ZERO; N];
        limbs[0] = P::Limb::ONE;
        limbs.into()
    }

    /// Checks if the element is zero.
    fn is_zero(element: &Self::BigInt) -> bool {
        let mut flag = false;
        for i in 0..N {
            flag = flag || element.limbs[i] != P::Limb::ZERO;
        }
        flag
    }

    fn reduce(element: &Self::BigInt) -> Self::BigInt {
        [P::Limb::ZERO; N].into()
    }

    fn rand<R: Rng + ?Sized>(rng: &mut R) -> Self::BigInt {
        // Takes a random collection of limbs and reduces it modulo the prime
        let mut res = [P::Limb::ZERO; N];

        for i in 0..N {
            res[i] = P::Limb::rand(rng);
        }
        Self::reduce(&res.into())
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

    fn mul_assign(lhs: &mut Self::BigInt, other: &Self::BigInt) {}

    fn inverse(element: &Self::BigInt) -> Option<Self::BigInt> {
        None
    }
}
