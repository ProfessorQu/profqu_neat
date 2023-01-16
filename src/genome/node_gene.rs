use std::fmt::Debug;

use crate::data_structures::PseudoFloat;

/// The node gene
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct NodeGene {
    /// The innovation number of this gene
    pub innovation_number: u32,
    /// The x coordinate of this gene, used for checking that we aren't creating recurrent connections and drawing the neural network
    pub x: PseudoFloat,
    /// The y coordinate of this gene, used for drawing the neural network
    pub y: PseudoFloat,
}

impl NodeGene {
    /// Create a new node gene
    pub fn new(innovation_number: u32) -> Self {
        Self {
            innovation_number,
            x: PseudoFloat::new(0.0),
            y: PseudoFloat::new(0.0),
        }
    }

    /// Get the hash code of this gene
    pub fn hash_code(&self) -> u32 {
        self.innovation_number
    }
}

impl Debug for NodeGene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node({:?}, x: {:?}, y: {:?})", self.innovation_number, self.x, self.y)
    }
}