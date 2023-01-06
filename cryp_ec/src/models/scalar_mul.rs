//! Scalar multiplication algorithms.
//!
//!
//! This module contains implementations of scalar multiplication algorithms for the
//! `CurveOperations` trait. This algorithm
//! will be used as default scalar multiplication and multi-scalar multiplication.
//!

use super::CurveOperations;

use super::Coordinates;
use core::borrow::Borrow;
use cryp_alg::Integer;
use cryp_alg::PrimeField;

pub struct ScalarMul;

pub struct VariableBaseMSM;
pub struct FixedBaseMSM;

impl ScalarMul {
    /// An implementation of the double-and-add algorithm for scalar multiplication.
    ///
    pub fn double_and_add<C: CurveOperations>(base: &C::Point, scalar: &impl Integer) -> C::Point {
        let mut res = C::identity();
        let mut base = *base;
        for &bit in scalar.into_bits_be() {
            if bit {
                C::add_in_place(&mut res, &base);
            }
            C::double_in_place(&mut base);
        }
        res
    }

    /// An implementation of the Montgomery ladder algorithm for scalar multiplication.
    ///
    /// This has the advantage of being constant-time.
    pub fn montgomery_ladder<C: CurveOperations>(
        base: &C::Point,
        scalar: &impl Integer,
    ) -> C::Point {
        let mut res = C::identity();
        let mut base = *base;
        for &bit in scalar.into_bits_be() {
            if bit {
                C::add_in_place(&mut res, &base);
                C::double_in_place(&mut base);
            } else {
                C::add_in_place(&mut base, &res);
                C::double_in_place(&mut res);
            }
        }
        res
    }
}

impl VariableBaseMSM {
    pub fn msm_simple<C: CurveOperations, I, J, N>(bases: I, scalars: J) -> C::Point
    where
        I: IntoIterator,
        I::Item: Borrow<C::Point>,
        J: IntoIterator,
        J::Item: Borrow<N>,
        N: PrimeField,
    {
        let mut res = C::identity();
        for (base, scalar) in bases.into_iter().zip(scalars.into_iter()) {
            C::add_in_place(
                &mut res,
                &ScalarMul::montgomery_ladder::<C>(base.borrow(), &scalar.borrow().as_int()),
            );
        }
        res
    }
}
