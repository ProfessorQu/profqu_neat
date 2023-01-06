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

    /// Calculate the distance between this and another genome
    pub fn distance(input_genome1: &Genome, input_genome2: &Genome) -> f32 {
        // If both genomes have no connections, their distance is 0
        if input_genome1.connections.len() == 0 && input_genome2.connections.len() == 0 {
            return 0.0
        }

        // Set the highest genome to be genome1
        let (genome1, genome2) = match input_genome1.highest_innov_num().cmp(&input_genome2.highest_innov_num()) {
            Ordering::Less => (input_genome2.clone(), input_genome1.clone()),
            _ => (input_genome1.clone(), input_genome2.clone())
        };

        let mut index1 = 0;
        let mut index2 = 0;

        let mut num_disjoint = 0;
        let mut num_excess = 0;
        let mut total_weight_diff = 0.0;
        let mut num_weight_similar = 0;

        while index1 < genome1.connections.len() && index2 < genome2.connections.len() {
            let connection1 = genome1.connections.get(index1)
                .expect("index_self is greater than or equal to self.connections.len()");
            let connection2 = genome2.connections.get(index2)
                .expect("index_other is greater than or equal to other.connections.len()");

            let in1 = connection1.innovation_number;
            let in2 = connection2.innovation_number;

            // println!("in1, in2: {}, {}", in1, in2);

            match in1.cmp(&in2) {
                Ordering::Equal => {    // Same gene
                    index1 += 1;
                    index2 += 1;
                    total_weight_diff += (connection1.weight.parse() - connection2.weight.parse()).abs();
                    num_weight_similar += 1;
                },
                Ordering::Greater => {  // Disjoint/excess gene of gene 1
                    index2 += 1;
                    num_disjoint += 1;
                },
                Ordering::Less => {     // Disjoint/excess gene of gene 2
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

        (DISJOINT_MULT * num_disjoint as f32 / total_genes) +
        (EXCESS_MULT * num_excess as f32 / total_genes) +
        WEIGHT_DIFF_MULT * average_weight_diff
    }

    /// Crossover two genomes
    pub fn crossover(g1: Self, g2: Self) -> Self {
        Self::new()
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
        
        assert_eq!(Genome::distance(&genome1, &genome1), 0.0);
        assert_eq!(Genome::distance(&genome2, &genome2), 0.0);
        assert_eq!(Genome::distance(&genome1, &genome2), 0.0);

        let connection = neat.get_connection(
            genome1.nodes.get(0).expect("Failed to find node in genome with index 0").clone(),
            genome1.nodes.get(2).expect("Failed to find node in genome with index 2").clone()
        );

        genome1.connections.add(connection);

        assert_eq!(Genome::distance(&genome1, &genome1), 0.0);
        assert_eq!(Genome::distance(&genome1, &genome2), 1.0);

        let connection = neat.get_connection(
            genome2.nodes.get(0).expect("Failed to find node in genome with index 0").clone(),
            genome2.nodes.get(2).expect("Failed to find node in genome with index 2").clone()
        );
        
        genome2.connections.add(connection);
        
        assert_eq!(Genome::distance(&genome2, &genome2), 0.0);
        assert_eq!(Genome::distance(&genome1, &genome2), 0.0);
    }
}