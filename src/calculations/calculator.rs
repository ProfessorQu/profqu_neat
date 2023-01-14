use std::{collections::HashMap, cell::RefCell, rc::Rc};

use crate::genome::Genome;

use super::{Node, Connection};

/// The struct to calculate the output of a genome
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Calculator {
    input_nodes: Vec<Rc<RefCell<Node>>>,
    hidden_nodes: Vec<Rc<RefCell<Node>>>,
    output_nodes: Vec<Rc<RefCell<Node>>>,
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
            let node = Node::new(node_gene.x.parse());
            let pointer = Rc::new(RefCell::new(node));

            node_hash_map.insert(node_gene.innovation_number, Rc::clone(&pointer));

            if node_gene.x.parse() <= 0.1 {
                calc.input_nodes.push(pointer);
            }
            else if node_gene.x.parse() >= 0.9 {
                calc.output_nodes.push(pointer);
            }
            else {
                calc.hidden_nodes.push(pointer);
            }
        }

        calc.hidden_nodes.sort();

        for connection_gene in connections.data {
            let from = connection_gene.from;
            let to = connection_gene.to;

            let node_from = node_hash_map.get(&from.innovation_number)
                    .expect("'from' is not in the hashmap");
            let node_to = node_hash_map.get(&to.innovation_number)
                    .expect("'to' is not in the hashmap");

            let mut connection = Connection::new(Rc::clone(node_from));
            connection.weight = connection_gene.weight.into();
            connection.enabled = connection_gene.enabled;
            let pointer = Rc::new(RefCell::new(connection));
            
            node_to.borrow_mut().connections.push(pointer);
        }

        calc
    }

    /// Calculate the outputs
    pub fn calculate(&mut self, inputs: Vec<f32>) -> Result<Vec<f32>, &'static str> {
        if inputs.len() + 1 != self.input_nodes.len() {
            return Err("Number of inputs aren't equal to number of input nodes")
        }

        for (i, input) in inputs.iter().enumerate() {
            self.input_nodes[i].borrow_mut().output = (*input);
        }
        
        self.input_nodes.last().expect("No input_nodes").borrow_mut().output = 1.0;

        for hidden_node in self.hidden_nodes.clone() {
            hidden_node.borrow_mut().calculate();
        }

        let mut outputs = vec![0.0; self.output_nodes.len()];

        for (i, output) in self.output_nodes.iter().enumerate() {
            output.borrow_mut().calculate();
            outputs[i] = output.borrow().output;
        }

        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use crate::Neat;

    use super::*;

    #[test]
    fn new() {
        Neat::test_config();
        let mut neat = Neat::new(3, 3, 10);

        let mut genome = neat.empty_genome();

        let calc = Calculator::new(genome.clone());
        
        assert_eq!(calc.input_nodes.len(), 4);
        assert_eq!(calc.output_nodes.len(), 3);
        assert_eq!(calc.hidden_nodes.len(), 0);

        genome.add_connection(&mut neat, 0, 4);

        let calc = Calculator::new(genome.clone());

        assert_eq!(calc.input_nodes.len(), 4);
        assert_eq!(calc.output_nodes.len(), 3);
        
        let node = neat.create_node(0.5, 0.5);
        genome.nodes.add(node);

        let calc = Calculator::new(genome.clone());

        assert_eq!(calc.input_nodes.len(), 4);
        assert_eq!(calc.output_nodes.len(), 3);
        assert_eq!(calc.hidden_nodes.len(), 1);
        assert_eq!(calc.hidden_nodes.get(0).unwrap().borrow().x, 0.5);
        
        let node = neat.create_node(0.3, 0.5);
        genome.nodes.add(node);

        let calc = Calculator::new(genome.clone());

        assert_eq!(calc.hidden_nodes.len(), 2);
        assert_eq!(calc.hidden_nodes.get(0).unwrap().borrow().x, 0.3);
    }

    #[test]
    fn calculate() {
        Neat::test_config();
        let mut neat = Neat::new(3, 3, 10);

        let mut genome = neat.empty_genome();
        
        let mut calc = Calculator::new(genome.clone());
        let result = calc.calculate(vec![0.0, 0.0, 0.0]).unwrap();
        assert_eq!(result, vec![0.5, 0.5, 0.5]);

        genome.add_connection(&mut neat, 0, 4);

        let mut calc = Calculator::new(genome.clone());
        assert_eq!(calc.calculate(vec![1.0, 0.0, 0.0]).unwrap(), vec![0.7310586, 0.5, 0.5]);
        
        genome.add_connection(&mut neat, 1, 4);
        
        let mut calc = Calculator::new(genome.clone());
        assert_eq!(calc.calculate(vec![1.0, 2.0, 0.0]).unwrap(), vec![0.95257413, 0.5, 0.5]);
    }

    #[test]
    fn calculate2() {
        Neat::test_config();
        let mut neat = Neat::new(3, 3, 10);

        let mut genome = neat.empty_genome();
        
        let mut calc = Calculator::new(genome.clone());
        let result = calc.calculate(vec![0.0, 0.0, 0.0]).unwrap();
        assert_eq!(result, vec![0.5, 0.5, 0.5]);
        assert_eq!(calc.input_nodes.last().unwrap().borrow().output, 1.0);

        genome.add_connection(&mut neat, 0, 4);

        let mut calc = Calculator::new(genome.clone());
        assert_eq!(calc.calculate(vec![1.0, 0.0, 0.0]).unwrap(), vec![0.7310586, 0.5, 0.5]);
        assert_eq!(calc.input_nodes.last().unwrap().borrow().output, 1.0);
        
        genome.add_connection(&mut neat, 1, 4);
        
        let mut calc = Calculator::new(genome.clone());
        assert_eq!(calc.calculate(vec![1.0, 2.0, 0.0]).unwrap(), vec![0.95257413, 0.5, 0.5]);
        assert_eq!(calc.input_nodes.last().unwrap().borrow().output, 1.0);
    }
}