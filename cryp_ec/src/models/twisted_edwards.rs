use super::{CurveOperations, Affine, ExtendedPoint, Field};

/// Twisted Edwards Curve parameters
///
///  ax2 + y2 = 1 + dx2y2
/// No assumptions on a and d. 
/// 
/// The operation used is the unified formulas from section 3.1. of the paper
/// "Twisted Edwards Curves Revisited" by Hisil, Wong, Carter, Dawson, and Dahab.
///  http://eprint.iacr.org/2008/522
pub trait TwistedEdwardsGeneralUnified {
    type Field : Field;
    type Scalar : Field;
    
    const A : Self::Field;
    const D : Self::Field;
}

/// Twisted Edwards Curve parameters for the case a=-1
///
///  -x2 + y2 = 1 + dx2y2
/// The operation used is the unified formulas (for this special case) from section 3.1. of the paper
/// "Twisted Edwards Curves Revisited" by Hisil, Wong, Carter, Dawson, and Dahab.
///  http://eprint.iacr.org/2008/522
pub trait TwistedEdwardsUnifiedAM1 {
    type Field : Field;
    type Scalar : Field;

    const D : Self::Field; 
    const D2 : Self::Field;

    fn verify() -> bool {
        Self::D2 == Self::D + Self::D
    }
}

/// A wrapper for Twisted Edwards Curve parameters
pub struct EdwardsAM1<P: TwistedEdwardsUnifiedAM1> {
    _marker: cryp_std::marker::PhantomData<P>,
}

impl<P: TwistedEdwardsUnifiedAM1> CurveOperations for EdwardsAM1<P> {
    type Field = P::Field;
    type Scalar = P::Scalar;
    type Point = ExtendedPoint<P::Field>;
    type Affine = Affine<P::Field>;

    #[allow(non_snake_case)]
    fn add_in_place(lhs: &mut Self::Point, rhs: &Self::Point) {
        let (X1, Y1, Z1, T1) = (lhs.X, lhs.Y, lhs.Z, lhs.T);
        let (X2, Y2, Z2, T2) = (rhs.X, rhs.Y, rhs.Z, rhs.T);

        //  Formulas from 2008 Hisil--Wong--Carter--Dawson, http://eprint.iacr.org/2008/522, Section 3.1 
        let A = (Y1 - X1) * (Y2 - X2);
        let B = (Y1 + X1) * (Y2 + X2);
        let C = T1 * P::D2 * T2;
        let D = Z1*Self::Field::from(2u32) * Z2;
        let E = B - A;
        let F = D - C;
        let G = D + C;
        let H = B + A;
        lhs.X = E * F;
        lhs.Y = G * H;
        lhs.T = E * H;
        lhs.Z = F * G;
    }

    #[allow(non_snake_case)]
    fn add_affine_in_place(lhs: &mut Self::Point, rhs: &Self::Affine) {
        let (X1, Y1, Z1, T1) = (lhs.X, lhs.Y, lhs.Z, lhs.T);
        let (X2, Y2, T2) = (rhs.x, rhs.y, rhs.x * rhs.y);

        let A = (Y1 - X1) * (Y2 - X2);
        let B = (Y1 + X1) * (Y2 + X2);
        let C = T1 * P::D2 * T2;
        let D = Z1*Self::Field::from(2u32);
        let E = B - A;
        let F = D - C;
        let G = D + C;
        let H = B + A;
        lhs.X = E * F;
        lhs.Y = G * H;
        lhs.T = E * H;
        lhs.Z = F * G;
    }

    fn double_in_place(point : &mut Self::Point) {
        let rhs = ExtendedPoint {X : point.X, Y : point.Y, Z: point.Z, T: point.T};
        Self::add_in_place(point, &rhs);
    }

    fn mul_in_place(point : &mut Self::Point, scalar: &Self::Scalar) {
        
    }
}

