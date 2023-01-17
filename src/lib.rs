//! A crate that implements the NEAT algorithm
//! Created according to a [tutorial](https://www.youtube.com/playlist?list=PLgomWLYGNl1fcL0o4exBShNeCC5tc6s9C)
//! from [Finn Eggers](https://www.youtube.com/@finneggers6612).
//! I tried to implement NEAT from the official github [repository](https://github.com/f3270/NEAT),
//! but I couldn't figure out how to do it, so I used Finn's implementation.
//! 
//! Then I looked on Youtube and found Finn Eggers and his tutorial really helped me with creating this library.
//! 
//! Doesn't allow recurrent connections in the networks.

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