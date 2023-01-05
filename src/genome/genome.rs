use super::{connection_gene::ConnectionGene, node_gene::NodeGene};
use crate::{data_structures::random_hash_set::RandomHashSet, neat::Neat};

/// Teh genome with the connections and nodes
struct Genome<'a> {
    connections: RandomHashSet<ConnectionGene>,
    nodes: RandomHashSet<NodeGene>,
    neat: &'a Neat
}

impl<'a> Genome<'a> {
    /// Create a new genome
    pub fn new(neat: &'a Neat) -> Self {
        Self {
            connections: RandomHashSet::new(),
            nodes: RandomHashSet::new(),
            neat
        }
    }

    /// Calculate the distance between this and another genome
    pub fn distance(&self, other: Genome) -> f32 {
        0.0
    }

    /// Crossover two genomes
    pub fn crossover(g1: Genome<'a>, g2: Genome) -> Self {
        Self::new(g1.neat)
    }

    /// Mutate this genome
    pub fn mutate(&self) {

    }
}