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

/// A Pederesen commitment scheme for vectors of arbitrary size
/// The maximal dimension is determined at setup time
pub struct PedersenVec<G: PrimeGroup> {
    _marker: cryp_std::marker::PhantomData<G>,
}

#[derive(Clone)]
pub struct PedersenPP<G: PrimeGroup, const N: usize> {
    g_vec: [G::Public; N],
    h: G::Public,
}

#[derive(Clone)]
pub struct PedersenVecPP<G: PrimeGroup> {
    g_vec: Vec<G::Public>,
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
        let group_elements = G::batch_generators(N + 1, rng);
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

        // Check public input has correct length
        assert_eq!(pp.g_vec.len(), N);
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
        // necessary checks
        assert!(G::is_valid(&commitment));
        assert_eq!(pp.g_vec.len(), N);

        // cverify commitment
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

// ----------------------------
// Implement vector commitment with heap allocated vectors


impl<G: PrimeGroup> VectorCommitment<Vec<G::ScalarField>> for PedersenVec<G> {
    type PublicParameters = PedersenVecPP<G>;
    type Commitment = G::Public;
    type Randomness = G::Public;
    type Error = ();

    fn setup<R: cryp_std::rand::Rng>(
        rng: &mut R,
        max_dim: usize,
    ) -> Result<Self::PublicParameters, Self::Error> {
        let group_elements = G::batch_generators(max_dim + 1, rng);
        assert!(group_elements.len() == max_dim+ 1);

        // Should succeed because of assert
        let g_vec = group_elements[0..max_dim].to_vec();
        let h = group_elements[max_dim];

        Ok(PedersenVecPP { g_vec, h })
    }

    fn commit(
        pp: &Self::PublicParameters,
        input: &Vec<G::ScalarField>,
        rng: Option<&mut impl Rng>,
    ) -> Result<(Self::Commitment, Self::Randomness), Self::Error> {
        // If rng is provided, compute a random field element r,
        // and compute the commitment as g^input * h^r, and output h^r as the randomness
        // If not, compute the commitment as g^input, and output h as the randomness

        assert!(input.len() <= pp.g_vec.len(), "Input vector is too long");

        // random field element, compute h^r
        // TODO: Error handling

        let h_rand = rng
            .map(G::ScalarField::rand)
            .map(|r| pp.h * &r)
            .map(|hr| {
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
        input: &Vec<G::ScalarField>,
        randomness: &Self::Randomness,
    ) -> Result<bool, Self::Error> {
        // Necessary checks
        assert!(G::is_valid(&commitment));
        assert!(input.len() <= pp.g_vec.len(), "Input vector is too long");

        // Verify commitment
        let commit_g = G::msm(&pp.g_vec, input);

        let commitment_check = (commit_g + randomness)
            .as_public()
            .expect("The group element should be able to convert to public");

        Ok(commitment == &commitment_check)
    }
}



impl<G: PrimeGroup> VCPublicParameters for PedersenVecPP<G> {
    fn max_dim(&self) -> usize {
        self.g_vec.len()
    }
}


// =============================

// Tests

// ==============================

#[cfg(test)]
mod tests {
    use super::*;
    use cryp_ec::curves::edwards25519::*;
    use cryp_std::rand::{rngs::ThreadRng, thread_rng};

    #[test]
    fn test_pedersen() {
        let mut rng = thread_rng();

        pub type PedEd = Pedersen<GroupEd25519, 1>;

        // our secret
        let secret = ScalarEd25519::rand(&mut rng);

        let pp = PedEd::setup(&mut rng, 1).unwrap();
        let (commitment, randomness) = PedEd::commit(&pp, &[secret], Some(&mut rng)).unwrap();

        assert_ne!(commitment, GroupEd25519::generator::<ThreadRng>(None));

        assert!(PedEd::verify(&pp, &commitment, &[secret], &randomness).unwrap());

        // A commitment to a fixed length vector
        const D: usize = 10;
        pub type PedEdVec = Pedersen<GroupEd25519, D>;
        let mut input = [ScalarEd25519::zero(); D];
        for i in 0..D {
            input[i] = ScalarEd25519::rand(&mut rng);
        }
        let pp = PedEdVec::setup(&mut rng, D).unwrap();
        let (commitment, randomness) = PedEdVec::commit(&pp, &input, Some(&mut rng)).unwrap();

        assert_ne!(commitment, GroupEd25519::generator::<ThreadRng>(None));

        assert!(PedEdVec::verify(&pp, &commitment, &input, &randomness).unwrap());

        // A commitment to a vectors of different length

        let d = 50;
        pub type PedVec = PedersenVec<GroupEd25519>;
        
        let pp = PedVec::setup(&mut rng, d).unwrap();

        let m = 10;
        let input = (0..m).map(|_| ScalarEd25519::rand(&mut rng)).collect::<Vec<_>>();
        let (commitment, randomness) = PedVec::commit(&pp, &input, Some(&mut rng)).unwrap();

        assert_ne!(commitment, GroupEd25519::generator::<ThreadRng>(None));

        assert!(PedVec::verify(&pp, &commitment, &input, &randomness).unwrap());
    }
}
