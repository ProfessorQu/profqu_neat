use std::collections::HashMap;

use crate::genome::Genome;

use super::{Node, Connection};

/// The struct to calculate the output of a genome
#[derive(Clone, PartialEq, Debug)]
pub struct Calculator {
    input_nodes: Vec<Node>,
    output_nodes: Vec<Node>,
    hidden_nodes: Vec<Node>,
}

impl Calculator {
    /// Create a new calculator from a genome
    pub fn new(genome: Genome) -> Self {
        let mut input_nodes = Vec::new();
        let mut output_nodes = Vec::new();
        let mut hidden_nodes = Vec::new();

        let nodes = genome.nodes;
        let connections = genome.connections;

        let mut node_hash_map: HashMap<u32, Node> = HashMap::new();

        for node_gene in nodes.data {
            let node = Node::new(node_gene.x.parse());
            node_hash_map.insert(node_gene.innovation_number, node.clone());

            if node_gene.x.parse() <= 0.1 {
                input_nodes.push(node);
            }
            else if node_gene.x.parse() >= 0.9 {
                output_nodes.push(node);
            }
            else {
                hidden_nodes.push(node);
            }
        }

        hidden_nodes.sort();

        for connection in connections.data {
            let from = connection.from;
            let to = connection.to;

            let node_from = node_hash_map.get(&from.innovation_number)
                                    .expect("Node in connection but not in hashmap").clone();
            let mut node_to = node_hash_map.get_mut(&to.innovation_number)
                                    .expect("Node in connection but not in hashmap");

            let mut new_connection = Connection::new(node_from.clone(), node_to.clone());
            new_connection.weight = connection.weight;
            new_connection.enabled = connection.enabled;

            node_to.connections.push(new_connection);
        }

        Self { input_nodes, output_nodes, hidden_nodes }
    }

    /// Calculate the outputs
    pub fn calculate(&mut self, inputs: Vec<f32>) -> Result<Vec<f32>, &'static str> {
        if inputs.len() != self.input_nodes.len() {
            return Err("inputs is not the same length as the input nodes")
        }

        for i in 0..self.input_nodes.len() {
            self.input_nodes[i].output = inputs[i].into();
        }

        for hidden_node in self.hidden_nodes.iter_mut() {
            hidden_node.calculate();
        }

        let mut output = Vec::new();

        for i in 0..self.output_nodes.len() {
            self.output_nodes[i].calculate();
            output[i] = self.output_nodes[i].output.into();
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Neat, genome::NodeGene};

    use super::*;

    #[test]
    fn new() {
        let mut neat = Neat::new(3, 3, 10);

        let mut genome = neat.empty_genome();

        genome.add_connection(&mut neat, 0, 4);

        let calc = Calculator::new(genome.clone());

        assert_eq!(calc.input_nodes.len(), 3);
        assert_eq!(calc.output_nodes.len(), 3);
        assert_eq!(calc.hidden_nodes.len(), 0);
        
        let node = neat.create_node(0.5, 0.5);
        genome.nodes.add(node);

        let calc = Calculator::new(genome.clone());

        assert_eq!(calc.hidden_nodes.len(), 1);
        assert_eq!(calc.hidden_nodes.get(0).unwrap().x.parse(), 0.5);
        
        let node = neat.create_node(0.3, 0.5);
        genome.nodes.add(node);

        let calc = Calculator::new(genome.clone());

        assert_eq!(calc.hidden_nodes.len(), 2);
        assert_eq!(calc.hidden_nodes.get(0).unwrap().x.parse(), 0.3);
    }
}