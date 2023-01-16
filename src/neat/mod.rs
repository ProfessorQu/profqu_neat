//! The main module where all the evolution happens

#[allow(clippy::module_inception)]
mod neat;
mod species;
mod client;
mod config;

pub use neat::{Neat, MAX_NODES};
pub use species::Species;
pub use client::Client;
pub use config::{Config, ActivationFunction};