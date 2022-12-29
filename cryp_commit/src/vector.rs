use cryp_std::hash::Hash;
use cryp_std::rand::Rng;

use cryp_alg::Field;

mod pedersen;

pub enum VectorCommitmentError {
    DimensionTooLarge,
}

pub trait VCPublicParameters: Clone {
    fn max_dim(&self) -> usize;
}
pub trait VCKey {
    fn max_dim(&self) -> usize;
    fn supported_dim(&self) -> usize;
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
    type CommitmentKey: VCKey;
    type VerifierKey: VCKey;
    type Commitment: Clone + Eq + Hash;
    type Randomness;
    type Error: From<VectorCommitmentError>;

    fn setup<R: Rng>(rng: &mut R, max_dim: usize) -> Self::PublicParameters;

    fn trim(
        pp: &Self::PublicParameters,
        supported_dim: usize,
        hiding_bound: usize,
    ) -> (Self::CommitmentKey, Self::VerifierKey);

    fn commit(
        ck: &Self::CommitmentKey,
        input: &V,
        rng: Option<&mut impl Rng>,
    ) -> Result<(Self::Commitment, Self::Randomness), Self::Error>;
}

/// The interface of a commitment scheme for vectors that supports inner product proofs
pub trait InnerProductCommitment<V: InnerProductVector>: VectorCommitment<V> {
    type Proof;
    type IPError: From<Self::Error>;

    fn open(
        ck: &Self::CommitmentKey,
        commitment: &Self::Commitment,
        randomness: &Self::Randomness,
        input: &V,
        public_vector: &V,
        rng: Option<&mut impl Rng>,
    ) -> Result<Self::Proof, Self::IPError>;

    fn verify(
        vk: &Self::VerifierKey,
        commitment: &Self::Commitment,
        public_vector: &V,
        claimed_inner_product: &V::Field,
        proof: &Self::Proof,
    ) -> Result<bool, Self::IPError>;
}
