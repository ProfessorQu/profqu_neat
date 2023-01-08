use crate::{genome::Genome, calculations::Calculator, data_structures::RandomHashSet};

use super::Species;

/// The client that controls the genome and the fitness
// #[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Client {
    genome: Genome,
    calculator: Option<Calculator>,
    pub fitness: f32,
    species: Species
}

impl Client {
    pub fn new(genome: Genome) -> Self {
        Self {
            genome,
            calculator: None,
            fitness: 0.0,
            species: Species { }
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