use super::{Bytes, Integer, Limb};

/// A fixed size big-precision integer type
#[derive(Debug, Clone, Copy)]
pub struct LimbInt<L: Limb, const N: usize> {
    pub limbs: [L; N],
}

impl<L: Limb, const N: usize> LimbInt<L, N> {
    #[inline]
    pub const fn zero() -> Self {
        Self::from_limbs([L::ZERO; N])
    }

    #[inline]
    pub const fn one() -> Self {
        let mut res = [L::ZERO; N];
        res[0] = L::ONE;
        Self::from_limbs(res)
    }

    #[inline]
    pub const fn from_limbs(limbs: [L; N]) -> Self {
        Self { limbs }
    }

    fn single_power(limb: L, i: usize) -> Self {
        let mut limbs = [L::ZERO; N];
        limbs[i] = limb;
        limbs.into()
    }

    /// Comparison in constant time (aspirationally)
    pub fn le(&self, other: &Self) -> bool {
        let mut res = true;
        let mut _dummy_res = true;
        let mut flag = true;
        for i in (0..N).rev() {
            if self.limbs[i] != other.limbs[i] {
                if flag {
                    res = self.limbs[i] < other.limbs[i];
                    flag = false;
                } else {
                    _dummy_res = self.limbs[i] < other.limbs[i];
                }
            }
        }
        res
    }

    pub fn le_non_ct(&self, other: &Self) -> bool {
        for i in (0..N).rev() {
            if self.limbs[i] != other.limbs[i] {
                return self.limbs[i] <= other.limbs[i];
            }
        }
        true
    }

    pub fn le_double(element: &(Self, Self), other: &(Self, Self)) -> bool {
        let high = element.1.le(&other.1);
        let low = element.0.le(&other.0);
        let equal = element.1 == other.1;

        high || (equal && low)
    }

    pub fn carrying_add(&self, rhs: Self, carry: L::Carry) -> (Self, L::Carry) {
        let mut carry = carry;
        let mut limbs = [L::ZERO; N];
        for i in 0..N {
            let (l, c) = self.limbs[i].add_carry(rhs.limbs[i], carry);
            limbs[i] = l;
            carry = c;
        }
        (limbs.into(), carry)
    }

    pub fn carrying_sub(&self, rhs: Self, carry: L::Carry) -> (Self, L::Carry) {
        let mut carry = carry;
        let mut limbs = [L::ZERO; N];
        for i in 0..N {
            let (l, c) = self.limbs[i].sub_carry(rhs.limbs[i], carry);
            limbs[i] = l;
            carry = c;
        }
        (limbs.into(), carry)
    }

    pub fn carrying_mul(&self, rhs: Self, carry: Self) -> (Self, Self) {
        let mut w_l = [L::ZERO; N];
        let mut w_h = [L::ZERO; N];

        for i in 0..N {
            let mut c = L::ZERO;
            for j in 0..(N - i) {
                let (v_1, u_1) = self.limbs[j].mul_carry(rhs.limbs[i], c);
                let (v, temp) = v_1.add_carry(w_l[i + j], L::NO);
                let (u, zer) = u_1.add_carry(L::ZERO, temp);
                debug_assert!(zer == L::NO);

                w_l[i + j] = v;
                c = u;
            }
            for j in (N - i)..N {
                let (v_1, u_1) = self.limbs[j].mul_carry(rhs.limbs[i], c);
                let (v, temp) = v_1.add_carry(w_h[i + j - N], L::NO);
                let (u, zer) = u_1.add_carry(L::ZERO, temp);
                debug_assert!(zer == L::NO);

                w_h[i + j - N] = v;
                c = u;
            }
            w_h[i] = c;
        }
        let (mut res_l, mut res_h) = (Self::from(w_l), Self::from(w_h));
        let flag;
        (res_l, flag) = res_l.carrying_add(carry, L::NO);

        // Non-constant time issue here
        let mut temp = [L::ZERO; N];
        if flag != L::NO {
            temp[0] = L::ONE;
        }
        let (h, z) = res_h.carrying_add(Self::from_limbs(temp), L::NO);
        assert_eq!(z, L::NO);
        res_h = h;
        (res_l, res_h)
    }

    pub fn mul_by_limb(&self, rhs: L) -> (Self, L) {
        let mut carry = L::ZERO;
        let mut limbs = [L::ZERO; N];
        for i in 0..N {
            let (l, c) = self.limbs[i].mul_carry(rhs, carry);
            limbs[i] = l;
            carry = c;
        }
        (limbs.into(), carry)
    }

    pub fn mul_by_limb_carry(&self, rhs: L, carry: L) -> (Self, L) {
        let mut carry = carry;
        let mut limbs = [L::ZERO; N];
        for i in 0..N {
            let (l, c) = self.limbs[i].mul_carry(rhs, carry);
            limbs[i] = l;
            carry = c;
        }
        (limbs.into(), carry)
    }

    /// Multplying by an element of the form r * b^index
    ///
    ///  Might be replaced with a slighly more efficient implementation
    pub fn mul_by_limb_shift(&self, rhs: L, index: usize) -> (Self, Self) {
        let other = Self::single_power(rhs, index);
        self.carrying_mul(other, Self::zero())
    }
}

impl<L: Limb, const N: usize> From<[L; N]> for LimbInt<L, N> {
    fn from(limbs: [L; N]) -> Self {
        Self { limbs }
    }
}

impl<L: Limb, const N: usize> Integer for LimbInt<L, N> {
    type Limb = L;
    fn into_limbs_le(&self) -> &[Self::Limb] {
        &self.limbs
    }
}

impl<L: Limb, const N: usize> cryp_std::fmt::Display for LimbInt<L, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "LimbInt({:?})", self.limbs)
    }
}

impl<L: Limb, const N: usize> PartialEq for LimbInt<L, N> {
    fn eq(&self, other: &Self) -> bool {
        self.limbs == other.limbs
    }
}

impl<L: Limb, const N: usize> Eq for LimbInt<L, N> {}

impl<L: Limb, const N: usize> cryp_std::hash::Hash for LimbInt<L, N> {
    fn hash<H: cryp_std::hash::Hasher>(&self, state: &mut H) {
        self.limbs.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cryp_std::vec::Vec;
    use num_bigint::BigUint;
    pub type LimbInt64 = LimbInt<u32, 2>;

    // Conversion to BigUint from the num_bigint crate
    impl<L: Limb, const N: usize> From<&LimbInt<L, N>> for BigUint {
        fn from(value: &LimbInt<L, N>) -> Self {
            let bytes_be: Vec<u8> = Bytes::into_iter_be(value).collect();
            Self::from_bytes_be(bytes_be.as_slice())
        }
    }

    impl<L: Limb, const N: usize> From<LimbInt<L, N>> for BigUint {
        fn from(value: LimbInt<L, N>) -> Self {
            Self::from(&value)
        }
    }

    fn to_u64(element: &LimbInt64) -> u64 {
        element.limbs[0] as u64 + ((element.limbs[1] as u64) << 32)
    }

    #[test]
    fn test_to_u64() {
        let a = LimbInt64::from([1u32, 2u32]);
        let a_64 = to_u64(&a);
        assert_eq!(a_64, 1 + (2u64 << 32));
        assert_eq!(a_64 / (u32::MAX as u64), 2);
        assert_eq!(a_64 % (u32::MAX as u64 + 1), 1);
    }

    #[test]
    fn test_bigunit_conversion() {
        let a_int = LimbInt64::from([1u32, 2u32]);
        let a = BigUint::from(a_int);

        assert_eq!(BigUint::from(a_int), BigUint::from(1u64 + (2u64 << 32)));
    }

    #[test]
    fn test_carrying_add() {
        let a_0 = LimbInt64::from([1u32, 1u32]);
        let b_0 = LimbInt64::from([1u32, 0u32]);
        let (res_0, c_0) = a_0.carrying_add(b_0, false);

        assert_eq!(res_0, LimbInt64::from([2u32, 1u32]));
        assert_eq!(c_0, false);

        let real_add = to_u64(&a_0) + to_u64(&b_0);
        assert_eq!(real_add, to_u64(&res_0));

        let a = LimbInt64::from([u32::MAX, u32::MAX]);
        let b = LimbInt64::from([100u32, 0u32]);
        let (res, carry) = a.carrying_add(b, false);
        let (real_add, c) = to_u64(&a).add_carry(to_u64(&b), false);
        assert_eq!(real_add, to_u64(&res));
        assert_eq!(carry, c);

        // compare with biguint
        let a_array = [u32::MAX, u32::MAX, u32::MAX, u32::MAX, 5, 7, 7, u32::MAX];
        let b_array = [u32::MAX, u32::MAX, u32::MAX, u32::MAX, 5, 7, 7, u32::MAX];
        let a = LimbInt::<u32, 8>::from(a_array);
        let b = LimbInt::<u32, 8>::from(b_array);
        let a_big = BigUint::from_slice(&a_array);
        let b_big = BigUint::from_slice(&b_array);

        let (res, carry) = a.carrying_add(b, false);
        let res_big = a_big + b_big;

        assert_eq!(res.limbs.as_slice(), &res_big.to_u32_digits()[..8]);
        assert_eq!(carry, res_big.to_u32_digits().len() > 8);
    }

    #[test]
    fn test_carrying_sub() {
        let a_0 = LimbInt64::from([1u32, 1u32]);
        let b_0 = LimbInt64::from([1u32, 0u32]);
        let (res_0, c_0) = a_0.carrying_sub(b_0, false);

        assert_eq!(res_0, LimbInt64::from([0u32, 1u32]));
        assert_eq!(c_0, false);
        assert_eq!(to_u64(&res_0), to_u64(&a_0) - to_u64(&b_0));

        let a = LimbInt64::from([0, 0]);
        let b = LimbInt64::from([500u32, 0u32]);
        let (res, carry) = a.carrying_sub(b, false);
        let (real_add, c) = to_u64(&a).sub_carry(to_u64(&b), false);
        assert_eq!(real_add, to_u64(&res));
        assert_eq!(carry, c);
    }

    #[test]
    fn test_carrying_mul() {
        let a_0 = LimbInt64::from([1u32, 1u32]);
        let b_0 = LimbInt64::from([1u32, 0u32]);
        let (res_0, c_0) = a_0.carrying_mul(b_0, LimbInt64::zero());

        assert_eq!(res_0, LimbInt64::from([1u32, 1u32]));
        assert_eq!(c_0, LimbInt64::zero());

        let real_res = to_u64(&a_0) * to_u64(&b_0);
        assert_eq!(real_res, to_u64(&res_0));

        let a = LimbInt64::from([1000, u32::MAX]);
        let b = LimbInt64::from([100u32, 0u32]);
        let (res, carry) = a.carrying_mul(b, LimbInt64::zero());
        let (real_add, c) = to_u64(&a).mul_carry(to_u64(&b), 0);
        assert_eq!(real_add, to_u64(&res));
        assert_eq!(to_u64(&carry), c);

        // compare with biguint
        let a_array = [1, 2, 400, 10000, 5, 7, 7, u32::MAX];
        let b_array = [5, 199, u32::MAX, u32::MAX, 5, 7, 7, 10];
        let a = LimbInt::<u32, 8>::from(a_array);
        let b = LimbInt::<u32, 8>::from(b_array);
        let a_big = BigUint::from_slice(&a_array);
        let b_big = BigUint::from_slice(&b_array);

        let (res, carry) = a.carrying_mul(b, LimbInt::<u32, 8>::zero());
        let res_big = a_big * b_big;
        let modulus = BigUint::from_slice(&[
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
        ]) + 1 as u32;

        assert_eq!(res.limbs.as_slice(), (&res_big % &modulus).to_u32_digits());
        assert_eq!(
            carry.limbs.as_slice(),
            (&res_big / &modulus).to_u32_digits()
        );
    }

    #[test]
    fn test_mul_by_limb() {
        let a = LimbInt64::from([1000, u32::MAX]);
        let b = 100u32;
        let b_int = LimbInt64::single_power(b, 0);
        let (res, carry) = a.mul_by_limb(b);
        let (res_big, carry_big) = a.carrying_mul(b_int, LimbInt64::zero());
        assert_eq!(res, res_big);
        assert_eq!(carry_big, LimbInt64::single_power(carry, 0));
    }

    #[test]
    fn test_mul_by_limb_shift() {
        let a = LimbInt64::from([29394944, u32::MAX]);
        let b = 100u32;
        let i = 1;
        let b_int = LimbInt64::single_power(b, i);
        let (res, carry) = a.mul_by_limb_shift(b, i);
        let (res_big, carry_big) = a.carrying_mul(b_int, LimbInt64::zero());
        assert_eq!(res, res_big);
        assert_eq!(carry, carry_big);
    }

    #[test]
    fn test_equality() {
        let a = LimbInt64::from([1000, u32::MAX]);
        let b = LimbInt64::from([1000, u32::MAX]);
        assert_eq!(a, b);
    }

    #[test]
    fn test_bigint() {
        use cryp_std::rand::thread_rng;
        let mut rng = thread_rng();
        use crate::helper::big_int_from_u64;
        use cryp_std::rand::UniformRand;

        // test for u_32

        let a = LimbInt::<u32, 7>::from([
            u32::rand(&mut rng),
            u32::rand(&mut rng),
            u32::rand(&mut rng),
            u32::rand(&mut rng),
            u32::rand(&mut rng),
            u32::rand(&mut rng),
            u32::rand(&mut rng),
        ]);

        let b = LimbInt::<u32, 7>::from([
            u32::rand(&mut rng),
            u32::rand(&mut rng),
            u32::rand(&mut rng),
            u32::rand(&mut rng),
            u32::rand(&mut rng),
            u32::rand(&mut rng),
            u32::rand(&mut rng),
        ]);

        assert_ne!(a, b);

        let (product_l, product_r) = a.carrying_mul(b, LimbInt::<u32, 7>::zero());

        let product: Vec<u32> = product_l
            .limbs
            .into_iter()
            .chain(product_r.limbs.into_iter())
            .collect();

        let n_a = BigUint::from_slice(&a.limbs);
        let n_b = BigUint::from_slice(&b.limbs);
        let n_p = BigUint::from_slice(&product);

        assert_eq!(n_p, n_a * n_b);

        //  test for u64

        let c = LimbInt::<u64, 4>::from([
            u64::rand(&mut rng),
            u64::rand(&mut rng),
            u64::rand(&mut rng),
            u64::rand(&mut rng),
        ]);

        let d = LimbInt::<u64, 4>::from([
            u64::rand(&mut rng),
            u64::rand(&mut rng),
            u64::rand(&mut rng),
            u64::rand(&mut rng),
        ]);

        assert_ne!(c, d);

        let (product_l, product_r) = c.carrying_mul(d, LimbInt::<u64, 4>::zero());

        let product: Vec<u64> = product_l
            .limbs
            .into_iter()
            .chain(product_r.limbs.into_iter())
            .collect();

        let n_c = big_int_from_u64(&c.limbs);
        let n_d = big_int_from_u64(&d.limbs);
        let n_prod = big_int_from_u64(&product);

        assert_eq!(n_prod, n_c * n_d);
    }
}
