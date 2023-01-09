use std::cmp::{Ordering, max};

use rand::{thread_rng, Rng};

use super::{connection_gene::ConnectionGene, node_gene::NodeGene};
use crate::data_structures::PseudoFloat;
use crate::data_structures::RandomHashSet;
use crate::neat::{Neat, Config};

#[cfg(test)]
#[path = "genome_test.rs"]
mod genome_test;

/// The genome with the connections and nodes
#[derive(Clone, PartialEq, Debug)]
pub struct Genome {
    pub connections: RandomHashSet<ConnectionGene>,
    pub nodes: RandomHashSet<NodeGene>
}

impl Genome {
    /// Create a new genome
    pub fn new() -> Self {
        Self {
            connections: RandomHashSet::new(),
            nodes: RandomHashSet::new()
        }
    }

    /// Get the highest innovation number of this genome
    pub fn highest_innov_num(&self) -> u32 {
        if self.connections.is_empty() {
            return 0
        }

        self.connections.get(self.connections.len() - 1).expect("Index out of bounds").innovation_number
    }

    /// Add a new connection to this genome
    pub fn add_connection(&mut self, neat: &mut Neat, index1: usize, index2: usize) {
        self.connections.add(
            neat.get_connection(
                *self.nodes.get(index1).expect("Failed to find node in genome with index1"),
                *self.nodes.get(index2).expect("Failed to find node in genome with index2")
            )
        );
    }
    
    /// Order two genomes according to their innovation number
    fn order_genomes(input_genome1: Genome, input_genome2: Genome) -> (Genome, Genome) {
        let innov1 = input_genome1.highest_innov_num();
        let innov2 = input_genome2.highest_innov_num();
        
        if innov1 < innov2 {
            (input_genome2, input_genome1)
        }
        else {
            (input_genome1, input_genome2)
        }
    }

    /// Get a connection by index
    fn get_connection(&self, index: usize) -> &ConnectionGene {
        self.connections.get(index).expect("Index out of range")
    }

    /// Calculate the distance between this and another genome
    /// ```rust
    /// use profqu_neat::{Neat, genome::Genome};
    /// 
    /// let mut neat = Neat::new(2, 2, 3);
    /// 
    /// let mut genome1 = neat.empty_genome();
    /// let mut genome2 = neat.empty_genome();
    /// 
    /// assert_eq!(Genome::distance(&genome1, &genome1), 0.0);
    /// assert_eq!(Genome::distance(&genome2, &genome2), 0.0);
    /// assert_eq!(Genome::distance(&genome1, &genome2), 0.0);
    /// 
    /// genome1.add_connection(&mut neat, 0, 2);
    /// 
    /// assert_eq!(Genome::distance(&genome1, &genome1), 0.0);
    /// assert_eq!(Genome::distance(&genome1, &genome2), 2.0);
    ///```
    pub fn distance(input_genome1: &Genome, input_genome2: &Genome) -> f32 {
        if input_genome1.connections.is_empty() && input_genome2.connections.is_empty() {
            return 0.0
        }

        // Set the highest genome to be genome1
        let (genome1, genome2) = Genome::order_genomes(input_genome1.clone(), input_genome2.clone());

        let mut index1 = 0;
        let mut index2 = 0;

        let mut num_disjoint = 0usize;
        let mut total_weight_diff = 0.0;
        let mut num_weight_similar = 0usize;

        // Go through all connections
        while index1 < genome1.connections.len() && index2 < genome2.connections.len() {
            let connection_gene1 = genome1.get_connection(index1);
            let connection_gene2 = genome2.get_connection(index2);

            let innov1 = connection_gene1.innovation_number;
            let innov2 = connection_gene2.innovation_number;

            match innov1.cmp(&innov2) {
                Ordering::Equal => {    // Same gene
                    num_weight_similar += 1;
                    total_weight_diff += (connection_gene1.weight.parse() - connection_gene2.weight.parse()).abs();
                    index1 += 1;
                    index2 += 1;
                },
                Ordering::Greater => {  // Disjoint gene of genome 1
                    num_disjoint += 1;
                    index2 += 1;
                },
                Ordering::Less => {     // Disjoint gene of genome 2
                    num_disjoint += 1;
                    index1 += 1;
                },
            }
        }

        let average_weight_diff = match num_weight_similar {
            0 => 0.0,
            _ => total_weight_diff / num_weight_similar as f32
        };

        let num_excess = genome1.connections.len() - index1;

        let mut total_genes = max(genome1.connections.len(), genome2.connections.len()) as f32;
        if total_genes < 20.0 {
            total_genes = 1.0;
        }

        Config::global().mult_disjoint * num_disjoint as f32 / total_genes +
        Config::global().mult_excess * num_excess as f32 / total_genes +
        Config::global().mult_weight_diff * average_weight_diff
    }

    /// Crossover two genomes, the first element should have the highest fitness
    /// ```rust
    /// use profqu_neat::{Neat, genome::Genome};
    /// let mut neat = Neat::new(3, 4, 10);
    /// 
    /// let mut genome1 = neat.empty_genome();
    /// let genome2 = neat.empty_genome();
    /// 
    /// let baby = Genome::crossover(&mut neat, &genome1, &genome2);
    /// 
    /// assert_eq!(Genome::distance(&genome1, &genome2), 0.0);
    /// assert_eq!(Genome::distance(&genome1, &baby), 0.0);
    ///
    /// genome1.add_connection(&mut neat, 0, 2);
    /// 
    /// let baby = Genome::crossover(&mut neat, &genome1, &genome2);
    /// 
    /// assert_eq!(Genome::distance(&genome1, &genome2), 2.0);
    /// assert_eq!(Genome::distance(&genome1, &baby), 0.0);
    /// ```
    pub fn crossover(neat: &mut Neat, genome1: &Genome, genome2: &Genome) -> Self {
        let mut baby = neat.empty_genome();

        let mut index1 = 0;
        let mut index2 = 0;

        // Go through all connections
        while index1 < genome1.connections.len() && index2 < genome2.connections.len() {
            let connection_gene1 = genome1.get_connection(index1);
            let connection_gene2 = genome2.get_connection(index2);

            let innov1 = connection_gene1.innovation_number;
            let innov2 = connection_gene2.innovation_number;

            // Add connections to the result genome accordingly
            match innov1.cmp(&innov2) {
                Ordering::Equal => {    // Same gene
                    if rand::random() {
                        baby.connections.add(*connection_gene1);
                    }
                    else {
                        baby.connections.add(*connection_gene2);
                    }

                    index1 += 1;
                    index2 += 1;
                },
                Ordering::Greater => {  // Disjoint gene of genome 2
                    // TODO: KEEP THIS IN MIND FOR BUGS BECAUSE IT'S ONLY USED FOR MAKING CROSSOVER2 CORRECT

                    index2 += 1;
                },
                Ordering::Less => {     // Disjoint gene of genome 1
                    baby.connections.add(*connection_gene1);

                    index1 += 1;
                },
            }
        }

        // Add all the excess nodes to the result genome
        while index1 < genome1.connections.len() {
            let connection1 = genome1.get_connection(index1);
            
            baby.connections.add(*connection1);

            index1 += 1;
        }

        for connection in baby.clone().connections.data {
            baby.nodes.add(connection.from);
            baby.nodes.add(connection.to);
        }

        baby
    }

    /// Mutate this genome with one of the following with a certain probabily
    ///  - Mutate a [new link](Self::mutate_link) with [`PROB_MUTATE_LINK`](crate::neat::PROB_MUTATE_LINK)
    ///  - Mutate a [new node](Self::mutate_node) with [`PROB_MUTATE_NODE`](crate::neat::PROB_MUTATE_NODE)
    ///  - Mutate a [weight shift](Self::mutate_weight_shift) with [`PROB_MUTATE_WEIGHT_SHIFT`](crate::neat::PROB_MUTATE_WEIGHT_SHIFT)
    ///  - Mutate a [new random weight](Self::mutate_weight_random) with [`PROB_MUTATE_WEIGHT_RANDOM`](crate::neat::PROB_MUTATE_WEIGHT_RANDOM)
    ///  - Mutate a [toggle in a link](Self::mutate_link_toggle) with [`PROB_MUTATE_TOGGLE_LINK`](crate::neat::PROB_MUTATE_TOGGLE_LINK)
    pub fn mutate(&mut self, neat: &mut Neat) {
        if Config::global().prob_mutate_link > rand::random() {
            self.mutate_link(neat);
        }
        if Config::global().prob_mutate_node > rand::random() {
            self.mutate_node(neat);
        }
        if Config::global().prob_mutate_weight_shift > rand::random() {
            self.mutate_weight_shift();
        }
        if Config::global().prob_mutate_weight_random > rand::random() {
            self.mutate_weight_random();
        }
        if Config::global().prob_mutate_toggle_link > rand::random() {
            self.mutate_link_toggle();
        }
    }

    /// Mutate a new link
    pub fn mutate_link(&mut self, neat: &mut Neat) {
        if self.nodes.len() <= 1 {
            return
        }

        for _ in 0..100 {
            let node1 = *self.nodes.random_element().expect("Nodes array is empty");
            let node2 = *self.nodes.random_element().expect("Nodes array is empty");

            if node1.x == node2.x {
                continue;
            }

            let connection = if node1.x.parse() < node2.x.parse() {
                ConnectionGene::new(node1, node2)
            }
            else {
                ConnectionGene::new(node2, node1)
            };

            if self.connections.contains(&connection) {
                continue;
            }

            let mut connection = neat.get_connection(connection.from, connection.to);
            let result = Genome::get_random_range(Config::global().weight_shift_strength);
            connection.weight = PseudoFloat::new(result);

            self.connections.add_sorted(connection);
            return;
        }
    }

    /// Mutate a new node
    pub fn mutate_node(&mut self, neat: &mut Neat) {
        if let Some(connection) = self.connections.clone().random_element() {
            let from = connection.from;
            let to = connection.to;

            let replace_index = neat.get_replace_index(from, to);
            let middle: NodeGene;
            if replace_index == 0 {
                let x = (from.x.parse() + to.x.parse()) / 2.0;
                let y = (from.y.parse() + to.y.parse()) / 2.0 + Genome::get_random_range(0.05);

                middle = neat.create_node(x, y);
                neat.set_replace_index(
                    from, to, 
                    middle.innovation_number.try_into().expect("usize too big to convert to u32")
                );
                println!("replace_index: {:?}, replace_index: {:?}", middle.innovation_number, neat.get_replace_index(from, to));
            }
            else {
                println!("HERE");
                middle = neat.get_node(replace_index).expect("Failed to get the replace_index");
                println!("HERE 2");
            }

            let mut connection1 = neat.get_connection(from, middle);
            let mut connection2= neat.get_connection(middle, to);

            connection1.weight = PseudoFloat::new(1.0);
            connection2.weight = connection.weight;
            connection2.enabled = connection.enabled;

            self.connections.remove_value(connection);
            self.connections.add(connection1);
            self.connections.add(connection2);

            self.nodes.add(middle);
        }
    }

    /// Get a random range from -constant to constant inclusive
    fn get_random_range(constant: f32) -> f32 {
        thread_rng().gen_range(-constant..=constant)
    }

    /// Mutate weight shift
    pub fn mutate_weight_shift(&mut self) {
        if let Some(connection) = self.connections.random_element() {
            let result = connection.weight.parse() + Genome::get_random_range(Config::global().weight_shift_strength);
            connection.weight = PseudoFloat::new(result);
        }
    }

    /// Mutate a weight and assign a new value to it
    pub fn mutate_weight_random(&mut self) {
        if let Some(connection) = self.connections.random_element() {
            let result = Genome::get_random_range(Config::global().weight_random_strength);
            connection.weight = PseudoFloat::new(result);
        }
    }

    /// Toggle the enabled status of a link
    pub fn mutate_link_toggle(&mut self) {
        if let Some(connection) = self.connections.random_element() {
            connection.enabled = !connection.enabled;
        }
    }
}

impl Default for Genome {
    fn default() -> Self {
        Self::new()
    }
}