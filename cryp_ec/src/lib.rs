#![cfg_attr(not(feature = "std"), no_std)]

//! Elliptic Curve implementation
//!
//! The 'models` module contains the implementations of the elliptic curve models.

mod curves;
mod models;

mod common {
    use super::*;
    pub use models::{GroupEC, PrimeGroupConfig, PrimeSubGroupConfig};
    pub use cryp_alg::{Group, PrimeGroup};
}

pub mod weierstrass {
    use super::*;
    pub use models::ShortWeierstrass;
    pub use common::*;
}

pub mod edwards {
    use super::*;
    pub use models::{TwistedEdwardsAM1, EdwardsAM1UnifiedOperations};
    pub use common::*;
}
