use std::collections::HashMap;
use crate::genome::*;

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
    pub fn get_node(&mut self, index: usize) -> NodeGene {
        let len = self.all_nodes.len();
        if index <= len {
            self.all_nodes[index].clone()
        }
        else {
            self.create_node(0.0, 0.0);
            self.all_nodes[len - 1].clone()
        }
    }

    pub fn get_connection(&mut self, node1: NodeGene, node2: NodeGene) -> ConnectionGene {
        let mut connection_gene = ConnectionGene::new(node1, node2);

        if self.all_connections.contains_key(&connection_gene) {
            connection_gene.innovation_number = self.all_connections.get(&connection_gene)
                                                    .expect("all_connections doesn't contain connection_gene")
                                                    .innovation_number;
        }
        else {
            connection_gene.innovation_number = self.all_connections.len() as u32 + 1;
            self.all_connections.insert(connection_gene.clone(), connection_gene.clone());
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
    fn outputs() {
        let neat = Neat::new(3, 3, 40);

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
            let node1 = NodeGene::new(rand::random());
            let node2 = NodeGene::new(rand::random());
    
            let connection = neat.get_connection(node1, node2);
    
            assert_eq!(connection.innovation_number, i + 1);
        }
    }
}