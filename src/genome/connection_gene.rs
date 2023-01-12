use std::fmt::Debug;

use crate::neat;
use super::{node_gene::NodeGene, gene::Gene};
use crate::data_structures::PseudoFloat;

/// The connection gene
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ConnectionGene {
    pub innovation_number: u32,
    pub from: NodeGene,
    pub to: NodeGene,
    pub weight: PseudoFloat,
    pub enabled: bool,
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
        self.from.innovation_number as u64 * neat::MAX_NODES + self.to.innovation_number as u64
    }
}

impl Gene for ConnectionGene {
    fn get_innovation_number(&self) -> u32 {
        self.innovation_number
    }

    fn set_innovation_number(&mut self, new: u32) {
        self.innovation_number = new
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
    
            assert_eq!(connection.hash_code(), from_innov as u64 * neat::MAX_NODES + to_innov as u64);
        }
    }
}