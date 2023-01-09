use cryp_std::fmt::Debug;
use cryp_std::hash::Hash;
use cryp_std::rand::UniformRand;

/// Limb is a trait which represents a single limb of a big integer.
///
/// Currently, it is assumed implicitly that Limb behaves like a power of two when
/// it comes to coversion from bits.
pub trait Limb:
    Sized + Copy + Clone + PartialEq + Eq + Debug + Send + Sync + Hash + UniformRand + PartialOrd + Ord
{
    /// The type used to represent a carry bit.
    type Carry: PartialEq + Eq + Copy + Clone + Debug;

    const BYTES: usize;
    type Bytes: 'static + Sized + Copy + Clone + IntoIterator<Item = u8>;

    const NO: Self::Carry;

    const ZERO: Self;
    const ONE: Self;

    ///  Calculates `self + rhs + carry` without the ability to overflow.
    ///
    /// Performs "ternary addition" which takes in an extra bit to add, and may return an
    /// additional bit of overflow. This allows for chaining together multiple additions
    /// to create "big integers" which represent larger values.
    fn add_carry(&self, rhs: Self, carry: Self::Carry) -> (Self, Self::Carry);

    ///  Calculates `self - rhs - carry` without the ability to overflow.
    fn sub_carry(&self, rhs: Self, carry: Self::Carry) -> (Self, Self::Carry);

    fn mul_carry(&self, rhs: Self, carry: Self) -> (Self, Self);

    fn into_bytes_be(&self) -> Self::Bytes;
    fn into_bytes_le(&self) -> Self::Bytes;

    fn from_bytes_be(bytes: &[u8]) -> Result<Self, WrongByteLengthError>;
    fn from_bytes_le(bytes: &[u8]) -> Result<Self, WrongByteLengthError>;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct WrongByteLengthError;

impl Limb for u32 {
    type Carry = bool;
    const ZERO: Self = 0;
    const ONE: Self = 1;

    const BYTES: usize = 4;
    type Bytes = [u8; 4];

    const NO: bool = false;

    fn add_carry(&self, rhs: Self, carry: Self::Carry) -> (Self, Self::Carry) {
        let (a, b) = self.overflowing_add(rhs);
        let (c, d) = a.overflowing_add(carry as u32);
        (c, b || d)
    }
    fn sub_carry(&self, rhs: Self, carry: Self::Carry) -> (Self, Self::Carry) {
        let (a, b) = self.overflowing_sub(rhs);
        let (c, d) = a.overflowing_sub(carry as u32);
        (c, b || d)
    }

    fn mul_carry(&self, rhs: Self, carry: Self) -> (Self, Self) {
        let mul = (*self as u64) * (rhs as u64) + (carry as u64);
        (mul as u32, (mul >> 32) as u32)
    }

    fn into_bytes_be(&self) -> Self::Bytes {
        self.to_be_bytes()
    }

    fn into_bytes_le(&self) -> Self::Bytes {
        self.to_le_bytes()
    }

    fn from_bytes_be(bytes: &[u8]) -> Result<Self, WrongByteLengthError> {
        bytes
            .try_into()
            .map_err(|_| WrongByteLengthError)
            .map(u32::from_be_bytes)
    }

    fn from_bytes_le(bytes: &[u8]) -> Result<Self, WrongByteLengthError> {
        bytes
            .try_into()
            .map_err(|_| WrongByteLengthError)
            .map(u32::from_le_bytes)
    }
}

impl Limb for u64 {
    type Carry = bool;
    const ZERO: Self = 0;
    const ONE: Self = 1;

    const BYTES: usize = 8;
    type Bytes = [u8; 8];

    const NO: bool = false;

    fn add_carry(&self, rhs: Self, carry: Self::Carry) -> (Self, Self::Carry) {
        let (a, b) = self.overflowing_add(rhs);
        let (c, d) = a.overflowing_add(carry as u64);
        (c, b || d)
    }

    fn sub_carry(&self, rhs: Self, carry: Self::Carry) -> (Self, Self::Carry) {
        let (a, b) = self.overflowing_sub(rhs);
        let (c, d) = a.overflowing_sub(carry as u64);
        (c, b || d)
    }

    fn mul_carry(&self, rhs: Self, carry: Self) -> (Self, Self) {
        let mul = (*self as u128) * (rhs as u128) + (carry as u128);
        (mul as u64, (mul >> 64) as u64)
    }

    fn into_bytes_be(&self) -> Self::Bytes {
        self.to_be_bytes()
    }

    fn into_bytes_le(&self) -> Self::Bytes {
        self.to_le_bytes()
    }

    fn from_bytes_be(bytes: &[u8]) -> Result<Self, WrongByteLengthError> {
        bytes
            .try_into()
            .map_err(|_| WrongByteLengthError)
            .map(u64::from_be_bytes)
    }

    fn from_bytes_le(bytes: &[u8]) -> Result<Self, WrongByteLengthError> {
        bytes
            .try_into()
            .map_err(|_| WrongByteLengthError)
            .map(u64::from_le_bytes)
    }
}

// -----------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_carry() {
        assert_eq!(0u32.add_carry(0, false), (0, false));
        assert_eq!(0u32.add_carry(0, true), (1, false));
        assert_eq!(0u32.add_carry(1, false), (1, false));
        assert_eq!(0u32.add_carry(1, true), (2, false));
        assert_eq!(0u32.add_carry(0, false), (0, false));
        assert_eq!(u64::MAX.add_carry(1, false), (0, true));
    }

    #[test]
    fn test_sub_carry() {
        assert_eq!(0u32.sub_carry(0, false), (0, false));
        assert_eq!(0u32.sub_carry(0, true), (u32::MAX, true));
        assert_eq!(0u32.sub_carry(1, false), (u32::MAX, true));
        assert_eq!(0u32.sub_carry(1, true), (u32::MAX - 1, true));
        assert_eq!(u64::MAX.sub_carry(1, false), (u64::MAX - 1, false));
        assert_eq!(u64::MAX.sub_carry(1, true), (u64::MAX - 2, false));
    }

    #[test]
    fn test_mul_carry() {
        use rand::thread_rng;
        let mut rng = thread_rng();
        use cryp_std::rand::UniformRand;
        for _ in 0..100 {
            let lhs = u64::rand(&mut rng);
            let rhs = u64::rand(&mut rng);
            let carry = u64::rand(&mut rng);
            let (a, b) = lhs.mul_carry(rhs, carry);
            let mul = (a as u128) + ((b as u128) << 64);
            assert_eq!(mul, (lhs as u128) * (rhs as u128) + (carry as u128));
        }
        assert_eq!(10u32.mul_carry(10u32, 0u32), (100, 0));
        assert_eq!(100u32.mul_carry(10u32, 1u32), (1001, 0));
        assert_eq!(u32::MAX.mul_carry(2u32, 1u32), (4294967295, 1));

        let (lhs, rhs, carry) = (u32::MAX / 4, 200, 10);
        let (a, b) = lhs.mul_carry(rhs, carry);
        let mul = (a as u64) + ((b as u64) << 32);
        assert_eq!(mul, (lhs as u64) * (rhs as u64) + (carry as u64));

        let (lhs, rhs, carry) = (u32::MAX / 4, 200, 10);
        let (a, b) = lhs.mul_carry(rhs, carry);
        let mul = (a as u64) + ((b as u64) << 32);
        assert_eq!(mul, (lhs as u64) * (rhs as u64) + (carry as u64));
    }
}
