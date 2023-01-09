use std::{fmt::Debug, rc::Rc, cell::RefCell};

use rand::seq::SliceRandom;

use crate::{data_structures::PseudoFloat, genome::Genome, Neat};

use super::{Client, neat};

#[derive(Clone)]
pub struct Species {
    clients: Vec<Rc<RefCell<Client>>>,
    representative: Rc<RefCell<Client>>,
    pub average_fitness: PseudoFloat
}

impl Species {
    /// Create a new species from a representative
    pub fn new(representative: Rc<RefCell<Client>>) -> Self {
        Self {
            clients: vec![Rc::clone(&representative)],
            representative,
            average_fitness: PseudoFloat::new(0.0),
        }
    }

    fn get_random_element(&self) -> Rc<RefCell<Client>> {
        Rc::clone(self.clients.choose(&mut rand::thread_rng()).expect("No clients in this species"))
    }

    /// Put a new client in this species if possible
    pub fn put(&mut self, client: Rc<RefCell<Client>>) -> bool {
        if client.borrow().distance(&self.representative.borrow()) < neat::SPECIES_THRESHOLD {
            client.borrow_mut().has_species = true;
            self.clients.push(Rc::clone(&client));

            true
        }
        else {
            false
        }
    }

    /// Put a species in this species without any checks
    pub fn force_put(&mut self, client: Rc<RefCell<Client>>) {
        client.borrow_mut().has_species = true;
        self.clients.push(Rc::clone(&client));
    }

    /// Make this species go extinct
    pub fn go_extinct(&mut self) {
        for client in &self.clients {
            client.borrow_mut().has_species = false;
        }
    }

    /// Calculate a new average fitness for this species
    pub fn evaluate_fitness(&mut self) {
        let mut total_fitness = 0.0;
        for client in &self.clients {
            total_fitness += client.borrow().fitness.parse();
        }

        self.average_fitness = PseudoFloat::new(total_fitness / self.clients.len() as f32);
    }

    /// Reset this species
    pub fn reset(&mut self) {
        // TODO: Make RandomHashSet more general
        self.representative = self.get_random_element();
        for client in &mut self.clients {
            client.borrow_mut().has_species = false;
        }

        self.clients.clear();

        self.representative.borrow_mut().has_species = true;
        self.clients.push(Rc::clone(&self.representative));
        self.average_fitness = PseudoFloat::new(0.0);
    }

    /// Kill 50% of this species
    pub fn kill(&mut self, percentage: f32) {
        // Sort so that the lowest fitness is at index 0
        self.clients.sort_by(
            |a, b|
            a.borrow().fitness.parse().total_cmp(
                &b.borrow().fitness.parse()
            )
        );

        for _ in 0..(percentage * self.clients.len() as f32) as usize {
            self.clients[0].borrow_mut().has_species = false;
            self.clients.remove(0);
        }
    }

    // Select random clients and let them breed with eachother
    pub fn breed(&self, neat: &mut Neat) -> Genome {
        let client1 = self.get_random_element();
        let client2 = self.get_random_element();

        if client1.borrow().fitness.parse() > client2.borrow().fitness.parse() {
            Genome::crossover(neat, &client1.borrow().genome, &client2.borrow().genome)
        }
        else {
            Genome::crossover(neat, &client2.borrow().genome, &client1.borrow().genome)
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
        write!(f, "Species {{ len: {:?}, fitness: {:?} }}", self.len(), self.average_fitness)
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
        for _ in 0..5000 {
            genome1.mutate(&mut neat);
            genome2.mutate(&mut neat);
        }

        let representative = Client::new(genome1);

        let mut species = Species::new(representative);
        
        assert_eq!(species.len(), 1);
        assert!(!species.is_empty());

        let new = Client::new(genome2);

        assert!(!species.put(Rc::clone(&new)));
        
        assert_eq!(species.len(), 1);

        species.force_put(new);

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

        let rep = Client::new(genome1);
        rep.borrow_mut().fitness = PseudoFloat::new(10.0);

        let mut species = Species::new(rep);
        
        for _ in 0..10 {
            let mut genome = neat.empty_genome();
            for _ in 0..100 {
                genome.mutate(&mut neat);
            }

            let client = Client::new(genome);
            
            species.force_put(client);
        }

        assert_eq!(species.len(), 11);
    }
}
