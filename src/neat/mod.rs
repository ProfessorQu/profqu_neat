//! The main module where all the evolution happens

mod client;
mod config;
#[allow(clippy::module_inception)]
mod neat;
mod species;

pub use client::Client;
pub use config::{ActivationFunction, Config};
pub use neat::{Neat, MAX_NODES};
pub use species::Species;
