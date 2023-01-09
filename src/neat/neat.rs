use std::{collections::HashMap, rc::Rc, cell::RefCell};
use rand::seq::SliceRandom;

use crate::genome::*;

use super::{Client, Species};

#[cfg(test)]
#[path ="neat_test.rs"]
mod neat_test;

/// The maximum number of nodes in a network
pub const MAX_NODES: u32 = 2u32.pow(20);

/// The multiplier for the disjoint genes in the `distance` function
pub const MULT_DISJOINT: f32 = 3.0;
/// The multiplier for the excess genes in the `distance` function
pub const MULT_EXCESS: f32 = 2.0;
/// The multiplier for the weight difference in the `distance` function
pub const MULT_WEIGHT_DIFF: f32 = 4.0;

/// The weight shifting strength when mutating
pub const WEIGHT_SHIFT_STRENGTH: f32 = 0.3;
/// The weight randomness strength when mutating
pub const WEIGHT_RANDOM_STRENGTH: f32 = 1.0;

/// The probability of mutating a new link
pub const PROB_MUTATE_LINK: f32 = 0.01;
/// The probability of mutating a new node
pub const PROB_MUTATE_NODE: f32 = 0.003;
/// The probability of mutating and shifting a weight
pub const PROB_MUTATE_WEIGHT_SHIFT: f32 = 0.002;
/// The probability of mutating and selecting a new random value for a weight
pub const PROB_MUTATE_WEIGHT_RANDOM: f32 = 0.002;
/// The probability of mutating and toggling a link
pub const PROB_MUTATE_TOGGLE_LINK: f32 = 0.0;

/// The threshold for creating a new species
pub const SPECIES_THRESHOLD: f32 = 4.0;

/// Determine the percentage of clients that will be killed
pub const KILL_PERCENTAGE: f32 = 0.2;

/// The struct that controls the entire library
pub struct Neat {
    all_connections: HashMap<u32, ConnectionGene>,
    all_nodes: Vec<NodeGene>,
    clients: Vec<Rc<RefCell<Client>>>,
    species: Vec<Species>,
    input_size: u32,
    output_size: u32,
    population_size: u32
}

impl Neat {
    /// Create a new neat struct
    /// ```rust
    /// use profqu_neat::Neat;
    /// 
    /// let neat = Neat::new(3, 3, 15);
    /// ```
    pub fn new(input_size: u32, output_size: u32, population_size: u32) -> Self {
        let mut neat = Self {
            all_connections: HashMap::new(),
            all_nodes: Vec::new(),
            clients: Vec::new(),
            species: Vec::new(),
            input_size,
            output_size,
            population_size
        };

        neat.reset(input_size, output_size, population_size);
        neat
    }

    /// Reset this neat struct with new values
    /// ```rust
    /// use profqu_neat::Neat;
    /// 
    /// let mut neat = Neat::new(3, 3, 15);
    /// let genome = neat.empty_genome();
    /// assert_eq!(genome.nodes.len(), 6);
    /// 
    /// neat.reset(3, 5, 4);
    /// 
    /// let genome = neat.empty_genome();
    /// assert_eq!(genome.nodes.len(), 8);
    /// ```
    pub fn reset(&mut self, input_size: u32, output_size: u32, population_size: u32) {
        self.input_size = input_size;
        self.output_size = output_size;
        self.population_size = population_size;

        self.all_connections.clear();
        self.all_nodes.clear();
        self.clients.clear();

        for input_index in 0..input_size as usize {
            let y = (input_index + 1) as f32 / (input_size + 1) as f32;
            self.create_node(0.1, y);
        }

        for output_index in 0..output_size as usize {
            let y = (output_index + 1) as f32 / (output_size + 1) as f32;
            self.create_node(0.9, y);
        }

        for _client_index in 0..population_size as usize {
            let client = Client::new(self.empty_genome());
            client.borrow_mut().generate_calculator();
            self.clients.push(client);
        }
    }

    /// Get a client from this structure
    pub fn get_client(&self, index: usize) -> Rc<RefCell<Client>> {
        Rc::clone(self.clients.get(index).expect("Index out of bounds"))
    }

    /// Create an empty genome with no hidden nodes or connections
    /// ```rust
    /// use profqu_neat::Neat;
    /// 
    /// let mut neat = Neat::new(3, 3, 100);
    ///
    /// let genome = neat.empty_genome();
    ///
    /// assert_eq!(genome.connections.len(), 0);
    /// assert_eq!(genome.nodes.len(), 6);
    /// ```
    pub fn empty_genome(&mut self) -> Genome {
        let mut genome = Genome::new();

        for index in 0..self.input_size as usize + self.output_size as usize {
            genome.nodes.add(self.get_node(index));
        }

        genome
    }

    /// Create a new node with a certain x and y coordinate
    pub fn create_node(&mut self, x: f32, y: f32) -> NodeGene {
        let mut node = NodeGene::new(self.all_nodes.len() as u32 + 1);

        node.x = x.into();
        node.y = y.into();

        self.all_nodes.push(node);
        let len = self.all_nodes.len();
        
        self.all_nodes[len - 1]
    }

    /// Get a new node if it's out of bounds
    pub fn get_node(&mut self, index: usize) -> NodeGene {
        let len = self.all_nodes.len();
        if index <= len {
            self.all_nodes[index]
        }
        else {
            self.create_node(0.0, 0.0)
        }
    }

    /// Create a new connection from node1 to node2
    pub fn get_connection(&mut self, node1: NodeGene, node2: NodeGene) -> ConnectionGene {
        let mut connection_gene = ConnectionGene::new(node1, node2);

        if self.all_connections.contains_key(&connection_gene.hash_code()) {
            connection_gene.innovation_number = self.all_connections.get(&connection_gene.hash_code())
                                                    .expect("all_connections doesn't contain connection_gene")
                                                    .innovation_number;
        }
        else {
            connection_gene.innovation_number = self.all_connections.len() as u32 + 1;
            self.all_connections.insert(connection_gene.hash_code(), connection_gene);
        }

        connection_gene
    }

    /// A wrapper function for all the evolution steps
    pub fn evolve(&mut self) {
        self.gen_species();
        self.kill();
        self.remove_extinct_species();
        self.reproduce();
        self.mutate();

        for client in &self.clients {
            client.borrow_mut().generate_calculator();
        }
    }

    /// Generate new species
    fn gen_species(&mut self) {
        for client in &self.clients {
            client.borrow_mut().has_species = false;
        }

        for species in &mut self.species {
            species.reset();
        }

        for client in &self.clients {
            if client.borrow().has_species { continue }

            let mut found = false;
            for species in &mut self.species {
                if species.put(Rc::clone(client)) {
                    found = true;
                    break;
                }
            }

            if !found {
                client.borrow_mut().has_species = true;
                self.species.push(Species::new(client.clone()));
            }
        }

        for species in &mut self.species {
            species.evaluate_fitness();
        }
    }

    /// Kill a certain percentage of species
    fn kill(&mut self) {
        for species in &mut self.species {
            species.kill(KILL_PERCENTAGE);
        }
    }

    /// Remove all the extinct species
    fn remove_extinct_species(&mut self) {
        // TODO: FIX ITER BOUNDS
        for i in (0..self.species.len()).rev() {
            if self.species[i].len() <= 1 {
                self.species[i].go_extinct();
                self.species.remove(i);
            }
        }
    }

    /// Reproduce the clients
    fn reproduce(&mut self) {
        let clients = self.clients.clone();
        let mut all_species = self.species.clone();
        for client in clients {
            if !client.borrow().has_species {
                let species = all_species
                    .choose_weighted_mut(&mut rand::thread_rng(), |s| s.average_fitness.parse())
                    .expect("species is empty");

                client.borrow_mut().genome = species.breed(self);
                species.force_put(Rc::clone(&client));
            }
        }

        self.species = all_species;
    }

    /// Mutate all the clients
    fn mutate(&mut self) {
        let mut clients = self.clients.clone();
        for client in &mut clients {
            client.borrow_mut().mutate(self);
        }

        self.clients = clients;
    }

    /// Returns the best client out of all of them
    pub fn best_client(&mut self) -> Option<Client> {
        let mut best_client = None;

        let mut best_fitness = f32::MIN;
        for client in &self.clients {
            let fitness = client.borrow().fitness.parse();
            if fitness > best_fitness {
                best_client = Some(client.borrow().clone());
                best_fitness = fitness;
            }
        }

        best_client
    }

    /// Print all the different species
    pub fn print_species(&self) {
        println!("#######################################################");
        for species in &self.species {
            println!("{species:?}");
        }
        println!("#######################################################");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evolve() {
        let mut neat = Neat::new(10, 1, 1000);

        let input: Vec<f32> = vec![rand::random(); 10];

        let fitness_before = neat.clients[0].borrow_mut()
            .calculate(input.clone()).expect("Failed to calculate")[0];

        for _ in 0..100 {
            for client in &neat.clients {
                let fitness = client.borrow_mut().calculate(input.clone()).expect("Failed to calculate")[0];
                client.borrow_mut().fitness = fitness.into();
            }

            neat.evolve();
        }

        let best = neat.best_client().expect("Failed to get client");
        neat.print_species();

        assert!(best.fitness.parse() > fitness_before);
    }
}