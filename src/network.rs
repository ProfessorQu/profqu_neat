use crate::node::{Node, NodeType};

/// The neural network
struct Network {
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
            if output.num_activations == 0 { return true }
        }

        false
    }

    /// Activate the neural network
    fn activate(&mut self) -> bool {
        let mut first_time = true;
        let mut abort_count = 0;

        while self.outputsoff() || first_time {
            abort_count += 1;
            if abort_count >= 20 {
                return false;
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
                        node.add_active_sum(node.active_sum);
                    }

                    // TODO: Add the sigmoid activation function
                    todo!();

                    node.num_activations += 1;
                }
            }

            first_time = false;
        }

        true
    }
}