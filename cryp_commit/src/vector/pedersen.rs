use super::*;
use cryp_alg::PrimeGroup;
use cryp_std::rand::UniformRand;

/// A pederesen commitment scheme for vectors of size N
///
/// The commitment is implements the following
///
/// * Setup(rnd) : returns a public parameter
pub struct Pedersen<G: PrimeGroup, const N: usize> {
    _marker: cryp_std::marker::PhantomData<G>,
}

#[derive(Clone)]
pub struct PedersenPP<G: PrimeGroup, const N: usize> {
    g_vec: [G::Public; N],
    h: G::Public,
}

impl<G: PrimeGroup, const N: usize> VectorCommitment<[G::ScalarField; N]> for Pedersen<G, N> {
    type PublicParameters = PedersenPP<G, N>;
    type Commitment = G::Public;
    type Randomness = G::Public;
    type Error = ();

    fn setup<R: cryp_std::rand::Rng>(
        rng: &mut R,
        max_dim: usize,
    ) -> Result<Self::PublicParameters, Self::Error> {
        assert!(max_dim == N);
        let group_elements = G::batch_generators(N + 1, Some(rng));
        assert!(group_elements.len() == N + 1);

        // Should succeed because of assert
        let g_vec: [G::Public; N] = group_elements[0..N].try_into().unwrap();
        let h = group_elements[N];

        Ok(PedersenPP { g_vec, h })
    }

    fn commit(
        pp: &Self::PublicParameters,
        input: &[G::ScalarField; N],
        rng: Option<&mut impl Rng>,
    ) -> Result<(Self::Commitment, Self::Randomness), Self::Error> {
        // If rng is provided, compute a random field element r,
        // and compute the commitment as g^input * h^r, and output h^r as the randomness
        // If not, compute the commitment as g^input, and output h as the randomness

        // random field element
        // TODO: Error handling
        let h_rand = rng.map(G::ScalarField::rand).map(|r| pp.h * &r).map(|hr| {
            hr.as_public()
                .expect("The group element should be able to convert to public")
        });

        let commit_g = G::msm(&pp.g_vec, input);

        let (commit_priv, randomness) = match h_rand {
            Some(hr) => (commit_g + hr, hr),
            None => (commit_g, pp.h),
        };

        // Convert to public (avoids projective coordinate attacks)
        let commitment = commit_priv
            .as_public()
            .expect("The group element should be able to convert to public");

        Ok((commitment, randomness))
    }

    fn verify(
        pp: &Self::PublicParameters,
        commitment: &Self::Commitment,
        input: &[G::ScalarField; N],
        randomness: &Self::Randomness,
    ) -> Result<bool, Self::Error> {
        assert!(G::is_valid(&commitment));
        let commit_g = G::msm(&pp.g_vec, input);

        let commitment_check = (commit_g + randomness)
            .as_public()
            .expect("The group element should be able to convert to public");

        Ok(commitment == &commitment_check)
    }
}

impl<G: PrimeGroup, const N: usize> VCPublicParameters for PedersenPP<G, N> {
    fn max_dim(&self) -> usize {
        N
    }
}
