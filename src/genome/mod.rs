mod node_gene;
mod connection_gene;
#[allow(clippy::module_inception)]
mod genome;
mod gene;

pub use node_gene::NodeGene;
pub use connection_gene::ConnectionGene;
pub use genome::Genome;
pub use gene::Gene;