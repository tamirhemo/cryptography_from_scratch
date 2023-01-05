use super::{CurveOperations,
          Field, JacobianPoint, Affine};


/// A trait for the parameters of a short Weierstrass curve.
/// 
/// The curve is defined by the equation `y^2 = x^3 + Ax + B`.
pub trait ShortWeierstrass {
    type Field: Field;
    type Scalar: Field;

    const A: Self::Field;
    const B: Self::Field;
}

/// Formulas from Faster addition and doubling on elliptic curves
/// 
/// https://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html#addition-add-2007-bl
pub struct ShortWeierstrassOperations<P: ShortWeierstrass> {
    _parameters: cryp_std::marker::PhantomData<P>,
}

impl<P : ShortWeierstrass> CurveOperations for ShortWeierstrassOperations<P> {
    type Field = P::Field;
    type Scalar = P::Scalar;
    type Point = JacobianPoint<P::Field>;
    type Affine = Affine<P::Field>;

    /// Adds a point to the given point in place.
    #[allow(non_snake_case)]
    fn add_in_place(lhs: &mut Self::Point, rhs: &Self::Point) {
        let (X1, Y1, Z1) = (lhs.X, lhs.Y, lhs.Z);
        let (X2, Y2, Z2) = (rhs.X, rhs.Y, rhs.Z);

        let Z1Z1 = Z1.square();
        let Z2Z2 = Z2.square();
        let U1 = X1 * Z2Z2;
        let U2 = X2 * Z1Z1;
        let S1 = Y1 * Z2 * Z2Z2;
        let S2 = Y2 * Z1 * Z1Z1;
        let H = U2 - U1;
        let I = (H + H).square();
        let J = H * I;
        let r = Self::Field::from(2u32)*(S2 - S1);
        let V = U1 * I;
        lhs.X = r.square()-J-Self::Field::from(2u32)*V;
        lhs.Y = r*(V-lhs.X)-Self::Field::from(2u32)*S1*J;
        lhs.Z = ((Z1+Z2).square()-Z1Z1-Z2Z2)*H;
    }
    /// Adds a point in affine representation to the given point in place.
    #[allow(non_snake_case)]
    fn add_affine_in_place(lhs: &mut Self::Point, rhs: &Self::Affine) {
        let (X1, Y1, Z1) = (lhs.X, lhs.Y, lhs.Z);
        let (x2, y2) = (rhs.x, rhs.y);

        // Formulas from: Faster addition and doubling on elliptic curves
        // https://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html#addition-madd-2007-bl
        let Z1Z1 = Z1.square();
        let U2 = x2 * Z1Z1;
        let S2 = y2 * Z1 * Z1Z1;
        let H = U2 - X1;
        let HH = H.square();
        let I = HH + HH;
        let J = H * I;
        let r = Self::Field::from(2u32)*(S2 - Y1);
        let V = X1 * I;
        lhs.X = r.square()-J-Self::Field::from(2u32)*V;
        lhs.Y = r*(V-lhs.X)-Self::Field::from(2u32)*Y1*J;
        lhs.Z = (Z1+H).square()-Z1Z1-HH;
    }
    /// Doubles the point in place.
    #[allow(non_snake_case)]
    fn double_in_place(point : &mut Self::Point) {

        let (X, Y, Z) = (point.X, point.Y, point.Z);

        // https://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html#doubling-dbl-2007-bl
        let XX = X.square();
        let YY = Y.square();
        let YYYY = YY.square();
        let ZZ = Z.square();
        let S = Self::Field::from(2u32)*(X+YY).square()-XX-YYYY;
        let M = Self::Field::from(3u32)*XX+P::A*ZZ.square();
        let T = M.square()-Self::Field::from(2u32)*S;
        point.X = T;
        point.Y = M*(S-T)-Self::Field::from(8u32)*YYYY;
        point.Z = (Y+Z).square()-YY-ZZ;
    }

    /// Multiplies the point by the given scalar in place.
    fn mul_in_place(point : &mut Self::Point, scalar: &Self::Scalar) {}
}


