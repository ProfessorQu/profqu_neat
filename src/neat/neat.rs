use std::collections::HashMap;
use crate::genome::*;

#[cfg(test)]
#[path ="neat_test.rs"]
mod neat_test;

pub const MAX_NODES: u32 = 2^20;

pub const DISJOINT_MULT: f32 = 1.0;
pub const EXCESS_MULT: f32 = 1.0;
pub const WEIGHT_DIFF_MULT: f32 = 1.0;

pub const WEIGHT_SHIFT_STRENGTH: f32 = 0.3;
pub const WEIGHT_RANDOM_STRENGTH: f32 = 1.0;

/// The struct that controls the entire library
pub struct Neat {
    all_connections: HashMap<u32, ConnectionGene>,
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

    /// Create an empty genome with no hidden nodes or connections
    pub fn empty_genome(&mut self) -> Genome {
        let mut genome = Genome::new();

        for index in 0..self.input_size as usize + self.output_size as usize {
            genome.nodes.add(self.get_node(index));
        }

        genome
    }

    /// Reset this neat struct with new values
    pub fn reset(&mut self, input_size: u32, output_size: u32, population_size: u32) {
        self.input_size = input_size;
        self.output_size = output_size;
        self.population_size = population_size;

        self.all_connections.clear();
        self.all_nodes.clear();

        for input_index in 0..input_size as usize {
            let y = (input_index + 1) as f32 / (input_size + 1) as f32;
            self.create_node(0.1, y);
        }

        for output_index in 0..output_size as usize {
            let y = (output_index + 1) as f32 / (output_size + 1) as f32;
            self.create_node(0.9, y);
        }
    }

    /// Create a new node with a certain x and y coordinate
    pub fn create_node(&mut self, x: f32, y: f32) -> NodeGene {
        let mut node = NodeGene::new(self.all_nodes.len() as u32 + 1);

        node.x = x.into();
        node.y = y.into();

        self.all_nodes.push(node);
        let len = self.all_nodes.len();
        
        self.all_nodes[len - 1]
    }

    /// Get a new node
    pub fn get_node(&mut self, index: usize) -> NodeGene {
        let len = self.all_nodes.len();
        if index <= len {
            self.all_nodes[index]
        }
        else {
            self.create_node(0.0, 0.0)
        }
    }

    pub fn get_connection(&mut self, node1: NodeGene, node2: NodeGene) -> ConnectionGene {
        let mut connection_gene = ConnectionGene::new(node1, node2);

        if self.all_connections.contains_key(&connection_gene.hash_code()) {
            connection_gene.innovation_number = self.all_connections.get(&connection_gene.hash_code())
                                                    .expect("all_connections doesn't contain connection_gene")
                                                    .innovation_number;
        }
        else {
            connection_gene.innovation_number = self.all_connections.len() as u32 + 1;
            self.all_connections.insert(connection_gene.hash_code(), connection_gene);
        }

        connection_gene
    }
}