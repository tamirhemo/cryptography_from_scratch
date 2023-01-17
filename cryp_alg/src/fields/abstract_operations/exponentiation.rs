//! Interfaces and algorithms for exponentiation
//! 
//! 

use super::{Integer, ArithmeticOperations};
use crate::biginteger::Bits;
use cryp_std::fmt::Debug;

pub trait Exponentiation<A : ArithmeticOperations> : 'static + Debug + Send + Sync {
    /// Exponentiation of an element.
    ///
    ///  Default implementation is based on the Montgomery ladder algorithm and runs
    /// in constant time depending only on the length of exp.to_bits_be().
    /// Thus, the user must make sure all secret exponents have the same bit length.
    fn exp(element: &A::BigInt, exp: &impl Integer) -> A::BigInt {
        let mut res = A::one();
        let mut base = *element;

        let bits = Bits::into_iter_be(exp);
        for bit in bits {
            if bit {
                A::mul_assign(&mut res, &base);
                A::square_assign(&mut base);
            } else {
                A::mul_assign(&mut base, &res);
                A::square_assign(&mut res);
            }
        }
        res
    }
}


