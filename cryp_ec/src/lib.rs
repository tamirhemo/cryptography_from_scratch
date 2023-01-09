#![cfg_attr(not(feature = "std"), no_std)]

//! An Elliptic Curve Library
//!
//! This library provides a set of traits and implementations for elliptic curves.
//!

pub mod curves;
mod models;

mod common {
    use super::*;
    pub use cryp_alg::{Group, PrimeGroup};
    pub use models::{
        Coordinates,
        Affine, ExtendedPoint, GroupEC, JacobianPoint, PrimeGroupConfig, PrimeSubGroupConfig,
        Projective, PublicEC,
    };
}

pub mod weierstrass {
    use super::*;
    pub use common::*;
    pub use models::ShortWeierstrass;
}

pub mod edwards {
    use super::*;
    pub use common::*;
    pub use models::{EdwardsAM1UnifiedOperations, TwistedEdwardsAM1, };
}


