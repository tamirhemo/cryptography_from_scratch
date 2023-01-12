use super::*;
use crate::models::Coordinates;

/// An interface for a prime order subgroup of an elliptic curve.
///
/// This trait allows the user to define a prime order subgroup of an elliptic curve
/// defined via the `CurveOperations` trait.
///
pub trait PrimeSubGroupConfig: CurveOperations + Debug + Sized + 'static + Eq + PartialEq {
    /// Finite field with the same order as the subgroup.
    type ScalarField: PrimeField;

    /// The cofactor of the curve
    /// This is the number of points on the curve divided by the order of the group.
    ///
    /// Cofactors are usually small, so we use a `u16` to represent them.
    const COFACTOR: u32;

    /// Gives a generator of the group.
    fn generator<R: Rng>(rng: Option<&mut R>) -> Self::Affine;

    /// Gives a vector of generators of the group of size `n`.
    ///
    /// The generators should be independent in the sense that the mutual
    /// discrete logarithms are not known.
    fn batch_generators<R: Rng>(n: usize, rng: &mut R) -> Vec<Self::Affine>;

    /// Scalar multiplication in constant time.
    ///
    /// Default implementation uses the montgomery ladder algorithm.
    fn scalar_mul(base: &Self::Point, scalar: &Self::ScalarField) -> Self::Point {
        let base_int = scalar.as_int();
        scalar_mul::ScalarMul::montgomery_ladder::<Self>(base, &base_int)
    }

    /// A multiplication with a secret scalar.
    ///
    /// This operation should run in constant time.
    ///
    /// The default implementation converts to projective and performs scalar_mul.
    fn scalar_mul_pub(base: &Self::Affine, scalar: &Self::ScalarField) -> Self::Point {
        let point = (*base).into();
        Self::scalar_mul(&point, scalar)
    }

    /// Multi-scalar multiplication with a vector of secret scalars.
    ///
    /// The iteretors should be of the same length.
    fn msm<I, J>(bases: I, scalars: J) -> Self::Point
    where
        I: IntoIterator,
        I::Item: Borrow<Self::Point>,
        J: IntoIterator,
        J::Item: Borrow<Self::ScalarField>,
    {
        scalar_mul::VariableBaseMSM::msm_simple::<Self, _, _, _>(bases, scalars)
    }

    /// Multi-scalar multiplication with a vector of secret scalars.
    ///
    /// The iteretors should be of the same length.
    fn msm_pub<I, J>(bases: I, scalars: J) -> Self::Point
    where
        I: IntoIterator,
        I::Item: Borrow<Self::Affine>,
        J: IntoIterator,
        J::Item: Borrow<Self::ScalarField>,
    {
        let points = bases.into_iter().map(|b| Self::Point::from(*b.borrow()));
        Self::msm(points, scalars)
    }
}

impl<T> PrimeGroupConfig for T
where
    T: PrimeSubGroupConfig,
{
    type Public = T::Affine;
    type ScalarField = T::ScalarField;

    fn is_valid(input: &Self::Public) -> bool {
        // An element is valid if input^MODULUS = identity
        let power = -T::ScalarField::one();
        let base = (*input).into();

        let mut base_power = T::scalar_mul(&base, &power);
        T::add_in_place(&mut base_power, &base);

        base_power == T::identity()
    }

    fn as_public(input: &Self::Point) -> Option<Self::Public> {
        input.into_affine()
    }

    fn add_public_in_place(lhs: &mut Self::Point, rhs: &Self::Public) {
        T::add_affine_in_place(lhs, rhs)
    }

    fn generator<R: Rng>(rng: Option<&mut R>) -> Self::Public {
        Self::generator(rng)
    }

    fn batch_generators<R: Rng>(n: usize, rng: &mut R) -> Vec<Self::Public> {
        Self::batch_generators(n, rng)
    }

    fn scalar_mul(base: &Self::Point, scalar: &Self::ScalarField) -> Self::Point {
        T::scalar_mul(base, scalar)
    }

    fn scalar_mul_pub(base: &Self::Affine, scalar: &Self::ScalarField) -> Self::Point {
        T::scalar_mul_pub(base, scalar)
    }

    fn msm<I, J>(bases: I, scalars: J) -> Self::Point
    where
        I: IntoIterator,
        I::Item: Borrow<Self::Point>,
        J: IntoIterator,
        J::Item: Borrow<Self::ScalarField>,
    {
        T::msm(bases, scalars)
    }

    fn msm_pub<I, J>(bases: I, scalars: J) -> Self::Point
    where
        I: IntoIterator,
        I::Item: Borrow<Self::Public>,
        J: IntoIterator,
        J::Item: Borrow<Self::ScalarField>,
    {
        T::msm_pub(bases, scalars)
    }
}
