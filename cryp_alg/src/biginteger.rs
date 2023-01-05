mod limb;

use limb::Limb;

pub struct BigInteger<D: Limb, const N: usize> {
    limbs: [D; N],
}
