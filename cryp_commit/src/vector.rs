use cryp_std::hash::Hash;
use cryp_std::rand::Rng;
use cryp_alg::Field;

mod pedersen;

pub trait VCPublicParameters: Clone {
    fn max_dim(&self) -> usize;
}

/// Minimal interface of a vector over a given field
pub trait Vector {
    type Field: Field;

    fn dim(&self) -> usize;
}

/// Minimal interface for a vector over a field with a fixed basis
pub trait BasedVector: Vector {
    fn get(&self, i: usize) -> &Self::Field;
}

/// Minimal interface for a vector in an inner product space
pub trait InnerProductVector: Vector {
    fn inner_product(&self, other: &Self) -> Self::Field;
}

/// Minimal interface for a vector commitment scheme
pub trait VectorCommitment<V: Vector> {
    type PublicParameters: VCPublicParameters;
    type Commitment: Clone + Eq + Hash;
    type Randomness;
    type Error;

    fn setup<R: Rng>(rng: &mut R, max_dim: usize) -> Result<Self::PublicParameters, Self::Error> ;

    /// Create a commitment to a vector of scalars
    /// 
    /// The operation should run in constant time in the following sense:
    /// 
    /// - The number of group operations should be independent of the input itself (can depend on the length of the input)
    /// - If rng is not None, the commitment is hiding, and takes a different amount of time from
    /// a non-hiding commitment. 
    fn commit(
        pp: &Self::PublicParameters,
        input: &V,
        rng: Option<&mut impl Rng>,
    ) -> Result<(Self::Commitment, Self::Randomness), Self::Error>;
}

/// The interface of a commitment scheme for vectors that supports inner product proofs
pub trait InnerProductCommitment<V: InnerProductVector>: VectorCommitment<V> {
    type Proof;
    type IPError: From<Self::Error>;

    fn open(
        pp: &Self::PublicParameters,
        commitment: &Self::Commitment,
        randomness: &Self::Randomness,
        input: &V,
        public_vector: &V,
        rng: Option<&mut impl Rng>,
    ) -> Result<Self::Proof, Self::IPError>;

    fn verify(
        pp: &Self::PublicParameters,
        commitment: &Self::Commitment,
        public_vector: &V,
        claimed_inner_product: &V::Field,
        proof: &Self::Proof,
    ) -> Result<bool, Self::IPError>;
}


impl<F: Field, const N: usize> Vector for [F; N] {
    type Field = F;

    fn dim(&self) -> usize {
        N
    }
}