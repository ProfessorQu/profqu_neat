use std::error::Error;

use crate::node::Node;

/// The neural network
pub struct Network {
    net_id: u32,
    inputs: Vec<Node>,
    outputs: Vec<Node>,
    all_nodes: Vec<Node>,
    num_nodes: u32,
    num_links: u32,
}

impl Network {
    /// Check if the outputs are all off
    fn outputsoff(&self) -> bool {
        for output in &self.outputs {
            // There is some activation, so they aren't all off
            if output.num_activations != 0 { return false }
        }

        true
    }

    /// Activate the neural network
    pub fn activate(&mut self) -> Result<(), String> {
        let mut first_time = true;
        let mut abort_count = 0;

        while self.outputsoff() || first_time {
            abort_count += 1;
            if abort_count >= 20 {
                return Err("Failed to activate network after {abort_count}".to_string())
            }

            for node in &mut self.all_nodes {
                if node.is_neuron() {
                    node.active_sum = 0.0;
                    node.activated = false;

                    for link in &node.incoming {
                        let add_amount = link.weight * link.in_node.get_active_sum();
                        if link.in_node.activated || link.in_node.is_sensor() {
                            node.activated = true;
                        }

                        node.active_sum += add_amount;
                    }
                }
            }

            for node in &mut self.all_nodes {
                if node.is_neuron() {
                    if node.activated {
                        node.archive_active_sum(node.active_sum);
                    }

                    // TODO: Add the sigmoid activation function
                    todo!();

                    node.num_activations += 1;
                }
            }

            first_time = false;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::{borrow::BorrowMut, rc::Rc};

    use crate::Link;

    use super::*;

    #[test]
    fn test_activate() {
        use crate::node::NodeType;

        let mut net = Network {
            net_id: 0,
            inputs: Vec::new(),
            outputs: Vec::new(),
            all_nodes: Vec::new(),
            num_nodes: 0,
            num_links: 0,
        };

        let mut input = Node::new(
            0,
            NodeType::Input,
        );

        let mut output = Node::new(
            0,
            NodeType::Output,
        );

        let mut link = Link {
            in_node: Box::new(input),
            out_node: Box::new(output),
            weight: 1.0,
        };
    }
}