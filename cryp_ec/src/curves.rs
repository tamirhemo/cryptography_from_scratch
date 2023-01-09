mod bls12_318;
mod ed25519;


pub mod edwards25519 {
    use super::*;
    pub use ed25519::{GroupEd25519, Fp25519, ScalarEd25519};
}