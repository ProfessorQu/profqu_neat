//! The module this crate uses to create and mutate/evolve genomes

mod node_gene;
mod connection_gene;
#[allow(clippy::module_inception)]
mod genome;


pub use node_gene::NodeGene;
pub use connection_gene::ConnectionGene;
pub use genome::Genome;