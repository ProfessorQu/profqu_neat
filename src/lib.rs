//! A crate that implements the NEAT algorithm
//! Doesn't allow recurrent connections, but if you want to implement it, feel free to do so

#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::float_cmp)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

pub mod data_structures;
pub mod neat;
pub mod genome;
pub mod calculations;

pub use neat::Neat;