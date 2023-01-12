use cryp_alg::Ring;
use cryp_alg::{Bits, Bytes};
use cryp_std::rand::thread_rng;
use num_bigint::BigUint;

pub struct RingTests<R: Ring>(cryp_std::marker::PhantomData<R>);

impl<R: Ring> RingTests<R> {
}
