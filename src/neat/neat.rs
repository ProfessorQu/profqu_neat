use std::{collections::HashMap, rc::Rc, cell::{RefCell, RefMut}};
use rand::seq::SliceRandom;

use crate::genome::*;

use super::{Client, Species, Config, config::CONFIG};

#[cfg(test)]
#[path ="neat_test.rs"]
mod neat_test;

/// The maximum number of nodes in a network
pub const MAX_NODES: u64 = 2u64.pow(20);

#[derive(Clone)]
/// The struct that controls the entire library
pub struct Neat {
    all_connections: HashMap<u64, ConnectionGene>,
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
    /// Neat::test_config();
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
    /// Neat::test_config();
    /// let mut neat = Neat::new(3, 3, 15);
    /// 
    /// let genome = neat.empty_genome();
    /// assert_eq!(genome.nodes.len(), 7);
    /// 
    /// neat.reset(3, 5, 4);
    /// 
    /// let genome = neat.empty_genome();
    /// assert_eq!(genome.nodes.len(), 9);
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

        // Create a bias node
        self.create_node(0.1, 0.9);

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

    #[doc(hidden)]
    /// Load the config at "src/test_config.txt"
    pub fn test_config() -> bool {
        let config = Config::from_file("src/test_config.txt");
        CONFIG.set(config).is_ok()
    }

    /// Load a config from a vector and return if it succeeded
    pub fn load_config_from_vec(config: Vec<f32>) -> bool {
        let config = Config::from_vec(config, "relu");
        CONFIG.set(config).is_ok()
    }

    /// Load the config from a file and return if it succeeded
    pub fn load_config_from_file(filename: &str) -> bool {
        let config = Config::from_file(filename);
        CONFIG.set(config).is_ok()
    }

    #[doc(hidden)]
    /// Get a client with some index from this structure
    pub fn get_client(&self, index: usize) -> Rc<RefCell<Client>> {
        Rc::clone(self.clients.get(index).expect("Index out of bounds"))
    }

    #[doc(hidden)]
    /// Create an empty genome with no hidden nodes or connections
    pub fn empty_genome(&mut self) -> Genome {
        let mut genome = Genome::new();

        for index in 0..self.input_size as usize + 1 + self.output_size as usize {
            genome.nodes.add(self.get_node(index + 1).expect("Failed to get a node"));
        }

        genome
    }

    #[doc(hidden)]
    /// Create a new node with certain x and y coordinates
    pub fn create_node(&mut self, x: f32, y: f32) -> NodeGene {
        let mut node = NodeGene::new(self.all_nodes.len() as u32 + 1);

        node.x = x.into();
        node.y = y.into();

        self.all_nodes.push(node);
        let len = self.all_nodes.len();
        
        self.all_nodes[len - 1]
    }

    #[doc(hidden)]
    /// Create a new node if it's out of bounds, othersize return the node at 'index'
    pub fn get_node(&mut self, index: usize) -> Option<NodeGene> {
        let len = self.all_nodes.len();
        if index <= len {
            Some(self.all_nodes[index - 1])
        }
        else {
            None
        }
    }

    #[doc(hidden)]
    /// Create a new connection from node1 to node2 if it doesn't exist already
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

    #[doc(hidden)]
    /// Set a replace index from a connection
    pub fn set_replace_index(&mut self, from: NodeGene, to: NodeGene, replace_index: usize) {
        self.all_connections.get_mut(&ConnectionGene::new(from, to).hash_code())
            .expect("Failed to find connection gene").replace_index = replace_index;
    }

    #[doc(hidden)]
    /// Get a replace index from a connection
    pub fn get_replace_index(&self, from: NodeGene, to: NodeGene) -> usize {
        let connection = ConnectionGene::new(from, to);
        if let Some(connection) = self.all_connections.get(&connection.hash_code()) {
            connection.replace_index
        }
        else {
            0
        }
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

    #[doc(hidden)]
    /// Generate new species
    pub fn gen_species(&mut self) {
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
                self.species.push(Species::new(Rc::clone(client)));
            }
        }
    }

    #[doc(hidden)]
    /// Kill a certain percentage of species
    pub fn kill(&mut self) {
        for species in &mut self.species {
            species.evaluate_fitness();
            species.kill(Config::global().kill_percentage);
        }
    }

    #[doc(hidden)]
    /// Remove all the extinct species
    pub fn remove_extinct_species(&mut self) {
        for i in (0..self.species.len()).rev() {
            if self.species[i].len() <= 1 {
                self.species[i].go_extinct();
                self.species.remove(i);
            }
        }
    }

    #[doc(hidden)]
    /// Reproduce the clients
    pub fn reproduce(&mut self) {
        let mut all_species = self.species.clone();
        let mut thread = rand::thread_rng();
        for client in self.clients.clone() {
            if !client.borrow().has_species {
                let species = all_species
                    .choose_weighted_mut(&mut thread, |s| s.average_fitness)
                    .expect("Species is empty");

                client.borrow_mut().genome = species.breed(self);
                species.force_put(Rc::clone(&client));
            }
        }

        self.species = all_species;
    }

    #[doc(hidden)]
    /// Mutate all the clients
    pub fn mutate(&mut self) {
        for client in self.clients.clone() {
            client.borrow_mut().mutate(self);
        }
    }

    /// Iterate over all the clients in this struct to set their fitness
    pub fn iter_clients(&mut self) -> Vec<RefMut<Client>> {
        let mut clients = Vec::new();
        for client in &self.clients {
            clients.push(client.borrow_mut());
        }
        
        clients
    }

    /// Returns the best client out of all of the clients
    /// ```rust
    /// use profqu_neat::Neat;
    /// 
    /// Neat::test_config();
    /// let mut neat = Neat::new(10, 1, 1000);
    /// 
    /// let input: Vec<f32> = vec![rand::random(); 10];
    /// 
    /// for _iteration in 0..10 {
    ///     for mut client in neat.iter_clients() {
    ///         let fitness = client.calculate(input.clone())[0];
    ///         client.fitness = fitness.into();
    ///     }
    /// 
    ///     neat.evolve();
    /// }
    /// 
    /// neat.print_species();
    /// ```
    pub fn best_client(&mut self) -> Option<Client> {
        let mut best_client = None;
        let mut best_fitness = f32::MIN;

        for client in &self.clients {
            let fitness = client.borrow().fitness;
            if fitness > best_fitness {
                best_client = Some(client.borrow().clone());
                best_fitness = fitness;
            }
        }

        best_client
    }

    /// Print all the different species
    /// ```rust
    /// use profqu_neat::Neat;
    /// 
    /// Neat::test_config();
    /// let mut neat = Neat::new(10, 1, 1000);
    /// 
    /// let input: Vec<f32> = vec![rand::random(); 10];
    /// 
    /// for _iteration in 0..10 {
    ///     for mut client in neat.iter_clients() {
    ///         let fitness = client.calculate(input.clone())[0];
    ///         client.fitness = fitness.into();
    ///     }
    /// 
    ///     neat.evolve();
    /// }
    /// 
    /// neat.print_species();
    /// ```
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
    #[ignore = "takes a while"]
    fn evolve() {
        Neat::test_config();
        let mut neat = Neat::new(10, 1, 1000);

        let input: Vec<f32> = vec![rand::random(); 10];

        let fitness_before = neat.clients[0].borrow_mut()
            .calculate(input.clone())[0];

        for _iteration in 0..200 {
            for mut client in neat.iter_clients() {
                let fitness = client.calculate(input.clone())[0];
                client.fitness = fitness;
            }

            neat.evolve();
        }

        let best = neat.best_client().expect("Failed to get client");
        neat.print_species();
        println!("Best: {:?}", best);

        assert!(best.fitness > fitness_before);
    }
}