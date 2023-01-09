use std::{fmt::Debug, rc::Rc, cell::RefCell};

use crate::{genome::{Genome, Gene}, calculations::Calculator, data_structures::PseudoFloat, Neat};

/// The client that controls the genome and the fitness
#[derive(Clone)]
pub struct Client {
    pub genome: Genome,
    calculator: Option<Calculator>,
    pub fitness: PseudoFloat,
    pub has_species: bool
}

impl Client {
    pub fn new(genome: Genome) -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(
                Self {
                    genome,
                    calculator: None,
                    fitness: PseudoFloat::new(0.0),
                    has_species: false
                }
            )
        )
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

    /// Calculate the distance from this client's genome to other's genome
    pub fn distance(&self, other: &Client) -> f32 {
        Genome::distance(&self.genome, &other.genome)
    }

    /// Mutate this client's genome
    pub fn mutate(&mut self, neat: &mut Neat) {
        self.genome.mutate(neat);
    }
}

impl Gene for Client {
    fn get_innovation_number(&self) -> u32 { 0 }
    fn set_innovation_number(&mut self, _new: u32) { }
}

impl Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Client {{ fitness: {:?} }}", self.fitness)
    }
}