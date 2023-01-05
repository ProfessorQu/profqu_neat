use std::rc::Rc;

use crate::node::Node;

/// The link between nodes
pub struct Link {
    pub in_node: Box<Node>,
    pub out_node: Box<Node>,
    pub weight: f32,
}