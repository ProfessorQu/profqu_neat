use std::fmt::Debug;

use crate::neat;
use super::node_gene::NodeGene;
use crate::data_structures::PseudoFloat;

/// The connection gene
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ConnectionGene {
    /// The innovation number of this connection gene
    pub innovation_number: u32,
    /// The node gene which it comes from
    pub from: NodeGene,
    /// The node gene which it goes to
    pub to: NodeGene,
    /// The weight of this connection
    pub weight: PseudoFloat,
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
            weight: PseudoFloat::new(1.0),
            enabled: true,
            replace_index: 0
        }
    }

    /// Get the hash code of this connection gene
    pub fn hash_code(&self) -> u64 {
        u64::from(self.from.innovation_number) * neat::MAX_NODES + u64::from(self.to.innovation_number)
    }
}

impl Debug for ConnectionGene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Connection({:?}, from: {:?}, to: {:?}, weight: {:?}, enabled: {:?}, replace_index: {:?})",
            self.innovation_number, self.from.innovation_number, self.to.innovation_number, self.weight, self.enabled, self.replace_index)
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_hash_code() {
        for _ in 0..10 {
            let from_innov: u32 = rand::thread_rng().gen_range(0..100);
            let to_innov: u32 = rand::thread_rng().gen_range(0..100);

            let from = NodeGene::new(from_innov);
            let to = NodeGene::new(to_innov);
    
            let connection = ConnectionGene::new(from, to);
    
            assert_eq!(connection.hash_code(), u64::from(from_innov) * neat::MAX_NODES + u64::from(to_innov));
        }
    }
}