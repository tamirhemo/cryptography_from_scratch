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



use cryp_alg::{Field, Group};

mod short_weierstrass;
mod twisted_edwards;
mod coordinates;

pub(crate) use coordinates::{Affine, Projective, ExtendedPoint, JacobianPoint};

pub trait Curve {
    type Parameters;
    type Point;

}

/// A trait for the coordinates of a point on an elliptic curve.
pub trait Coordinates: PartialEq + Eq + Clone + Copy + Sized {
    type Field;
    type Affine;

    fn into_affine(&self) -> Self::Affine;
}


/// A trait for the operations on an elliptic curve.
pub trait CurveOperations {
    type Field : Field;
    type Scalar: Field;
    type Affine;
    type Point : Coordinates<Field = Self::Field, Affine = Self::Affine>;

    /// Adds a point to the given point in place.
    fn add_in_place(lhs: &mut Self::Point, rhs: &Self::Point);
    /// Adds a point in affine representation to the given point in place.
    fn add_affine_in_place(lhs: &mut Self::Point, rhs: &Self::Affine);
    /// Doubles the point in place.
    fn double_in_place(point : &mut Self::Point);
    /// Multiplies the point by the given scalar in place.
    fn mul_in_place(point : &mut Self::Point, scalar: &Self::Scalar);

    /// Doubles the given point.
    fn double(point : &Self::Point) -> Self::Point {
        let mut result = *point;
        Self::double_in_place(&mut result);
        result
    }

    fn mul(point : &Self::Point, scalar: &Self::Scalar) -> Self::Point {
        let mut result = *point;
        Self::mul_in_place(&mut result, scalar);
        result
    }
}



