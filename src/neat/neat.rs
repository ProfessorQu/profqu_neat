use std::{collections::HashMap, borrow::BorrowMut, ops::Deref};

use crate::{genome::{*, connection_gene::ConnectionGene, node_gene::NodeGene}, data_structures::random_hash_set::RandomHashSet};

pub const MAX_NODES: u32 = 2^20;

/// The struct that controls the entire library
pub struct Neat {
    all_connections: HashMap<ConnectionGene, ConnectionGene>,
    all_nodes: Vec<NodeGene>,
    input_size: u32,
    output_size: u32,
    population_size: u32
}

impl Neat {
    /// Create a new neat struct
    pub fn new(input_size: u32, output_size: u32, population_size: u32) -> Self {
        let mut neat = Self {
            all_connections: HashMap::new(),
            all_nodes: Vec::new(),
            input_size,
            output_size,
            population_size
        };

        neat.reset(input_size, output_size, population_size);
        neat
    }

    /// Reset this neat struct
    pub fn reset(&mut self, input_size: u32, output_size: u32, population_size: u32) {
        self.input_size = input_size;
        self.output_size = output_size;
        self.population_size = population_size;

        self.all_connections.clear();
        self.all_nodes.clear();

        for input_index in 0..input_size as usize {
            let y = (input_index + 1) as f32 / (input_size + 1) as f32;
            let mut node = self.create_node(0.1, y);
        }

        for output_index in 0..output_size as usize {
            let y = (output_index + 1) as f32 / (output_size + 1) as f32;
            let mut node = self.create_node(0.9, y);
        }
    }

    /// Create a new node with a certain x and y coordinate
    pub fn create_node(&mut self, x: f32, y: f32) {
        let mut node = NodeGene::new(self.all_nodes.len() as u32 + 1);
        node.x = x.into();
        node.y = y.into();
        self.all_nodes.push(node);
    }

    /// Get a new node
    pub fn get_node(&mut self, index: usize) -> &mut NodeGene {
        let len = self.all_nodes.len();
        if index <= len {
            &mut self.all_nodes[index - 1]
        }
        else {
            self.create_node(0.0, 0.0);
            &mut self.all_nodes[len - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let neat = Neat::new(3, 3, 2);
        assert_eq!(neat.all_nodes.len(), 6);

        assert_eq!(neat.input_size, 3);
        assert_eq!(neat.output_size, 3);
        assert_eq!(neat.population_size, 2);
    }

    #[test]
    fn test_inputs() {
        let neat = Neat::new(3, 3, 2);

        let x: f32 = neat.all_nodes[0].x.clone().into();
        let y: f32 = neat.all_nodes[0].y.clone().into();
        assert_eq!(x, 0.1);
        assert_eq!(y, 0.25);
        
        let x: f32 = neat.all_nodes[1].x.clone().into();
        let y: f32 = neat.all_nodes[1].y.clone().into();
        assert_eq!(x, 0.1);
        assert_eq!(y, 0.5);
        
        let x: f32 = neat.all_nodes[2].x.clone().into();
        let y: f32 = neat.all_nodes[2].y.clone().into();
        assert_eq!(x, 0.1);
        assert_eq!(y, 0.75);
    }

    #[test]
    fn test_outputs() {
        let neat = Neat::new(3, 3, 2);

        let x: f32 = neat.all_nodes[3].x.clone().into();
        let y: f32 = neat.all_nodes[3].y.clone().into();
        assert_eq!(x, 0.9);
        assert_eq!(y, 0.25);
        
        let x: f32 = neat.all_nodes[4].x.clone().into();
        let y: f32 = neat.all_nodes[4].y.clone().into();
        assert_eq!(x, 0.9);
        assert_eq!(y, 0.5);
        
        let x: f32 = neat.all_nodes[5].x.clone().into();
        let y: f32 = neat.all_nodes[5].y.clone().into();
        assert_eq!(x, 0.9);
        assert_eq!(y, 0.75);
    }
}