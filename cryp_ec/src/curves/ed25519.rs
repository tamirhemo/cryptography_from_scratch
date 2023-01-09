
use crate::{edwards::*, models::Coordinates};
use cryp_alg::ff::*;
use cryp_std::vec::Vec;
use cryp_std::rand::Rng;

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

    const C: [u64; 4] = [38, 0, 0, 0];
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

    // 2^252+27742317777372353535851937790883648493
    const MODULUS: [Self::Limb; 4] = [
        6346243789798364141,
        1503914060200516822,
        0,
        1152921504606846976,
    ];

    // 7237005577332262213973186563042994240413239274941949949428319933631315875101
    const R: [Self::Limb; 4] = [
        15486807595281847581,
        14334777244411350896,
        18446744073709551614,
        1152921504606846975,
    ];

    // 1627715501170711445284395025044413883736156588369414752970002579683115011841
    const R2: [Self::Limb; 4] = [
        11819153939886771969,
        14991950615390032711,
        14910419812499177061,
        259310039853996605,
    ];

    const MP: Self::Limb = 15183074304973897243;
}

impl PrimeSubGroupConfig for EdwardsAM1UnifiedOperations<Ed25519Parameters> {
    type ScalarField = ScalarEd25519;

    const COFACTOR: u32 = 8;

    fn generator<R: Rng>(rng: Option<&mut R>) -> Self::Affine {

        let x = Fp25519::from_int(&Ed25519Parameters::X.into());
        let y = Fp25519::from_int(&Ed25519Parameters::Y.into());
        let affine_point = Self::Affine::new(x, y);
        
        let mut point = Self::Point::from(affine_point);
        if let Some(rng) = rng {
            let scalar = ScalarEd25519::rand(rng);
            point = <Self as PrimeSubGroupConfig>::scalar_mul(&point, &scalar);
        }
        point.into_affine().unwrap()
    }

    fn batch_generators<R: Rng>(
        n: usize,
        rng: &mut R,
    ) -> cryp_std::vec::Vec<Self::Affine> {
        let mut generators = Vec::with_capacity(n+1);

        for _ in 0..n {
            generators.push(<Self as PrimeSubGroupConfig>::generator(Some(rng)));
        }

        generators
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cryp_std::rand::thread_rng;
    use cryp_std::rand::UniformRand;

    #[test]
    fn test_field() {
        let x = Fp25519::from_int(&[3293829, 232323, 4473653, 2323].into());

        let modulus_minus_one: [u64; 4] = [
            18446744073709551596,
            18446744073709551615,
            18446744073709551615,
            9223372036854775807,
        ];
        assert_eq!(Fp25519::from_int(&modulus_minus_one.into()) + Fp25519::one(),
        Fp25519::zero());

        assert_eq!(x + x, x.double());
        assert_eq!(x - x, Fp25519::zero());
        assert_eq!(x.exp(&[1200u64])*x.exp(&[250u64]), x.exp(&[1450u64]));
        assert_eq!(x.exp(&modulus_minus_one), Fp25519::one());

        let mut rng = thread_rng();
        let y = Fp25519::from_int(&[
            u64::rand(& mut rng), 
            u64::rand(& mut rng), 
            u64::rand(& mut rng), 
            u64::rand(& mut rng)].into());

        assert_eq!(y + y, y.double());
        assert_eq!(y - y, Fp25519::zero());
        assert_eq!(y.exp(&[1200u64])*y.exp(&[250u64]), y.exp(&[1450u64]));
        assert_eq!(x.exp(&modulus_minus_one), Fp25519::one());
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
    fn test_scalar_field() {


        let one = ScalarEd25519::one();
        let zero = ScalarEd25519::zero();
        let x = ScalarEd25519::from_int(&[3293829, 232323, 4473653, 2323].into());

        let mut rng = thread_rng();
        let y = ScalarEd25519::from_int(&[
            u64::rand(& mut rng), 
            u64::rand(& mut rng), 
            u64::rand(& mut rng), 
            u64::rand(& mut rng)].into());

        let modulus_minus_one : [u64; 4] = [
            6346243789798364140,
            1503914060200516822,
            0,
            1152921504606846976,
        ];
        
        let power = ScalarEd25519::from_int(&modulus_minus_one.into());
        assert_eq!(power + one, zero);

        assert_eq!(x*x.inverse().unwrap(), one);
        assert_eq!(x.inverse().unwrap(), x.exp(&(power-one).as_int()));
    }

    #[test]
    fn test_group() {

        let x = Fp25519::from_int(&Ed25519Parameters::X.into());
        let y = Fp25519::from_int(&Ed25519Parameters::Y.into());
        let affine_point = AffineEd25519::new(Affine::new(
            x,
            y,
        ));

        let one = Fp25519::one();
        let d = Ed25519Parameters::D;

        let point = GroupEd25519::from(affine_point);

        // check on curve
        assert_eq!(-x.square() + y.square(), one + d * x.square() * y.square());

        // check random elements 
        let mut rng = thread_rng();
        let generators = GroupEC::batch_generators(10, &mut rng);

        assert_ne!(generators[8], generators[4]);
        

        let identity = GroupEd25519::identity();
        assert_ne!(GroupEC::from(generators[8]), identity);

        assert_eq!(point + point, point.double());
        assert_eq!(point.double().double(), point.double() + point.double());
        assert_eq!(identity + point, point);
        assert_eq!(identity + point.double(), point.double());
        assert_eq!(point.mul_int(&[2u32]), point.double());
        assert_eq!(x.square(), x.exp(&[2u32]));
        assert_eq!(x*&(one + one), x.double());

        let order : [u64; 4] = [
            6346243789798364141,
            1503914060200516822,
            0,
            1152921504606846976,
        ]; 

        let order_minus_one : [u64; 4] = [
            6346243789798364140,
            1503914060200516822,
            0,
            1152921504606846976,
        ]; 

        assert_eq!(point.mul_int(&order), identity);
        assert_eq!(point.mul_int(&order_minus_one), -point);
        assert_eq!(point-point, identity);

        let mod_minus_one = ScalarEd25519::from_int(&order_minus_one.into());
        assert_ne!(point*&mod_minus_one, point);
        assert_eq!(point*&mod_minus_one, -point);

    }
}
