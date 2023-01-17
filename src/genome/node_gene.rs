use std::{fmt::Debug, hash::Hash};

/// The node gene of some genome
#[derive(Clone, Copy)]
pub struct NodeGene {
    /// The innovation number of this gene
    pub innovation_number: u32,
    /// The x coordinate of this gene, used for checking that we aren't creating recurrent connections and drawing the neural network
    pub x: f32,
    /// The y coordinate of this gene, used for drawing the neural network
    pub y: f32,
}

impl NodeGene {
    /// Create a new node gene
    pub fn new(innovation_number: u32) -> Self {
        Self {
            innovation_number,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl Debug for NodeGene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node({:?}, x: {:?}, y: {:?})", self.innovation_number, self.x, self.y)
    }
}

impl PartialEq for NodeGene {
    fn eq(&self, other: &Self) -> bool {
        self.innovation_number == other.innovation_number
    }
}

impl Eq for NodeGene {}

impl Hash for NodeGene {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.innovation_number.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::hash_map::DefaultHasher, hash::Hasher};

    use rand::Rng;

    use super::*;

    #[test]
    fn test_hash() {
        let innov: u32 = rand::thread_rng().gen_range(0..100);
        
        let from = NodeGene::new(innov);
        let to = NodeGene::new(innov);
        
        let mut state_from = DefaultHasher::new();

        from.hash(&mut state_from);
        let result_from = state_from.finish();

        let mut state_to = DefaultHasher::new();
        to.hash(&mut state_to);
        let result_to = state_to.finish();

        assert_eq!(result_from, result_to);
    }
}