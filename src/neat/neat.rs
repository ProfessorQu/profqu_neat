use std::collections::HashMap;
use crate::genome::*;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let neat = Neat::new(3, 3, 15);
        assert_eq!(neat.all_nodes.len(), 6);

        assert_eq!(neat.input_size, 3);
        assert_eq!(neat.output_size, 3);
        assert_eq!(neat.population_size, 15);
    }

    #[test]
    fn inputs() {
        let neat = Neat::new(3, 3, 200);

        let x: f32 = neat.all_nodes[0].x.into();
        let y: f32 = neat.all_nodes[0].y.into();
        assert_eq!(x, 0.1);
        assert_eq!(y, 0.25);
        
        let x: f32 = neat.all_nodes[1].x.into();
        let y: f32 = neat.all_nodes[1].y.into();
        assert_eq!(x, 0.1);
        assert_eq!(y, 0.5);
        
        let x: f32 = neat.all_nodes[2].x.into();
        let y: f32 = neat.all_nodes[2].y.into();
        assert_eq!(x, 0.1);
        assert_eq!(y, 0.75);
    }

    #[test]
    fn outputs() {
        let neat = Neat::new(3, 3, 40);

        let x: f32 = neat.all_nodes[3].x.into();
        let y: f32 = neat.all_nodes[3].y.into();
        assert_eq!(x, 0.9);
        assert_eq!(y, 0.25);
        
        let x: f32 = neat.all_nodes[4].x.into();
        let y: f32 = neat.all_nodes[4].y.into();
        assert_eq!(x, 0.9);
        assert_eq!(y, 0.5);
        
        let x: f32 = neat.all_nodes[5].x.into();
        let y: f32 = neat.all_nodes[5].y.into();
        assert_eq!(x, 0.9);
        assert_eq!(y, 0.75);
    }

    #[test]
    fn empty_genome() {
        let mut neat = Neat::new(3, 3, 100);

        let genome = neat.empty_genome();

        for node in neat.all_nodes {
            assert!(genome.nodes.contains(&node));
        }
    }

    #[test]
    fn get_connection() {
        let mut neat = Neat::new(3, 3, 100);

        for i in 0..10 {
            let node1 = NodeGene::new(i * 2);
            let node2 = NodeGene::new(1 + i * 2);
    
            let connection = neat.get_connection(node1, node2);
            let connection2 = neat.get_connection(node1, node2);

            let connection3 = neat.get_connection(node2, node1);

            // Test innovation numbers of same connections
            assert_eq!(connection.innovation_number, i * 2 + 1);
            assert_eq!(connection.innovation_number, connection2.innovation_number);

            // Test innovation numbers of different connections
            assert_eq!(connection3.innovation_number, i * 2 + 2);
            assert_ne!(connection.innovation_number, connection3.innovation_number);

            // Test equality of same and different connections
            assert_eq!(connection, connection2);
            assert_ne!(connection, connection3);

            // Test equality of hash codes of same and different connections
            assert_eq!(connection.hash_code(), connection2.hash_code());
            assert_ne!(connection2.hash_code(), connection3.hash_code());
        }
    }
}