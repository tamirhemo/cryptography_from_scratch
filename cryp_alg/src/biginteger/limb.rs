use cryp_std::fmt::Debug;
use cryp_std::hash::Hash;
use cryp_std::rand::UniformRand;

pub trait Limb:
    Sized + From<u32> + Copy + Clone + Eq + Debug + Send + Sync + Hash + UniformRand + PartialOrd + Ord
{
    /// The type used to represent a carry bit.
    type Carry : PartialEq + Eq + Copy + Clone + Debug;

    const NO : Self::Carry;

    const ZERO: Self;
    const ONE: Self;

    ///  Calculates `self + rhs + carry` without the ability to overflow.
    ///
    /// Performs "ternary addition" which takes in an extra bit to add, and may return an
    /// additional bit of overflow. This allows for chaining together multiple additions
    /// to create "big integers" which represent larger values.
    fn carrying_add(&self, rhs: Self, carry: Self::Carry) -> (Self, Self::Carry);

    ///  Calculates `self - rhs - carry` without the ability to overflow.
    fn carrying_sub(&self, rhs: Self, carry: Self::Carry) -> (Self, Self::Carry);
}

impl Limb for u32 {
    type Carry = bool;
    const ZERO: Self = 0;
    const ONE: Self = 1;

    const NO :bool = false;

    fn carrying_add(&self, rhs: Self, carry: Self::Carry) -> (Self, Self::Carry) {
        let (a, b) = self.overflowing_add(rhs);
        let (c, d) = a.overflowing_add(carry as u32);
        (c, b || d)
    }
    fn carrying_sub(&self, rhs: Self, carry: Self::Carry) -> (Self, Self::Carry) {
        let (a, b) = self.overflowing_sub(rhs);
        let (c, d) = a.overflowing_sub(carry as u32);
        (c, b || d)
    }
}

impl Limb for u64 {
    type Carry = bool;
    const ZERO: Self = 0;
    const ONE: Self = 1;

    const NO :bool = false;

    fn carrying_add(&self, rhs: Self, carry: Self::Carry) -> (Self, Self::Carry) {
        let (a, b) = self.overflowing_add(rhs);
        let (c, d) = a.overflowing_add(carry as u64);
        (c, b || d)
    }

    fn carrying_sub(&self, rhs: Self, carry: Self::Carry) -> (Self, Self::Carry) {
        let (a, b) = self.overflowing_sub(rhs);
        let (c, d) = a.overflowing_sub(carry as u64);
        (c, b || d)
    }
}


impl Limb for u128 {
    type Carry = bool;
    const ZERO: Self = 0;
    const ONE: Self = 1;

    const NO :bool = false;

    fn carrying_add(&self, rhs: Self, carry: Self::Carry) -> (Self, Self::Carry) {
        let (a, b) = self.overflowing_add(rhs);
        let (c, d) = a.overflowing_add(carry as u128);
        (c, b || d)
    }
    fn carrying_sub(&self, rhs: Self, carry: Self::Carry) -> (Self, Self::Carry) {
        let (a, b) = self.overflowing_sub(rhs);
        let (c, d) = a.overflowing_sub(carry as u128);
        (c, b || d)
    }
}

// -----------------------------------

///  Calculates `self + rhs + carry` without the ability to overflow.
///
/// Performs "ternary addition" which takes in an extra bit to add, and may return an
/// additional bit of overflow. This allows for chaining together multiple additions
/// to create "big integers" which represent larger values.
///
/// The implementation currently matches the currently nighly Rust implementation (which may change).
pub const fn carrying_add(element: u64, rhs: u64, carry: bool) -> (u64, bool) {
    // note: longer-term this should be done via an intrinsic, but
    //   this has been shown to generate optimal code for now, and
    //   LLVM doesn't have an equivalent intrinsic
    let (a, b) = element.overflowing_add(rhs);
    let (c, d) = a.overflowing_add(carry as u64);
    (c, b || d)
}

#[cfg(test)]
mod tests {
    // test the carry_add function
    use super::*;
    #[test]
    fn test_adc() {
        assert_eq!(carrying_add(0, 0, false), (0, false));
        assert_eq!(carrying_add(0, 0, true), (1, false));
        assert_eq!(carrying_add(0, 1, false), (1, false));
        assert_eq!(carrying_add(0, 1, true), (2, false));
        assert_eq!(carrying_add(1, 0, false), (1, false));
        assert_eq!(carrying_add(u64::MAX, 1, false), (0, true));
    }
}
