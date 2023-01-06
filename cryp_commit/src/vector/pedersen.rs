use cryp_alg::PrimeGroup;
use cryp_std::rand::UniformRand;
use super::*;


pub struct Pedersen<G : PrimeGroup, const N : usize>{
    _marker: cryp_std::marker::PhantomData<G>,
}

#[derive(Clone)]
pub struct PedersenPP<G : PrimeGroup, const N : usize>{
    g_vec : [G::Public; N],
    h : G::Public,
}

impl<G : PrimeGroup, const N : usize> VectorCommitment<[G::ScalarField; N]> for Pedersen<G, N> {
    type PublicParameters = PedersenPP<G, N>;
    type Commitment = G::Public;
    type Randomness = G::Public;
    type Error = ();

    fn setup<R: cryp_std::rand::Rng>(rng: &mut R, max_dim: usize) 
        -> Result<Self::PublicParameters, Self::Error> {
        assert!(max_dim == N);
        let group_elements = G::batch_generators(N+1, Some(rng));
        assert!(group_elements.len() == N+1);

        // Should succeed because of assert
        let g_vec : [G::Public; N] = group_elements[0..N].try_into().unwrap();
        let h = group_elements[N];

        Ok(PedersenPP{g_vec, h})
    }

    fn commit(
            pp: &Self::PublicParameters,
            input: &[G::ScalarField; N],
            rng: Option<&mut impl Rng>,
        ) -> Result<(Self::Commitment, Self::Randomness), Self::Error> {
        let randomness = match rng {
            Some(rng) => {
                let r = G::ScalarField::rand(rng);
                let hr = pp.h*&r;
                Some(hr)
            },
            None => None,
        };
        let commit = G::msm(&pp.g_vec, input);

        randomness.map(|hr| hr + commit);
        Err(())
    }

}




impl<G : PrimeGroup, const N : usize> VCPublicParameters for PedersenPP<G, N> {
    fn max_dim(&self) -> usize {
        N
    }
}

