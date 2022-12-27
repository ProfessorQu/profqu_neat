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
    pub fn get_active_sum(&self) -> f32 {
        match self.num_activations > 0 {
            true => self.active_sum,
            false => 0.0
        }
    }

    pub fn is_sensor(&self) -> bool {
        self.node_type == NodeType::Sensor
    }

    pub fn is_neuron(&self) -> bool {
        self.node_type == NodeType::Neuron
    }

    pub fn add_active_sum(&mut self, sum: f32) {
        self.prev_active_sums.insert(0, sum);
        self.prev_active_sums.pop();
    }
}