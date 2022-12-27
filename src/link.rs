use crate::node::Node;

/// The link between nodes
pub struct Link {
    pub in_node: Node,
    pub out_node: Node,
    pub weight: f32,
}