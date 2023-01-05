use super::{Coordinates, Field};


/// Standard affine coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Affine<F: Field> {
    pub x: F,
    pub y: F,
}

/***************
Projective Point
****************/

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

    fn into_affine(&self) -> Self::Affine {
        let x = self.X/self.Z;
        let y = self.Y/self.Z;

        Affine { x, y }
    }
}

impl<F: Field> PartialEq for Projective<F> {
    fn eq(&self, other: &Self) -> bool {
        (self.X * other.Z) == (other.X * self.Z) && (self.Y * other.Z) == (other.Y * self.Z)
    }
}

impl <F: Field> Eq for Projective<F> {}


/************************************
Extended Point (for Twisted Edwards)
*************************************/

///  Extended Twisted Edwards Coordinates
/// 
/// x = X/Z , y = Y/Z, T = XY/Z
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct ExtendedPoint<F: Field> {
    pub X: F,
    pub Y: F,
    pub Z: F,
    pub T: F,
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

    fn into_affine(&self) -> Self::Affine {
        let x = self.X/self.Z;
        let y = self.Y/self.Z;

        Affine { x, y }
    }
}


/************************************
Jacobian Point 
*************************************/

#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct JacobianPoint<F: Field> {
    pub X: F,
    pub Y: F,
    pub Z: F,
}

impl<F: Field> PartialEq for JacobianPoint<F> {
    fn eq(&self, other: &Self) -> bool {
        self.X*other.Z.square() == other.X*self.Z.square() && self.Y*other.Z.exp(3) == other.Y*self.Z.exp(3)
    }
}

impl<F: Field> Eq for JacobianPoint<F> {}

impl<F: Field> Coordinates for JacobianPoint<F> {
    type Field = F;
    type Affine = Affine<F>;

    fn into_affine(&self) -> Self::Affine {
        let x = self.X/(self.Z.square());
        let y = self.Y/(self.Z.exp(3));

        Affine { x, y }
    }
}