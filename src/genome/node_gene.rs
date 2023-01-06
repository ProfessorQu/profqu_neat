use crate::data_structures::pseudo_float::PseudoFloat;

/// The node gene
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct NodeGene {
    pub innovation_number: u32,
    pub x: PseudoFloat,
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