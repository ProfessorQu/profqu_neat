use crate::neat;
use crate::data_structures::PseudoFloat;
use super::Node;

use std::rc::Rc;
use std::cell::RefCell;

/// The connection for calculations
#[derive(PartialEq, Clone, Debug)]
pub struct Connection {
    pub from: RefCell<Node>,
    pub to: RefCell<Node>,
    pub weight: PseudoFloat,
    pub enabled: bool,
}

impl Connection {
    /// Create a new connection gene
    pub fn new(from: RefCell<Node>, to: RefCell<Node>) -> Self {
        Self {
            from,
            to,
            weight: PseudoFloat::new(1.0),
            enabled: true,
        }
    }
}