use std::{collections::HashMap, rc::Rc, cell::RefCell, borrow::BorrowMut, ops::Deref};

use crate::genome::Genome;

use super::{Node, Connection, node};

/// The struct to calculate the output of a genome
#[derive(Clone, PartialEq, Debug)]
pub struct Calculator {
    input_nodes: Vec<RefCell<Node>>,
    hidden_nodes: Vec<RefCell<Node>>,
    output_nodes: Vec<RefCell<Node>>,
}

impl Calculator {
    /// Create a new calculator from a genome
    pub fn new(genome: Genome) -> Self {
        let mut calc = Self {
            input_nodes: Vec::new(),
            hidden_nodes: Vec::new(),
            output_nodes: Vec::new(),
        };

        let nodes = genome.nodes;
        let connections = genome.connections;

        let mut node_hash_map = HashMap::new();

        for node_gene in nodes.data {
            let node = RefCell::new(
                    Node::new(node_gene.x.parse())
            );

            node_hash_map.insert(node_gene.innovation_number, node.clone());

            if node_gene.x.parse() <= 0.1 {
                calc.input_nodes.push(node);
            }
            else if node_gene.x.parse() >= 0.9 {
                calc.output_nodes.push(node);
            }
            else {
                calc.hidden_nodes.push(node);
            }
        }

        calc.hidden_nodes.sort();

        for connection_gene in connections.data {
            let from = connection_gene.from;
            let to = connection_gene.to;

            let node_from = node_hash_map.get(&from.innovation_number)
                    .expect("'from' is not in the hashmap").clone();
            let node_to = node_hash_map.get(&to.innovation_number)
                    .expect("'to' is not in the hashmap").clone();

            let mut connection = Connection::new(node_from, RefCell::clone(&node_to));
            connection.weight = connection_gene.weight;
            connection.enabled = connection_gene.enabled;

            node_to.borrow_mut().connections.push(connection);
        }

        calc
    }

    /// Calculate the outputs
    pub fn calculate(&mut self, inputs: Vec<f32>) -> Result<Vec<f32>, &'static str> {
        for i in 0..self.input_nodes.len() {
            self.input_nodes[i].borrow_mut().output = inputs[i].into();
        }

        for hidden_node in self.hidden_nodes.clone() {
            hidden_node.borrow_mut().calculate();
        }

        let mut output = vec![0.0; self.output_nodes.len()];

        for i in 0..self.output_nodes.len() {
            self.output_nodes[i].borrow_mut().calculate();
            output[i] = self.output_nodes[i].borrow_mut().output.into();
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

        let calc = Calculator::new(genome.clone());
        
        assert_eq!(calc.input_nodes.len(), 3);
        assert_eq!(calc.output_nodes.len(), 3);
        assert_eq!(calc.hidden_nodes.len(), 0);

        genome.add_connection(&mut neat, 0, 4);

        let calc = Calculator::new(genome.clone());

        assert_eq!(calc.input_nodes.len(), 3);
        assert_eq!(calc.output_nodes.len(), 3);
        
        let node = neat.create_node(0.5, 0.5);
        genome.nodes.add(node);

        let calc = Calculator::new(genome.clone());

        assert_eq!(calc.input_nodes.len(), 3);
        assert_eq!(calc.output_nodes.len(), 3);
        assert_eq!(calc.hidden_nodes.len(), 1);
        assert_eq!(calc.hidden_nodes.get(0).unwrap().borrow().x.parse(), 0.5);
        
        let node = neat.create_node(0.3, 0.5);
        genome.nodes.add(node);

        let calc = Calculator::new(genome.clone());

        assert_eq!(calc.hidden_nodes.len(), 2);
        assert_eq!(calc.hidden_nodes.get(0).unwrap().borrow().x.parse(), 0.3);
    }

    #[test]
    fn calculate() {
        let mut neat = Neat::new(3, 3, 10);

        let mut genome = neat.empty_genome();
        
        genome.generate_calculator();
        let result = genome.calculate(vec![0.0, 0.0, 0.0]).unwrap();
        assert_eq!(result, vec![0.5, 0.5, 0.5]);

        for _ in 0..100 {
            genome.mutate(&mut neat);
        }

        genome.generate_calculator();
        assert_ne!(genome.calculate(vec![6.9, 4.2, 40.9]).unwrap(), result);
    }
}