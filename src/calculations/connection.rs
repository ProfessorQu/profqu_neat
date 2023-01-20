use super::Node;

use std::cell::RefCell;
use std::rc::Rc;

/// The connection for calculations
#[derive(PartialEq, Clone, Debug)]
pub struct Connection {
    /// Pointers to the nodes this connection is from
    pub from: Rc<RefCell<Node>>,
    /// The weight of this connection
    pub weight: f32,
    /// Whether this connection is enabled or not
    pub enabled: bool,
}

impl Connection {
    /// Create a new connection using a reference to a cell where it comes from
    pub fn new(from: Rc<RefCell<Node>>) -> Self {
        Self {
            from,
            weight: 1.0,
            enabled: true,
        }
    }
}
