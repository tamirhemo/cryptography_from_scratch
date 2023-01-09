use crate::edwards::*;
use cryp_alg::ff::*;

pub type Fp25519 = F<GeneralReductionOperations<4, SolinasReduction<4, Fp25519Params>>>;
pub type ScalarEd25519 = F<MontgomeryOperations<4, ScalarEd25519Parameters>>;
pub type GroupEd25519 = GroupEC<EdwardsAM1UnifiedOperations<Ed25519Parameters>>;
pub type AffineEd25519 = PublicEC<EdwardsAM1UnifiedOperations<Ed25519Parameters>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519Parameters;

/// Parameters for the prime field Fp25519
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fp25519Params;

impl Ed25519Parameters {
    // 15112221349535400772501151409588531511454012693041857206046113283949847762202
    const X: [u64; 4] = [
        14507833142362363162,
        7578651490590762930,
        13881468655802702940,
        2407515759118799870,
    ];

    // 46316835694926478169428394003475163141307993866256225615783033603165251855960
    const Y: [u64; 4] = [
        7378697629483820632,
        7378697629483820646,
        7378697629483820646,
        7378697629483820646,
    ];
}

impl MontParameters<4usize> for Fp25519Params {
    type Limb = u64;

    const MODULUS: [Self::Limb; 4] = [
        18446744073709551597,
        18446744073709551615,
        18446744073709551615,
        9223372036854775807,
    ];

    const R: [Self::Limb; 4] = [38, 0, 0, 0];

    const R2: [Self::Limb; 4] = [1444, 0, 0, 0];
    const MP: Self::Limb = 9708812670373448219;
}

impl SolinasParameters<4usize> for Fp25519Params {
    type Limb = u64;

    // 2^255-19 =  57896044618658097711785492504343953926634992332820282019728792003956564819949
    const MODULUS: [Self::Limb; 4] = [
        18446744073709551597,
        18446744073709551615,
        18446744073709551615,
        9223372036854775807,
    ];

    const C: u64 = 38;
}

impl TwistedEdwardsAM1 for Ed25519Parameters {
    type Field = Fp25519;

    // The element d in the regular representation
    // d  =  -121665/121666
    // 37095705934669439343138083508754565189542113879843219016388785533085940283555
    const D: Self::Field =
        Fp25519::from_RAW_limbs(<Fp25519 as PrimeField>::BigInteger::from_limbs([
            8496970652267935907,
            31536524315187371,
            10144147576115030168,
            5909686906226998899,
        ]));

    // The element d2 = d + d in the regular representation
    const D2: Self::Field =
        Fp25519::from_RAW_limbs(<Fp25519 as PrimeField>::BigInteger::from_limbs([
            16993941304535871833,
            63073048630374742,
            1841551078520508720,
            2596001775599221991,
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
        1152921504606846975,
    ];

    const R2: [Self::Limb; 4] = [
        8297226434800960841,
        7720218508174777668,
        5971159353574489177,
        2223003849640556432,
    ];

    const MP: Self::Limb = 15183074304973897243;
}

impl PrimeSubGroupConfig for EdwardsAM1UnifiedOperations<Ed25519Parameters> {
    type ScalarField = ScalarEd25519;

    const COFACTOR: u32 = 8;

    fn generator(rng: Option<impl cryp_std::rand::Rng>) -> Self::Affine {
        Self::Affine::new(
            Fp25519::from_int(&Ed25519Parameters::X.into()),
            Fp25519::from_int(&Ed25519Parameters::Y.into()),
        )
    }

    fn rand(rng: impl cryp_std::rand::Rng) -> Self::Affine {
        Self::Affine::new(Fp25519::one(), Fp25519::one())
    }

    fn batch_generators(
        n: usize,
        rng: Option<impl cryp_std::rand::Rng>,
    ) -> cryp_std::vec::Vec<Self::Affine> {
        cryp_std::vec::Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cryp_std::rand::rngs::mock::StepRng;

    #[test]
    fn test_field() {
        let x = Fp25519::from_int(&[3293829, 232323, 4473653, 2323].into());

        let power: [u64; 4] = [
            18446744073709551596,
            18446744073709551615,
            18446744073709551615,
            9223372036854775807,
        ];

        assert_eq!(x + x, x.double());
        assert_eq!(x - x, Fp25519::zero());
        assert_eq!(x.exp(&[1200u64])*x.exp(&[250u64]), x.exp(&[1450u64]));
        assert_eq!(x.exp(&power), Fp25519::one());
    }

    #[test]
    fn test_parameters() {
        // d  =  -121665/121666
        let numerator = Fp25519::from_int(&[121665, 0, 0, 0].into());
        let denominator = Fp25519::from_int(&[121666, 0, 0, 0].into());

        let d = -numerator * denominator.inverse().unwrap();

        assert_eq!(d, Ed25519Parameters::D);
        assert_eq!(d + d, Ed25519Parameters::D2);
    }

    #[test]
    fn test_group() {
        let affine_point = AffineEd25519::new(Affine::new(
            Fp25519::from_int(&Ed25519Parameters::X.into()),
            Fp25519::from_int(&Ed25519Parameters::Y.into()),
        ));

        let point = GroupEd25519::from(affine_point);

        let identity = GroupEd25519::identity();

        assert_eq!(point + point, point.double());
        assert_eq!(point + identity, point);
        assert_eq!(point.mul_int(&[1u32]), point);

        let order : [u64; 4] = [
            6346243789798364141,
            1503914060200516822,
            0,
            1152921504606846976,
        ]; 

        //assert_eq!(identity.as_public(), point.as_public());
        //assert_eq!(point.mul_int(&[1u32]), point);
    }
}
