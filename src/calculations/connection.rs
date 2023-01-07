use crate::neat;
use crate::data_structures::PseudoFloat;
use super::Node;

/// The connection for calculations
#[derive(PartialEq, Clone, Debug)]
pub struct Connection {
    pub from: Node,
    pub to: Node,
    pub weight: PseudoFloat,
    pub enabled: bool,
}

impl Connection {
    /// Create a new connection gene
    pub fn new(from: Node, to: Node) -> Self {
        Self {
            from,
            to,
            weight: PseudoFloat::new(1.0),
            enabled: true,
        }
    }
}