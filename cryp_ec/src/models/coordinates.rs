use super::Field;
use cryp_std::fmt::{Debug, Display};
use cryp_std::hash::{Hash, Hasher};

/// A trait for the coordinates of a point on an elliptic curve.
pub trait Coordinates:
    PartialEq + Eq + Display + Clone + Hash + Copy + Sized + Send + Sync + Debug + From<Self::Affine>
{
    type Field;
    type Affine;

    fn into_affine(&self) -> Option<Self::Affine>;
}

/// Standard affine coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Affine<F: Field> {
    pub x: F,
    pub y: F,
}

impl<F: Field> Display for Affine<F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Affine(x={}, y={})", self.x, self.y)
    }
}

// ---------------------------------------------
// Projective Point
// ---------------------------------------------

/// Standard projective coordinates
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct Projective<F: Field> {
    pub X: F,
    pub Y: F,
    pub Z: F,
}

impl<F: Field> Coordinates for Projective<F> {
    type Field = F;
    type Affine = Affine<F>;

    fn into_affine(&self) -> Option<Self::Affine> {
        if self.Z == F::zero() {
            return None;
        }
        let x = self.X / self.Z;
        let y = self.Y / self.Z;

        Some(Affine { x, y })
    }
}

impl<F: Field> PartialEq for Projective<F> {
    fn eq(&self, other: &Self) -> bool {
        (self.X * other.Z) == (other.X * self.Z) && (self.Y * other.Z) == (other.Y * self.Z)
    }
}

impl<F: Field> Eq for Projective<F> {}

impl<F: Field> Hash for Projective<F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.into_affine().hash(state);
    }
}

impl<F: Field> Display for Projective<F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Projective(X = {}, Y = {}, Z = {})",
            self.X, self.Y, self.Z
        )
    }
}

impl<F: Field> From<Affine<F>> for Projective<F> {
    fn from(affine: Affine<F>) -> Self {
        Projective {
            X: affine.x,
            Y: affine.y,
            Z: F::one(),
        }
    }
}

// ---------------------------------------------
// Extended Point (for Twisted Edwards)
// ---------------------------------------------

///  Extended Twisted Edwards Coordinates
///
/// x = X/Z , y = Y/Z, T = XY/Z
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct ExtendedPoint<F: Field> {
    pub X: F,
    pub Y: F,
    pub T: F,
    pub Z: F,
}

impl<F: Field> Hash for ExtendedPoint<F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.into_affine().hash(state);
    }
}

impl<F: Field> PartialEq for ExtendedPoint<F> {
    fn eq(&self, other: &Self) -> bool {
        (self.X * other.Z) == (other.X * self.Z) && (self.Y * other.Z) == (other.Y * self.Z)
    }
}

impl<F: Field> Eq for ExtendedPoint<F> {}

impl<F: Field> Coordinates for ExtendedPoint<F> {
    type Field = F;
    type Affine = Affine<F>;

    fn into_affine(&self) -> Option<Self::Affine> {
        if self.Z == F::zero() {
            return None;
        }
        let x = self.X / self.Z;
        let y = self.Y / self.Z;

        Some(Affine { x, y })
    }
}

impl<F: Field> Display for ExtendedPoint<F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "ExtendedPoint(X = {}, Y = {}, T = {}, Z = {})",
            self.X, self.Y, self.T, self.Z
        )
    }
}

impl<F: Field> From<Affine<F>> for ExtendedPoint<F> {
    fn from(affine: Affine<F>) -> Self {
        ExtendedPoint {
            X: affine.x,
            Y: affine.y,
            T: affine.x * affine.y,
            Z: F::one(),
        }
    }
}

// ---------------------------------------------
// Jacobian Point
// ---------------------------------------------

#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct JacobianPoint<F: Field> {
    pub X: F,
    pub Y: F,
    pub Z: F,
}

impl<F: Field> Coordinates for JacobianPoint<F> {
    type Field = F;
    type Affine = Affine<F>;

    fn into_affine(&self) -> Option<Self::Affine> {
        if self.Z == F::zero() {
            return None;
        }
        let x = self.X / (self.Z.square());
        let y = self.Y / (self.Z.exp(3));

        Some(Affine { x, y })
    }
}

impl<F: Field> PartialEq for JacobianPoint<F> {
    fn eq(&self, other: &Self) -> bool {
        self.X * other.Z.square() == other.X * self.Z.square()
            && self.Y * other.Z.exp(3) == other.Y * self.Z.exp(3)
    }
}

impl<F: Field> Eq for JacobianPoint<F> {}

impl<F: Field> Hash for JacobianPoint<F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.into_affine().hash(state);
    }
}

impl<F: Field> Display for JacobianPoint<F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "JacobianPoint(X = {}, Y = {}, Z = {})",
            self.X, self.Y, self.Z
        )
    }
}

impl<F: Field> From<Affine<F>> for JacobianPoint<F> {
    fn from(affine: Affine<F>) -> Self {
        JacobianPoint {
            X: affine.x,
            Y: affine.y,
            Z: F::one(),
        }
    }
}
