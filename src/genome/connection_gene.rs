use crate::neat;

use super::node_gene::NodeGene;

use crate::data_structures::pseudo_float::PseudoFloat;

/// The connection gene
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ConnectionGene {
    pub innovation_number: u32,
    from: NodeGene,
    to: NodeGene,
    weight: PseudoFloat,
    enabled: bool,
}

impl ConnectionGene {
    /// Create a new connection gene
    pub fn new(from: NodeGene, to: NodeGene) -> Self {
        Self {
            innovation_number: ConnectionGene::calculate_hash_code(&from, &to),
            from,
            to,
            weight: PseudoFloat::new(1.0),
            enabled: true,
        }
    }

    /// Get the hash code of the connection gene
    pub fn hash_code(&self) -> u32 {
        ConnectionGene::calculate_hash_code(&self.from, &self.to)
    }

    /// Calculate the hash code according to the innovation numbers from the to and from genes
    fn calculate_hash_code(from: &NodeGene, to: &NodeGene) -> u32 {
        from.innovation_number * neat::MAX_NODES + to.innovation_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_hash_code() {
        for _ in 0..10 {
            let from_innov: u32 = rand::random();
            let to_innov: u32 = rand::random();

            let from = NodeGene::new(from_innov);
            let to = NodeGene::new(to_innov);
    
            let connection = ConnectionGene::new(from, to);
    
            assert_eq!(connection.hash_code(), from_innov * neat::MAX_NODES + to_innov);
        }
    }
}