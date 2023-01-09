//! Coordinates for elliptic curves and implemntations of the curve operations.
//!
//! This module contains traits for the different coordinate systems used in elliptic curves and
//! implementations of the curve operations on them. The module contains implementations
//! of some of the most common curves coordinates and curve operations currently in use.
//!
//! We currently support the following coordinate systems:
//!
//! - Short Weierstrass: `short_weierstrass` module
//! Curves of the form `y^2 = x^3 + Ax + B`
//! where A, B are non-zero constants.
//!
//! - Twisted Edwards
//! Curves of the form `Ax^2 + y^2 = 1 + Dx^2y^2`
//! where A, D are constants.
//!
//!
//! Curve operations are implemented through the `CurveOperations` trait. These usually depend
//! on the specific model of the curve and the coordinates. For example,
//!  it is usually cheaper to add an affine point to a projective point than it is to add two
//! projective points. Moreover, the same underlying curve model can have different operations
//! with different tradeoffs implemented or perhaps differ in assumptions about the parameters.
//! For example, the twisted Edwards curve operations are more efficient
//! if one assumes that `A` is equal to `-1`.
//!
//! The `primegroup` module contains the `PrimeGroupConfig` trait which encodes the information
//! needed to define a prime order group related to the elliptic curve. This can be a subroup
//! of the elliptic curve, in which case the `PrimeSubGroupConfig` trait can be used, and each
//! type implementing it automatically implements the `PrimeGroupConfig` trait.
//!
//!

use cryp_std::{hash::Hash, rand::Rng, vec::Vec};

use cryp_alg::{Field, PrimeField};

mod coordinates;
mod primegroup;
mod scalar_mul;
mod short_weierstrass;
mod twisted_edwards;

pub use coordinates::{Affine, Coordinates, ExtendedPoint, JacobianPoint, Projective};
pub use primegroup::{GroupEC, PrimeGroupConfig, PrimeSubGroupConfig, PublicEC};
pub use short_weierstrass::ShortWeierstrass;
pub use twisted_edwards::{EdwardsAM1UnifiedOperations, TwistedEdwardsAM1};

/// A trait for the operations on an elliptic curve.
///
/// This trait defines operations that might have specific formulas
/// for different curve models and coordinate systems.
pub trait CurveOperations {
    type Field: Field;
    type Affine: Clone + Copy + PartialEq + Eq + Hash + Send + Sync + Into<Self::Point>;
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
