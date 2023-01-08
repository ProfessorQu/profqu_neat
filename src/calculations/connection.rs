use crate::data_structures::PseudoFloat;
use super::Node;

use std::cell::RefCell;
use std::rc::Rc;

/// The connection for calculations
#[derive(PartialEq, Clone, Debug)]
pub struct Connection {
    pub from: Rc<RefCell<Node>>,
    pub weight: PseudoFloat,
    pub enabled: bool,
}

impl Connection {
    pub fn new(from: Rc<RefCell<Node>>) -> Self {
        Self {
            from,
            weight:PseudoFloat::new(1.0),
            enabled: true
        }
    }
}