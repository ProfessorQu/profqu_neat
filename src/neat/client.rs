use crate::{genome::{Genome, Gene}, calculations::Calculator, data_structures::{RandomHashSet, PseudoFloat}};

use super::Species;

/// The client that controls the genome and the fitness
#[derive(PartialEq, Debug, Clone)]
pub struct Client {
    genome: Genome,
    calculator: Option<Calculator>,
    pub fitness: PseudoFloat
}

impl Client {
    pub fn new(genome: Genome) -> Self {
        Self {
            genome,
            calculator: None,
            fitness: PseudoFloat::new(0.0)
        }
    }

    /// Generate a calculator for this genome
    pub fn generate_calculator(&mut self) {
        self.calculator = Some(Calculator::new(self.genome.clone()));
    }

    /// Calculate the outputs
    pub fn calculate(&mut self, inputs: Vec<f32>) -> Result<Vec<f32>, &'static str> {
        if self.calculator.is_none() {
            self.generate_calculator()
        }

        self.calculator.as_mut().expect("Failed to generate calculator").calculate(inputs)
    }
}

impl Gene for Client {
    fn get_innovation_number(&self) -> u32 { 0 }
    fn set_innovation_number(&mut self, new: u32) { }
}