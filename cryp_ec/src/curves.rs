//! Implementations of particulat elliptic curve groups 
//! 
//! 
//! Currently we have an implementation of the [EdSDA Edwards cuve](https://www.rfc-editor.org/rfc/rfc8032#page-16) over the prime field modulus `2^255 - 19`.
//!    ```rust
//!    use cryp_ec::curves::edwards25519::*;
//!    use cryp_std::rand::thread_rng;
//!    
//!    let mut rng = thread_rng();
//!    let g = GroupEd25519::generator(Some(&mut rng));
//!    ```
//! This gives a generator for the prime order subgroup of the curve in affine coordinates. 
//! If we want to perform group operations, we can convert the point to the internal coordinate
//! system being used, in this case, the extended twisted Edwards coordinates.
//! 
//!    ```rust
//!   # use cryp_ec::curves::edwards25519::*;
//!   # use cryp_std::rand::thread_rng;
//!   # 
//!   # let mut rng = thread_rng();
//!   # let g = GroupEd25519::generator(Some(&mut rng));
//!    
//!    let g : GroupEd25519 = g.into();
//!    let identity = GroupEd25519::identity();
//!    
//!    assert_eq!(identity, identity + identity);
//!    assert_eq!(g, g + identity);
//!    assert_ne!(g, identity);
//! 
//!    ```
//! 
//!  We can also check that the group element has the correct order.
//! 
//! 
//! //!    ```rust
//!   # use cryp_ec::curves::edwards25519::*;
//!   # use cryp_std::rand::thread_rng;
//!   # 
//!   # let mut rng = thread_rng();
//!   # let g = GroupEd25519::generator(Some(&mut rng));
//!   #
//!   # let g : GroupEd25519 = g.into();
//!   # let identity = GroupEd25519::identity();
//!   # 
//!    let order = GroupEd25519::ScalarFied::MODULUS;
//!    let one = GroupEd25519::ScalarFied::one();
//! 
//!    assert_eq!(identity, g*&order);
//!    assert_ne!(identity, g*&(order-one));
//! 
//!    ```


mod bls12_318;
mod ed25519;


pub mod edwards25519 {
    use super::*;
    pub use ed25519::{GroupEd25519, Fp25519, ScalarEd25519};
    pub use crate::common::*;
}