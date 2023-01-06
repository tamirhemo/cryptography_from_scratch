#![cfg_attr(not(feature = "std"), no_std)]

//! Elliptic Curve implementation

use cryp_alg::{PrimeGroup, Field, Group};

mod curves;
mod models;


pub trait Curve {
    type Field: Field;
    type Point : Group;

    type Group : PrimeGroup;
}


