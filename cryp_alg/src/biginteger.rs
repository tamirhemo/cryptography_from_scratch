mod limb;

use limb::Limb;

pub trait Integer {
    fn into_bits_be(&self) -> &[bool];
    fn into_bits_le(&self) -> &[bool];
}
