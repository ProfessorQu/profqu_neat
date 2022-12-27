use crate::link::Link;

#[derive(PartialEq)]
pub enum NodeType {
    Sensor, Neuron,
}

enum NodePlace {
    Hidden, Input, Output, Bias
}


pub struct Node {
    node_id: u32,
    node_type: NodeType,
    pub activated: bool,
    pub active_sum: f32,
    pub prev_active_sums: Vec<f32>,
    pub num_activations: u32,
    pub incoming: Vec<Link>,
    pub outcoming: Vec<Link>,
}

impl Node {
    /// Create a new neuron
    pub fn new() -> Self {
        Self {
            node_id: 0,
            node_type: NodeType::Neuron,
            activated: false,
            active_sum: 0.0,
            prev_active_sums: Vec::new(),
            num_activations: 0,
            incoming: Vec::new(),
            outcoming: Vec::new(),
        }
    }

    /// Get the total active sum of the node
    pub fn get_active_sum(&self) -> f32 {
        match self.num_activations > 0 {
            true => self.active_sum,
            false => 0.0
        }
    }

    /// Check if the node's type is a sensor
    pub fn is_sensor(&self) -> bool {
        self.node_type == NodeType::Sensor
    }

    /// Check if the node's type is a neuron
    pub fn is_neuron(&self) -> bool {
        self.node_type == NodeType::Neuron
    }

    /// Set the node's type to `node_type`
    pub fn set_type(&mut self, node_type: NodeType) {
        self.node_type = node_type;
    }

    /// Archive an active sum to the history
    pub fn archive_active_sum(&mut self, sum: f32) {
        self.prev_active_sums.insert(0, sum);
        if self.prev_active_sums.len() > 2 {
            self.prev_active_sums.pop();
        }
    }
}