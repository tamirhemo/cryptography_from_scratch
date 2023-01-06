mod limb;

pub use limb::Limb;

pub trait Integer {
    fn into_bits_be(&self) -> &[bool];
    fn into_bits_le(&self) -> &[bool];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LimbInt<L: Limb, const N: usize> {
    pub limbs: [L; N],
}

impl<L: Limb, const N: usize> LimbInt<L, N> {
    pub fn carrying_add(&self, rhs: Self, carry: L::Carry) -> (Self, L::Carry) {
        let mut carry = carry;
        let mut limbs = [L::ZERO; N];
        for i in 0..N {
            let (l, c) = self.limbs[i].carrying_add(rhs.limbs[i], carry);
            limbs[i] = l;
            carry = c;
        }
        (limbs.into(), carry)
    }

    pub fn carrying_sub(&self, rhs: Self, carry: L::Carry) -> (Self, L::Carry) {
        let mut carry = carry;
        let mut limbs = [L::ZERO; N];
        for i in 0..N {
            let (l, c) = self.limbs[i].carrying_sub(rhs.limbs[i], carry);
            limbs[i] = l;
            carry = c;
        }
        (limbs.into(), carry)
    }
}

impl<L: Limb, const N: usize> From<[L; N]> for LimbInt<L, N> {
    fn from(limbs: [L; N]) -> Self {
        Self { limbs }
    }
}

impl<L: Limb, const N: usize> From<u32> for LimbInt<L, N> {
    fn from(value: u32) -> Self {
        let mut limbs = [L::ZERO; N];
        limbs[0] = L::from(value);
        limbs.into()
    }
}

impl<L: Limb, const N: usize> Integer for LimbInt<L, N> {
    fn into_bits_be(&self) -> &[bool] {
        unimplemented!()
    }

    fn into_bits_le(&self) -> &[bool] {
        unimplemented!()
    }
}

impl<L: Limb, const N: usize> cryp_std::fmt::Display for LimbInt<L, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "LimbInt({:?})", self.limbs)
    }
}
