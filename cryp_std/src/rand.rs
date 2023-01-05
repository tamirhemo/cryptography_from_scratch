#[doc(hidden)]
pub use rand::*;

pub use randtraits::UniformRand;

mod randtraits {
    use super::distributions::Standard;
    use super::prelude::*;

    pub trait UniformRand: Sized {
        fn rand<R: rand::RngCore + ?Sized>(rng: &mut R) -> Self;
    }

    impl<T> UniformRand for T
    where
        Standard: Distribution<T>,
    {
        fn rand<R: rand::RngCore + ?Sized>(rng: &mut R) -> Self {
            rng.gen()
        }
    }
}
