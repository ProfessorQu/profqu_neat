use std::{fmt::Debug, hash::Hash};

use crate::neat;

use super::node_gene::NodeGene;

/// The connection gene of some genome
#[derive(Clone, Copy)]
pub struct ConnectionGene {
    /// The innovation number of this connection gene
    pub innovation_number: u32,
    /// The node gene which it comes from
    pub from: NodeGene,
    /// The node gene which it goes to
    pub to: NodeGene,
    /// The weight of this connection
    pub weight: f32,
    /// Whether this gene is enabled
    pub enabled: bool,
    /// The index for getting this connection instead of creating a new connection
    pub replace_index: usize,
}

impl ConnectionGene {
    /// Create a new connection gene
    pub fn new(from: NodeGene, to: NodeGene) -> Self {
        Self {
            innovation_number: 0,
            from,
            to,
            weight: 1.0,
            enabled: true,
            replace_index: 0,
        }
    }

    /// Get the hash code of this function
    pub fn hash_code(&self) -> u64 {
        u64::from(self.from.innovation_number) * neat::MAX_NODES
            + u64::from(self.to.innovation_number)
    }
}

impl Debug for ConnectionGene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Connection({:?}, from: {:?}, to: {:?}, weight: {:?}, enabled: {:?}, replace_index: {:?})",
            self.innovation_number, self.from, self.to, self.weight, self.enabled, self.replace_index)
    }
}

impl PartialEq for ConnectionGene {
    fn eq(&self, other: &Self) -> bool {
        self.innovation_number == other.innovation_number
            && self.enabled == other.enabled
            && self.from == other.from
            && self.to == other.to
            && self.weight == other.weight
            && self.replace_index == other.replace_index
    }
}

impl Eq for ConnectionGene {}

impl Hash for ConnectionGene {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.from.innovation_number.hash(state);
        self.to.innovation_number.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::hash_map::DefaultHasher, hash::Hasher};

    use rand::Rng;

    use super::*;

    #[test]
    fn test_hash() {
        for _ in 0..10 {
            let from_innov: u32 = rand::thread_rng().gen_range(0..100);
            let to_innov: u32 = rand::thread_rng().gen_range(0..100);

            let from = NodeGene::new(from_innov);
            let to = NodeGene::new(to_innov);

            let connection = ConnectionGene::new(from, to);
            let mut state_connection = DefaultHasher::new();

            connection.hash(&mut state_connection);
            let result_connection = state_connection.finish();

            let connection = ConnectionGene::new(from, to);
            let mut state_nodes = DefaultHasher::new();

            connection.from.hash(&mut state_nodes);
            connection.to.hash(&mut state_nodes);

            let result_nodes = state_nodes.finish();

            assert_eq!(result_connection, result_nodes);
        }
    }
}
