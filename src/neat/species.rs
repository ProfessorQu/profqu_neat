use std::fmt::Debug;

use rand::seq::SliceRandom;

use crate::{data_structures::PseudoFloat, genome::Genome, Neat};

use super::{Client, neat};

#[derive(Clone)]
pub struct Species {
    clients: Vec<Client>,
    representative: Client,
    fitness: PseudoFloat
}

impl Species {
    /// Create a new species from a representative
    pub fn new(representative: Client) -> Self {
        Self {
            clients: vec![representative.clone()],
            representative,
            fitness: PseudoFloat::new(0.0),
        }
    }

    fn get_random_element(&self) -> Client {
        self.clients.choose(&mut rand::thread_rng()).expect("No clients in this species").clone()
    }

    /// Put a new client in this species if possible
    pub fn put(&mut self, client: &mut Client) -> bool {
        if client.distance(&self.representative) < neat::SPECIES_THRESHOLD {
            client.has_species = true;
            self.clients.push(client.clone());

            true
        }
        else {
            false
        }
    }

    /// Put a species in this species without any checks
    pub fn force_put(&mut self, client: &mut Client) {
        client.has_species = true;
        self.clients.push(client.clone());
    }

    /// Make this species go extinct
    pub fn go_extinct(&mut self) {
        for client in &mut self.clients {
            client.has_species = false;
        }
    }

    /// Calculate a new average fitness for this species
    pub fn evaluate_fitness(&mut self) {
        let mut total_fitness = 0.0;
        for client in &self.clients {
            total_fitness += client.fitness.parse();
        }

        self.fitness = PseudoFloat::new(total_fitness / self.clients.len() as f32);
    }

    /// Reset this species
    pub fn reset(&mut self) {
        // TODO: Make RandomHashSet more general
        self.representative = self.get_random_element();
        for client in &mut self.clients {
            client.has_species = false;
        }

        self.clients.clear();

        self.representative.has_species = true;
        self.clients.push(self.representative.clone());
        self.fitness = PseudoFloat::new(0.0);
    }

    /// Kill 50% of this species
    pub fn kill(&mut self, percentage: f32) {
        // Sort so that the lowest fitness is at index 0
        self.clients.sort_by(
            |a, b|
            a.fitness.parse().total_cmp(
                &b.fitness.parse()
            )
        );

        for i in 0..(percentage * self.clients.len() as f32) as usize {
            self.clients[i].has_species = false;
            self.clients.remove(i);
        }
    }

    // Select random clients and let them breed with eachother
    pub fn breed(&self, neat: &mut Neat) -> Genome {
        let client1 = self.get_random_element();
        let client2 = self.get_random_element();

        if client1.fitness.parse() > client2.fitness.parse() {
            Genome::crossover(neat, &client1.genome, &client2.genome)
        }
        else {
            Genome::crossover(neat, &client2.genome, &client1.genome)
        }
    }

    /// Return the length of all clients in this species
    pub fn len(&self) -> usize {
        self.clients.len()
    }

    /// Return if the clients are empty
    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }
}

impl Debug for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Species {{ representative: {:?}, fitness: {:?} }}", self.representative, self.fitness)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let mut neat = Neat::new(3, 3, 100);

        let mut genome = neat.empty_genome();
        for _ in 0..100 {
            genome.mutate(&mut neat);
        }
        
        let representative = Client::new(genome);

        let species = Species::new(representative);

        assert_eq!(species.len(), 1);
        assert!(!species.is_empty());
    }

    #[test]
    fn put() {
        let mut neat = Neat::new(3, 3, 100);

        let mut genome1 = neat.empty_genome();
        let mut genome2 = neat.empty_genome();
        for _ in 0..100 {
            genome1.mutate(&mut neat);
            genome2.mutate(&mut neat);
        }

        let representative = Client::new(genome1);

        let mut species = Species::new(representative);
        
        assert_eq!(species.len(), 1);
        assert!(!species.is_empty());

        let mut new = Client::new(genome2);

        assert!(!species.put(&mut new));
        
        assert_eq!(species.len(), 1);

        species.force_put(&mut new);

        assert_eq!(species.len(), 2);
    }

    #[test]
    fn go_extinct() {
        let mut neat = Neat::new(3, 3, 100);

        let mut genome1 = neat.empty_genome();
        let mut genome2 = neat.empty_genome();
        for _ in 0..100 {
            genome1.mutate(&mut neat);
            genome2.mutate(&mut neat);
        }

        let mut rep = Client::new(genome1);
        rep.fitness = PseudoFloat::new(10.0);

        let mut species = Species::new(rep);
        
        for _ in 0..10 {
            let mut genome = neat.empty_genome();
            for _ in 0..100 {
                genome.mutate(&mut neat);
            }

            let mut client = Client::new(genome);
            
            species.force_put(&mut client);
        }

        assert_eq!(species.len(), 11);
    }
}
