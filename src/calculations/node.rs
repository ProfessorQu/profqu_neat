use std::{cell::RefCell, rc::Rc};

use crate::neat::{Config, ActivationFunction};

use super::Connection;

/// The node for calculations
#[derive(PartialEq, Clone, Debug)]
pub struct Node {
    /// The x coordinate of the node, used for checking if a connection would be recursive
    pub x: f32,
    /// The calculated output, calculated according to the incoming connections
    pub output: f32,
    /// The incoming connections to this node
    pub connections: Vec<Rc<RefCell<Connection>>>,
    activation: fn(f32) -> f32,
}

impl Node {
    /// Create a new node, with a certain x value
    pub fn new(x: f32) -> Self {
        Self {
            x,
            output: 0.,
            connections: Vec::new(),
            activation: match Config::global().activation {
                ActivationFunction::Relu => Self::relu_activation,
                ActivationFunction::Sigmoid => Self::sigmoid_activation,
            }
        }
    }

    /// Calculate a the output value given all it's inputs
    pub fn calculate(&mut self) {
        let mut sum = 0.0;

        for connection in &self.connections {
            if connection.borrow().enabled {
                sum += connection.borrow().weight * connection.borrow().from.borrow().output;
            }
        }

        self.output = (self.activation)(sum);
    }

    /// The `ReLu` activation function
    fn relu_activation(input: f32) -> f32 {
        if input <= 0.0 {
            0.0
        }
        else {
            input
        }
    }

    /// The sigmoid activation function
    fn sigmoid_activation(input: f32) -> f32 {
        1.0 / (1.0 + (-input).exp())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl Eq for Node { }

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.total_cmp(&other.x)
    }
}