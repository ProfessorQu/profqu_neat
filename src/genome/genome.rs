use super::{connection_gene::ConnectionGene, node_gene::NodeGene};
use crate::data_structures::random_hash_set::RandomHashSet;
use crate::neat::Neat;

/// Teh genome with the connections and nodes
pub struct Genome {
    pub connections: RandomHashSet<ConnectionGene>,
    pub nodes: RandomHashSet<NodeGene>
}

impl Genome {
    /// Create a new genome
    pub fn new() -> Self {
        Self {
            connections: RandomHashSet::new(),
            nodes: RandomHashSet::new()
        }
    }

    /// Calculate the distance between this and another genome
    pub fn distance(&self, other: Genome) -> f32 {
        0.0
    }

    /// Crossover two genomes
    pub fn crossover(g1: Self, g2: Self) -> Self {
        // Self::new(g1.neat)
        g1
    }

    /// Mutate this genome
    pub fn mutate(&self) {

    }
}