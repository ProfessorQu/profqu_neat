use core::panic;
use std::cmp::{Ordering, max};

use super::{connection_gene::ConnectionGene, node_gene::NodeGene};
use crate::data_structures::random_hash_set::RandomHashSet;
use crate::neat::Neat; 
use crate::neat::{DISJOINT_MULT, WEIGHT_DIFF_MULT, EXCESS_MULT};

/// Teh genome with the connections and nodes
#[derive(Clone)]
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
        match self.connections.get(self.connections.len()) {
            Some(gene) => gene.innovation_number,
            None => 0
        }
    }

    pub fn add_connection(&mut self, neat: &mut Neat, index1: usize, index2: usize) {
        self.connections.add(
            neat.get_connection(
                self.nodes.get(index1).expect("Failed to find node in genome with index1").clone(),
                self.nodes.get(index2).expect("Failed to find node in genome with index2").clone()
            )
        );
    }
    
    fn order_genomes(input_genome1: Genome, input_genome2: Genome) -> (Genome, Genome) {
        match input_genome1.highest_innov_num().cmp(&input_genome2.highest_innov_num()) {
            Ordering::Less =>   (input_genome2, input_genome1),
            _ =>                (input_genome1, input_genome2)
        }
    }

    fn get_connection(&self, index: usize) -> &ConnectionGene {
        self.connections.get(index).expect("Index out of range")
    }

    /// Calculate the distance between this and another genome
    pub fn distance(input_genome1: &Genome, input_genome2: &Genome) -> f32 {
        // If both genomes have no connections, their distance is 0
        if input_genome1.connections.len() == 0 && input_genome2.connections.len() == 0 {
            return 0.0
        }

        // Set the highest genome to be genome1
        let (genome1, genome2) = Genome::order_genomes(input_genome1.clone(), input_genome2.clone());

        let mut index1 = 0;
        let mut index2 = 0;

        let mut num_disjoint = 0usize;
        let mut num_excess = 0;
        let mut total_weight_diff = 0.0;
        let mut num_weight_similar = 0usize;

        // Go through all connections
        while index1 < genome1.connections.len() && index2 < genome2.connections.len() {
            let connection1 = genome1.get_connection(index1);
            let connection2 = genome2.get_connection(index2);

            let in1 = connection1.innovation_number;
            let in2 = connection2.innovation_number;

            match in1.cmp(&in2) {
                Ordering::Equal => {    // Same gene
                    index1 += 1;
                    index2 += 1;
                    total_weight_diff += (connection1.weight.parse() - connection2.weight.parse()).abs();
                    num_weight_similar += 1;
                },
                Ordering::Greater => {  // Disjoint gene of genome 1
                    index2 += 1;
                    num_disjoint += 1;
                },
                Ordering::Less => {     // Disjoint gene of genome 2
                    index1 += 1;
                    num_disjoint += 1;
                },
            }
        }

        let average_weight_diff = match num_weight_similar {
            0 => 0.0,
            _ => total_weight_diff / num_weight_similar as f32
        };

        num_excess = genome1.connections.len() - index1;

        let mut total_genes = max(genome1.connections.len(), genome2.connections.len()) as f32;
        if total_genes < 20.0 {
            total_genes = 1.0;
        }

        DISJOINT_MULT * num_disjoint as f32 / total_genes +
        EXCESS_MULT * num_excess as f32 / total_genes +
        WEIGHT_DIFF_MULT * average_weight_diff
    }

    /// Crossover two genomes
    pub fn crossover(neat: &mut Neat, input_genome1: &Genome, input_genome2: &Genome) -> Self {
        // Set the highest genome to be genome1
        let (genome1, genome2) = Genome::order_genomes(input_genome1.clone(), input_genome2.clone());

        let mut result_genome = neat.empty_genome();

        let mut index1 = 0;
        let mut index2 = 0;

        // Go through all connections
        while index1 < genome1.connections.len() && index2 < genome2.connections.len() {
            let connection1 = genome1.get_connection(index1);
            let connection2 = genome2.get_connection(index2);

            let in1 = connection1.innovation_number;
            let in2 = connection2.innovation_number;

            // Add connections to the result genome accordingly
            match in1.cmp(&in2) {
                Ordering::Equal => {    // Same gene
                    if rand::random() {
                        result_genome.connections.add(connection1.clone());
                    }
                    else {
                        result_genome.connections.add(connection2.clone());
                    }

                    index1 += 1;
                    index2 += 1;
                },
                Ordering::Greater => {  // Disjoint gene of genome 1
                    index2 += 1;
                },
                Ordering::Less => {     // Disjoint gene of genome 2
                    result_genome.connections.add(connection1.clone());

                    index1 += 1;
                },
            }
        }

        // Add all the excess nodes to the result genome
        while index1 < genome1.connections.len() {
            let connection1 = genome1.get_connection(index1);
            
            result_genome.connections.add(connection1.clone());

            index1 += 1;
        }

        for connection in result_genome.clone().connections.data {
            result_genome.nodes.add(connection.from);
            result_genome.nodes.add(connection.to);
        }

        result_genome
    }

    /// Mutate this genome
    pub fn mutate(&self) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance() {
        let mut neat = Neat::new(2, 2, 3);
        
        let mut genome1 = neat.empty_genome();
        let mut genome2 = neat.empty_genome();
        
        // Test that the distances are all zero for empty genomes
        assert_eq!(Genome::distance(&genome1, &genome1), 0.0);
        assert_eq!(Genome::distance(&genome2, &genome2), 0.0);
        assert_eq!(Genome::distance(&genome1, &genome2), 0.0);

        // Create and add a new connection to genome1
        genome1.add_connection(&mut neat, 0, 2);

        // Now test the distances again
        assert_eq!(Genome::distance(&genome1, &genome1), 0.0);
        assert_eq!(Genome::distance(&genome1, &genome2), 1.0);
        
        // Create and add a new connection to genome2 which is identical to genome1
        genome2.add_connection(&mut neat, 0, 2);
        
        // Now test the distances again
        assert_eq!(Genome::distance(&genome2, &genome2), 0.0);
        assert_eq!(Genome::distance(&genome1, &genome2), 0.0);
    }

    #[test]
    fn crossover() {
        let mut neat = Neat::new(2, 2, 3);

        let mut genome1 = neat.empty_genome();
        let mut genome2 = neat.empty_genome();

        // Crossover
        let baby = Genome::crossover(&mut neat, &genome1, &genome2);

        // Test distance
        assert_eq!(Genome::distance(&genome1, &genome2), 0.0);
        assert_eq!(Genome::distance(&genome1, &baby), 0.0);

        // Add connection
        genome1.add_connection(&mut neat, 0, 2);
        
        // Test distance with connection
        assert_eq!(Genome::distance(&genome1, &genome2), 1.0);
        assert_eq!(Genome::distance(&genome1, &baby), 1.0);
        
        // Create a new crossover
        let baby = Genome::crossover(&mut neat, &genome1, &genome2);
        
        // Distances have shifted
        assert_eq!(Genome::distance(&genome1, &genome2), 1.0);
        assert_eq!(Genome::distance(&genome1, &baby), 0.0);
        
        // Add a connection to genome2
        genome2.add_connection(&mut neat, 3, 2);
        
        assert_eq!(Genome::distance(&genome1, &genome2), 1.0);
        assert_eq!(Genome::distance(&genome2, &baby), 2.0);
        
        // Crossover again to get closer to both
        let baby = Genome::crossover(&mut neat, &genome1, &genome2);

        // Now test the distance again
        assert_eq!(Genome::distance(&genome1, &genome2), 1.0);
        assert_eq!(Genome::distance(&genome2, &baby), 2.0);
    }
}