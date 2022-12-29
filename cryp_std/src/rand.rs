#[doc(hidden)]
pub use rand::*;

pub trait UniformRand: Sized {
    fn rand<R: rand::RngCore + ?Sized>(rng: &mut R) -> Self;
}