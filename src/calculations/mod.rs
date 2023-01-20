//! The calculations required to calculate any genomes output
//!
//! It contains a `Calculator` which is created by a client to calculate it's genome's output with some input.
//! The `Connection` and `Node` structs are used in the `Calculator` to calculate the outputs.

mod calculator;
mod connection;
mod node;

pub use calculator::Calculator;
pub use connection::Connection;
pub use node::Node;
