use super::coordinates::{Affine, ExtendedPoint};
use super::{CurveOperations, Field};

mod a_minus_one_unified;
mod general_unified;

pub use a_minus_one_unified::EdwardsAM1UnifiedOperations;

/// Twisted Edwards Curve parameters
///
///  ax2 + y2 = 1 + dx2y2
/// No assumptions on a and d.
pub trait TwistedEdwardsGeneral {
    type Field: Field;

    const A: Self::Field;
    const D: Self::Field;
}

/// Twisted Edwards Curve parameters for the case a=-1
///
///  -x2 + y2 = 1 + dx2y2
/// The operation used is the unified formulas (for this special case) from section 3.1. of the paper
/// "Twisted Edwards Curves Revisited" by Hisil, Wong, Carter, Dawson, and Dahab.
///  http://eprint.iacr.org/2008/522
pub trait TwistedEdwardsAM1 {
    type Field: Field;

    const D: Self::Field;
    const D2: Self::Field;

    fn verify() -> bool {
        Self::D2 == Self::D + Self::D
    }
}
