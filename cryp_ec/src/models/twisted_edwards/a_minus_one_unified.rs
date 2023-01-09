use super::*;

/// A wrapper for Twisted Edwards Curve parameters
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EdwardsAM1UnifiedOperations<P: TwistedEdwardsAM1> {
    _marker: cryp_std::marker::PhantomData<P>,
}

impl<P: TwistedEdwardsAM1> CurveOperations for EdwardsAM1UnifiedOperations<P> {
    type Field = P::Field;
    type Point = ExtendedPoint<P::Field>;
    type Affine = Affine<P::Field>;

    const UNIFIED: bool = true;

    fn identity() -> Self::Point {
        ExtendedPoint {
            X: P::Field::zero(),
            Y: P::Field::one(),
            T: P::Field::zero(),
            Z: P::Field::one(),
        }
    }
    fn neg_in_place(point: &mut Self::Point) {
        point.X = -point.X;
        point.T = -point.T;
    }

    #[allow(non_snake_case)]
    fn add_in_place(lhs: &mut Self::Point, rhs: &Self::Point) {
        let (X1, Y1, Z1, T1) = (lhs.X, lhs.Y, lhs.Z, lhs.T);
        let (X2, Y2, Z2, T2) = (rhs.X, rhs.Y, rhs.Z, rhs.T);

        //  Formulas from 2008 Hisil--Wong--Carter--Dawson, http://eprint.iacr.org/2008/522, Section 3.1
        let A = (Y1 - X1) * (Y2 - X2);
        let B = (Y1 + X1) * (Y2 + X2);
        let C = (P::D.double()) * T1 * T2;
        let D = Z1.double() * Z2;
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
        let C = T1 * (P::D.double()) * T2;
        let D = Z1.double();
        let E = B - A;
        let F = D - C;
        let G = D + C;
        let H = B + A;
        lhs.X = E * F;
        lhs.Y = G * H;
        lhs.T = E * H;
        lhs.Z = F * G;
    }

    fn double_in_place(point: &mut Self::Point) {
        let rhs = ExtendedPoint {
            X: point.X,
            Y: point.Y,
            Z: point.Z,
            T: point.T,
        };
        Self::add_in_place(point, &rhs);
    }
}
