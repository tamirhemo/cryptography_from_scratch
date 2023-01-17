
use cryp_std::fmt::Debug;

use super::{ArithmeticOperations};

pub trait Inversion<A : ArithmeticOperations> : 'static + Send + Sync + Debug {
    /// The multiplicative inverse of an element, if exists.
    fn inverse(element: &A::BigInt) -> Option<A::BigInt>;
}