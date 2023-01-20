use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{calculations::Calculator, genome::Genome, Neat};

/// The client that controls the genome and the fitness
#[derive(Clone)]
pub struct Client {
    /// The genome of this client
    pub genome: Genome,
    calculator: Option<Calculator>,
    /// The fitness of this client
    pub fitness: f32,
    /// A boolean to determine whether this client is a part of a species or not
    pub has_species: bool,
}

impl Client {
    /// Create a new client
    ///
    /// Not meant to be called directly
    pub fn new(genome: Genome) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            genome,
            calculator: None,
            fitness: 0.0,
            has_species: false,
        }))
    }

    #[doc(hidden)]
    /// Generate a calculator for this genome
    pub fn generate_calculator(&mut self) {
        self.calculator = Some(Calculator::new(self.genome.clone()));
    }

    /// Calculate the outputs
    ///
    /// # Examples
    ///
    /// ```rust
    /// use profqu_neat::Neat;
    ///
    /// Neat::test_config();
    /// let mut neat = Neat::new(3, 3, 5);
    ///
    /// for mut client in neat.iter_clients() {
    ///     let result = client.calculate(&vec![5.0, 1.0, 2.0]);
    ///     client.fitness = result[0] + result[1] * result[2];
    /// }
    /// ```
    pub fn calculate(&mut self, inputs: &Vec<f32>) -> Vec<f32> {
        if self.calculator.is_none() {
            self.generate_calculator();
        }

        self.calculator
            .as_mut()
            .expect("Failed to generate calculator")
            .calculate(inputs)
            .expect("Failed to calculate")
    }

    #[doc(hidden)]
    /// Calculate the distance from this client's genome to other's genome
    pub fn distance(&self, other: &Client) -> f32 {
        Genome::distance(&self.genome, &other.genome)
    }

    /// Mutate this client's genome
    pub fn mutate(&mut self, neat: &mut Neat) {
        self.genome.mutate(neat);
    }
}

impl Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Client {{ fitness: {:?}, has_species: {:?} }}",
            self.fitness, self.has_species
        )
    }
}
