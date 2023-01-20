//! The module this crate uses to create and mutate/evolve genomes

mod connection_gene;
#[allow(clippy::module_inception)]
mod genome;
mod node_gene;

pub use connection_gene::ConnectionGene;
pub use genome::Genome;
pub use node_gene::NodeGene;
