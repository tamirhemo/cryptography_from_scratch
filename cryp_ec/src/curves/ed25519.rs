use cryp_alg::ff::*;
use crate::edwards::*;


pub type Fp25519 = F<MontgomeryOperations<4, Fp25519Params>>;

pub type ScalarEd25519 = F<MontgomeryOperations<4, ScalarEd25519Parameters>>;

pub type GroupEd25519 = GroupEC<EdwardsAM1UnifiedOperations<Ed25519Parameters>>;


/// Parameters for the prime field Fp25519
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fp25519Params;

impl MontParameters<4usize> for Fp25519Params {
    type Limb = u64;

    const MODULUS: [Self::Limb; 4] = [
        18446744073709551597,
        18446744073709551615,
        18446744073709551615,
        9223372036854775807,
    ];

    const R: [Self::Limb; 4] = [19, 0, 0, 0];

    const R2: [Self::Limb; 4] = [361, 0, 0, 0];
    const MP: Self::Limb = 9708812670373448219;
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519Parameters;

impl TwistedEdwardsAM1 for Ed25519Parameters {
    type Field = Fp25519;

    // The element d in the Montgomery representation
    const D: Self::Field =
        Fp25519::from_RAW_limbs(<Fp25519 as PrimeField>::BigInteger::from_limbs([
            18446744073709551613u64,
            18446744073709551615u64,
            18446744073709551615u64,
            1456321897198780415u64,
        ]));

    // The element d2 in the Montgomery representation
    const D2: Self::Field =
        Fp25519::from_RAW_limbs(<Fp25519 as PrimeField>::BigInteger::from_limbs([
            18446744073709551610,
            18446744073709551615,
            18446744073709551615,
            2912643794397560831,
        ]));
}



// The scalar Field
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScalarEd25519Parameters;

impl MontParameters<4usize> for ScalarEd25519Parameters {
    type Limb = u64;

    const MODULUS: [Self::Limb; 4] = [
        6346243789798364141,
        1503914060200516822,
        0,
        1152921504606846976,
    ];

    const R: [Self::Limb; 4] = [
        15486807595281847581, 
        14334777244411350896,
        18446744073709551614,
        1152921504606846975];

    const R2: [Self::Limb; 4] = [
        8297226434800960841,
        7720218508174777668,
        5971159353574489177,
        2223003849640556432];

    const MP: Self::Limb = 15183074304973897243;
}


impl PrimeSubGroupConfig for EdwardsAM1UnifiedOperations<Ed25519Parameters> {
    type ScalarField = ScalarEd25519;

    const COFACTOR: u32 = 8;

    fn generator(rng: Option<impl cryp_std::rand::Rng>) -> Self::Affine {
        Self::Affine::new(Fp25519::one(), Fp25519::one())
    }

    fn rand(rng: impl cryp_std::rand::Rng) -> Self::Affine {
        Self::Affine::new(Fp25519::one(), Fp25519::one())
    }

    fn batch_generators(n: usize, rng: Option<impl cryp_std::rand::Rng>) -> cryp_std::vec::Vec<Self::Affine> {
        cryp_std::vec::Vec::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use cryp_std::rand::rngs::mock::StepRng;

    #[test]
    fn test_curve_parameters() {
        assert!(Ed25519Parameters::verify());

        let d = -Fp25519::from_int(&[121665, 0, 0, 0].into())
            / Fp25519::from_int(&[121666, 0, 0, 0].into());

        let d2 = d + d;

        assert_eq!(d, Ed25519Parameters::D);
        assert_eq!(d2, Ed25519Parameters::D2);
        assert_eq!(d2, d.double());
    }

    #[test]
    fn test_field() {
        let element = Fp25519::from_int(&[0u64, 1u64, 0u64, 1u64].into());

        let power = element.exp(&<Fp25519 as PrimeField>::BigInteger::from([
            0u64, 1u64, 2555u64, 1u64,
        ]));

        assert!(power != element);
    }

    #[test]
    fn test_scalar_field() {

        let identity = GroupEd25519::identity();
        let generator = GroupEd25519::from(GroupEd25519::generator::<StepRng>(None));

        assert_eq!(generator.double(), generator + generator);
        assert_eq!(generator + identity, generator);
    }
}
