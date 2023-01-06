#![cfg_attr(not(feature = "std"), no_std)]

//! Elliptic Curve implementation
//!
//! The 'models` module contains the implementations of the elliptic curve models.

use cryp_alg::{Field, Group, PrimeGroup};

mod curves;
mod models;
