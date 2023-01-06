//! Coordinates for elliptic curves and implemntations of the curve operations.
//!
//! This module contains traits for the different coordinate systems used in elliptic curves and
//! implementations of the curve operations on them. The module contains implementations
//! of some of the most common curves coordinates and curve operations currently in use.
//!
//! We currently support the following coordinate systems:
//!
//! * Short Weierstrass: `short_weierstrass` module
//! Curves of the form `y^2 = x^3 + Ax + B`
//! where A, B are non-zero constants.
//!
//! * Twisted Edwards
//! Curves of the form `Ax^2 + y^2 = 1 + Dx^2y^2`
//! where A, D are constants.
//!

use cryp_std::{
    rand::{Rng, UniformRand},
    vec::Vec,
    hash::Hash,
};

use cryp_alg::{Field, Group, PrimeField};

mod coordinates;
mod primegroup;
mod scalar_mul;
mod short_weierstrass;
mod twisted_edwards;

pub(crate) use coordinates::Coordinates;

/// A trait for the operations on an elliptic curve.
pub trait CurveOperations  {
    type Field: Field;
    type Affine : Clone + Copy + PartialEq + Eq + Hash + Send + Sync + Into<Self::Point>;
    type Point: Coordinates<Field = Self::Field, Affine = Self::Affine>;

    const UNIFIED: bool;

    /// Identity point of the curve.
    fn identity() -> Self::Point;
    /// Negates the point in place.
    fn neg_in_place(point: &mut Self::Point);
    /// Adds a point to the given point in place.
    fn add_in_place(lhs: &mut Self::Point, rhs: &Self::Point);
    /// Adds a point in affine representation to the given point in place.
    fn add_affine_in_place(lhs: &mut Self::Point, rhs: &Self::Affine);
    /// Doubles the point in place.
    fn double_in_place(point: &mut Self::Point);
}
