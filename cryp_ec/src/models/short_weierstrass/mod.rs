use super::coordinates::{Affine, JacobianPoint};
use super::CurveOperations;
use super::ff::*;

mod jacobian_general;

pub use jacobian_general::ShortWeierstrassOperations;

/// A trait for the parameters of a short Weierstrass curve.
///
/// The curve is defined by the equation `y^2 = x^3 + Ax + B`.
pub trait ShortWeierstrass {
    type Field: Field;

    const A: Self::Field;
    const B: Self::Field;
}
