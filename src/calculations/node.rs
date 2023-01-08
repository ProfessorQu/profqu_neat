use std::{cell::RefCell, rc::Rc};

use crate::data_structures::PseudoFloat;

use super::Connection;

/// The node for calculations
#[derive(PartialEq, Clone, Debug)]
pub struct Node {
    pub x: PseudoFloat,
    pub output: PseudoFloat,
    pub connections: Vec<Rc<RefCell<Connection>>>,
}

impl Node {
    /// Create a new node
    pub fn new(x: f32) -> Self {
        Self {
            x: PseudoFloat::new(x),
            output: PseudoFloat::new(0.0),
            connections: Vec::new()
        }
    }

    /// Calculate a the output value
    pub fn calculate(&mut self) {
        let mut sum = 0.0;

        for connection in &self.connections {
            if connection.borrow().enabled {
                sum += connection.borrow().weight.parse() * connection.borrow().from.borrow().output.parse();
            }
        }

        self.output = Self::activation(sum).into();
    }

    /// The activation function
    fn activation(input: f32) -> f32 {
        1.0 / (1.0 + (-input).exp())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.x.parse().partial_cmp(&other.x.parse())
    }
}

impl Eq for Node { }

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.parse().total_cmp(&other.x.parse())
    }
}