use super::*;
use core::borrow::Borrow;
use cryp_alg::{Group, PrimeGroup};
use cryp_std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

mod subgroup;

pub use subgroup::PrimeSubGroupConfig;

/// A general interface for a prime order group using elliptic curve operations.
///
/// The group can come from a prime order subgroup (as is most common) or even
/// be a quotient group such as in the case of Ristretto.
pub trait PrimeGroupConfig: CurveOperations + Sized + 'static + PartialEq + Eq {
    type Public: Into<Self::Point> + Send + Sync + Hash + PartialEq + Eq + Clone + Copy;
    /// Finite field with the same order as the subgroup.
    type ScalarField: PrimeField;

    /// Gives a generator of the group.
    fn generator(rng: Option<impl Rng>) -> Self::Public;

    /// Gives a random element of the group.
    fn rand(rng: impl Rng) -> Self::Public;

    /// Verifies that the `Public` element is valid group element.
    fn is_valid(input: &Self::Public) -> bool;

    /// Attempts to convert a `Point` element to a `Public`.
    fn as_public(input: &Self::Point) -> Option<Self::Public>;

    /// Adding a point to a point in the public representation.
    fn add_public_in_place(lhs: &mut Self::Point, rhs: &Self::Public);

    /// Gives a vector of generators of the group of size `n`.
    ///
    /// The generators should be independent in the sense that the mutual
    /// discrete logarithms are not known.
    fn batch_generators(n: usize, rng: Option<impl Rng>) -> Vec<Self::Public>;

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
    /// The default implementation uses the montgomery ladder algorithm.
    fn scalar_mul_pub(base: &Self::Public, scalar: &Self::ScalarField) -> Self::Point {
        let point = (*base).into();
        Self::scalar_mul(&point, scalar)
    }

    /// Multi-scalar multiplication in constant time.
    ///
    /// The iteretors should be of the same length.
    fn msm<I, J>(bases: I, scalars: J) -> Self::Point
    where
        I: IntoIterator,
        I::Item: Borrow<Self::Point>,
        J: IntoIterator,
        J::Item: Borrow<Self::ScalarField>;

    /// Multi-scalar multiplication with a vector of secret scalars.
    ///
    /// The default implementation converts the elements to `Point` and uses msm.
    fn msm_pub<I, J>(bases: I, scalars: J) -> Self::Point
    where
        I: IntoIterator,
        I::Item: Borrow<Self::Public>,
        J: IntoIterator,
        J::Item: Borrow<Self::ScalarField>;
}

#[derive(Debug)]
pub struct GroupEC<P: PrimeGroupConfig> {
    point: P::Point,
}

#[derive(Debug)]
pub struct PublicEC<P: PrimeGroupConfig> {
    point: P::Public,
}

impl<P: PrimeGroupConfig> From<PublicEC<P>> for GroupEC<P> {
    fn from(public: PublicEC<P>) -> Self {
        Self {
            point: public.into_point(),
        }
    }
}

// -----------------------------------------

// Group and PrimeGroup traits for GroupEC

// -----------------------------------------

impl<P: PrimeGroupConfig> Group for GroupEC<P> {
    fn identity() -> Self {
        Self {
            point: P::identity(),
        }
    }

    fn double_in_place(&mut self) {
        P::double_in_place(&mut self.point);
    }
}

impl<P: PrimeGroupConfig> PrimeGroup for GroupEC<P> {
    type ScalarField = P::ScalarField;
    type Public = PublicEC<P>;

    fn is_valid(input: &Self::Public) -> bool {
        P::is_valid(&input.point)
    }

    fn as_public(&self) -> Option<Self::Public> {
        P::as_public(&self.point).map(PublicEC::new)
    }

    fn generator(rng: Option<impl Rng>) -> Self::Public {
        PublicEC::new(P::generator(rng))
    }

    fn batch_generators(n: usize, rng: Option<impl Rng>) -> Vec<Self::Public> {
        P::batch_generators(n, rng)
            .into_iter()
            .map(PublicEC::new)
            .collect()
    }

    fn msm<I, J>(bases: I, scalars: J) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<Self::Public>,
        J: IntoIterator,
        J::Item: Borrow<<Self as PrimeGroup>::ScalarField>,
    {
        let bases = bases.into_iter().map(|b| b.borrow().point);
        let point = P::msm_pub(bases, scalars);
        Self::new(point)
    }
}

impl<P: PrimeGroupConfig> PublicEC<P> {
    pub fn new(point: P::Public) -> Self {
        Self { point }
    }

    pub fn into_point(self) -> P::Point {
        self.point.into()
    }
}

// -----------------------------------------

// Basic traits (Clone, Copy, Hash) for GroupEC and PublicEC

//------------------------------------------

impl<P: PrimeGroupConfig> Copy for GroupEC<P> {}

impl<P: PrimeGroupConfig> PartialEq for GroupEC<P> {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

impl<P: PrimeGroupConfig> Eq for GroupEC<P> {}

impl<P: PrimeGroupConfig> GroupEC<P> {
    pub fn new(point: P::Point) -> Self {
        Self { point }
    }
}

impl<P: PrimeGroupConfig> Hash for GroupEC<P> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.point.hash(state);
    }
}

impl<P: PrimeGroupConfig> Display for GroupEC<P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "GroupEC({})", self.point)
    }
}

impl<P: PrimeGroupConfig> Clone for GroupEC<P> {
    fn clone(&self) -> Self {
        Self {
            point: self.point.clone(),
        }
    }
}

impl<P: PrimeGroupConfig> Hash for PublicEC<P> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.point.hash(state);
    }
}

impl<P: PrimeGroupConfig> PartialEq for PublicEC<P> {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

impl<P: PrimeGroupConfig> Eq for PublicEC<P> {}

impl<P: PrimeGroupConfig> Clone for PublicEC<P> {
    fn clone(&self) -> Self {
        Self {
            point: self.point.clone(),
        }
    }
}

impl<P: PrimeGroupConfig> Copy for PublicEC<P> {}

// -----------------------------------------

// Implementing the operations as ops traits

// -----------------------------------------

impl<P: PrimeGroupConfig> AddAssign<&GroupEC<P>> for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    fn add_assign(&mut self, other: &GroupEC<P>) {
        P::add_in_place(&mut self.point, &other.point);
    }
}

impl<P: PrimeGroupConfig> AddAssign for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    fn add_assign(&mut self, other: GroupEC<P>) {
        P::add_in_place(&mut self.point, &other.point);
    }
}

impl<P: PrimeGroupConfig> Add<&GroupEC<P>> for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    type Output = GroupEC<P>;

    fn add(mut self, other: &GroupEC<P>) -> GroupEC<P> {
        self += other;
        self
    }
}

impl<P: PrimeGroupConfig> Add for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    type Output = GroupEC<P>;

    fn add(mut self, other: GroupEC<P>) -> GroupEC<P> {
        self += other;
        self
    }
}

impl<P: PrimeGroupConfig> Neg for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    type Output = GroupEC<P>;

    fn neg(mut self) -> GroupEC<P> {
        P::neg_in_place(&mut self.point);
        self
    }
}

impl<P: PrimeGroupConfig> SubAssign<&GroupEC<P>> for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    fn sub_assign(&mut self, other: &GroupEC<P>) {
        P::neg_in_place(&mut self.point);
        *self += other;
        P::neg_in_place(&mut self.point);
    }
}

impl<P: PrimeGroupConfig> SubAssign for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    fn sub_assign(&mut self, other: GroupEC<P>) {
        *self -= &other;
    }
}

impl<P: PrimeGroupConfig> Sub for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    type Output = GroupEC<P>;

    fn sub(mut self, other: GroupEC<P>) -> GroupEC<P> {
        self -= other;
        self
    }
}

impl<P: PrimeGroupConfig> Sub<&GroupEC<P>> for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    type Output = GroupEC<P>;

    fn sub(mut self, other: &GroupEC<P>) -> GroupEC<P> {
        self -= other;
        self
    }
}

impl<P: PrimeGroupConfig> iter::Sum for GroupEC<P> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::new(P::identity()), |acc, x| acc + x)
    }
}

impl<'a, P: PrimeGroupConfig> iter::Sum<&'a GroupEC<P>> for GroupEC<P> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a GroupEC<P>>,
    {
        iter.fold(Self::new(P::identity()), |acc, x| acc + x)
    }
}

impl<P: PrimeGroupConfig> Mul<&P::ScalarField> for GroupEC<P> {
    type Output = GroupEC<P>;

    fn mul(self, other: &P::ScalarField) -> GroupEC<P> {
        Self {
            point: P::scalar_mul(&self.point, other),
        }
    }
}

impl<P: PrimeGroupConfig> MulAssign<&P::ScalarField> for GroupEC<P> {
    fn mul_assign(&mut self, other: &P::ScalarField) {
        let res = *self * other;
        *self = res;
    }
}

impl<P: PrimeGroupConfig> Mul<&P::ScalarField> for PublicEC<P> {
    type Output = GroupEC<P>;

    fn mul(self, other: &P::ScalarField) -> GroupEC<P> {
        GroupEC {
            point: P::scalar_mul_pub(&self.point, other),
        }
    }
}

impl<P: PrimeGroupConfig> Mul<&P::ScalarField> for &PublicEC<P> {
    type Output = GroupEC<P>;

    fn mul(self, other: &P::ScalarField) -> GroupEC<P> {
        GroupEC {
            point: P::scalar_mul_pub(&self.point, other),
        }
    }
}

impl<P: PrimeGroupConfig> Add<&PublicEC<P>> for PublicEC<P> {
    type Output = GroupEC<P>;

    fn add(self, other: &PublicEC<P>) -> GroupEC<P> {
        let mut res = self.into();
        res += other;
        res
    }
}

impl<P: PrimeGroupConfig> Add<PublicEC<P>> for PublicEC<P> {
    type Output = GroupEC<P>;

    fn add(self, other: PublicEC<P>) -> GroupEC<P> {
        let mut res = self.into();
        res += other;
        res
    }
}

impl<P: PrimeGroupConfig> AddAssign<&PublicEC<P>> for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    fn add_assign(&mut self, other: &PublicEC<P>) {
        P::add_public_in_place(&mut self.point, &other.point);
    }
}

impl<P: PrimeGroupConfig> Add<&PublicEC<P>> for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    type Output = GroupEC<P>;

    fn add(mut self, other: &PublicEC<P>) -> GroupEC<P> {
        self += other;
        self
    }
}

impl<P: PrimeGroupConfig> AddAssign<PublicEC<P>> for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    fn add_assign(&mut self, other: PublicEC<P>) {
        *self += &other;
    }
}

impl<P: PrimeGroupConfig> Add<PublicEC<P>> for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    type Output = GroupEC<P>;

    fn add(mut self, other: PublicEC<P>) -> GroupEC<P> {
        self += other;
        self
    }
}

impl<P: PrimeGroupConfig> SubAssign<&PublicEC<P>> for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    fn sub_assign(&mut self, other: &PublicEC<P>) {
        P::neg_in_place(&mut self.point);
        *self += other;
        P::neg_in_place(&mut self.point);
    }
}

impl<P: PrimeGroupConfig> Sub<&PublicEC<P>> for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    type Output = GroupEC<P>;

    fn sub(mut self, other: &PublicEC<P>) -> GroupEC<P> {
        self -= other;
        self
    }
}

impl<P: PrimeGroupConfig> SubAssign<PublicEC<P>> for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    fn sub_assign(&mut self, other: PublicEC<P>) {
        *self -= &other;
    }
}

impl<P: PrimeGroupConfig> Sub<PublicEC<P>> for GroupEC<P>
where
    P: PrimeGroupConfig,
{
    type Output = GroupEC<P>;

    fn sub(mut self, other: PublicEC<P>) -> GroupEC<P> {
        self -= other;
        self
    }
}
