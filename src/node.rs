use crate::link::Link;

#[derive(PartialEq, Clone)]
pub enum NodeType {
    Hidden, Input, Output, Bias
}

/// The nodes in the network
pub struct Node {
    node_id: u32,
    node_type: NodeType,
    pub activated: bool,
    pub active_sum: f32,
    pub prev_active_sums: Vec<f32>,
    pub num_activations: u32,
    pub incoming: Vec<Link>,
    pub outgoing: Vec<Link>,
}

impl Node {
    /// Create a new node
    pub fn new(node_id: u32, node_type: NodeType) -> Self {
        Self {
            node_id,
            node_type,
            activated: false,
            active_sum: 0.0,
            prev_active_sums: Vec::new(),
            num_activations: 0,
            incoming: Vec::new(),
            outgoing: Vec::new(),
        }
    }

    /// Get the total active sum of the node
    pub fn get_active_sum(&self) -> f32 {
        match self.num_activations > 0 {
            true => self.active_sum,
            false => 0.0
        }
    }

    /// Add an incoming connection
    pub fn add_incoming(&mut self, feed_node: &Node, weight: f32) {
        // let link = Link {
        //     in_node: feed_node,
        //     out_node: self,
        //     weight
        // };

        // self.incoming.push(link);
    }

    /// Check if the node's type is a sensor
    pub fn is_sensor(&self) -> bool {
        self.node_type == NodeType::Input || self.node_type == NodeType::Output
    }

    /// Check if the node's type is a neuron
    pub fn is_neuron(&self) -> bool {
        !self.is_sensor()
    }

    /// Archive an active sum to the history
    pub fn archive_active_sum(&mut self, sum: f32) {
        self.prev_active_sums.insert(0, sum);
        if self.prev_active_sums.len() > 2 {
            self.prev_active_sums.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_node_type() {
        let mut node = Node {
            node_id: 0,
            node_type: NodeType::Hidden,
            activated: false,
            active_sum: 0.0,
            prev_active_sums: Vec::new(),
            num_activations: 0,
            incoming: Vec::new(),
            outgoing: Vec::new(),
        };
        
        assert!(node.is_neuron());
        assert!(!node.is_sensor());
        node.node_type = NodeType::Input;
        assert!(!node.is_neuron());
        assert!(node.is_sensor());
    }
    
    #[test]
    fn test_node_history() {
        let mut node = Node {
            node_id: 0,
            node_type: NodeType::Hidden,
            activated: false,
            active_sum: 0.0,
            prev_active_sums: Vec::new(),
            num_activations: 0,
            incoming: Vec::new(),
            outgoing: Vec::new(),
        };

        assert_eq!(node.prev_active_sums.len(), 0);
        node.archive_active_sum(1.0);
        assert_eq!(node.prev_active_sums.len(), 1);
        node.archive_active_sum(17.0);
        assert_eq!(node.prev_active_sums.len(), 2);
        node.archive_active_sum(7.0);
        assert_eq!(node.prev_active_sums.len(), 2);
    }
}