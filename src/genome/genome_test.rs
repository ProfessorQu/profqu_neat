use crate::Neat;
use crate::genome::Genome;

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
    assert_eq!(Genome::distance(&genome1, &genome2), 2.0);
    
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
    assert_eq!(Genome::distance(&genome1, &genome2), 2.0);
    assert_eq!(Genome::distance(&genome1, &baby), 2.0);
    
    // Create a new crossover
    let baby = Genome::crossover(&mut neat, &genome1, &genome2);
    
    // Distances have shifted
    assert_eq!(Genome::distance(&genome1, &genome2), 2.0);
    assert_eq!(Genome::distance(&genome1, &baby), 0.0);
    
    // Add a connection to genome2
    genome2.add_connection(&mut neat, 3, 2);
    
    assert_eq!(Genome::distance(&genome1, &genome2), 5.0);
    assert_eq!(Genome::distance(&genome2, &baby), 5.0);
    
    // Crossover again to get closer to both
    let baby = Genome::crossover(&mut neat, &genome1, &genome2);

    // Now test the distance again
    assert_eq!(Genome::distance(&genome1, &genome2), 5.0);
    assert_eq!(Genome::distance(&genome1, &baby), 0.0);
    assert_eq!(Genome::distance(&genome2, &baby), 5.0);
}

#[test]
fn crossover2() {
    let mut neat = Neat::new(2, 2, 3);

    let mut genome1 = neat.empty_genome();
    let mut genome2 = neat.empty_genome();

    let mut baby = neat.empty_genome();
    
    let mut previous1: f32;
    let mut previous2: f32;

    for _ in 0..10 {
        for _ in 0..200 {
            genome1.mutate(&mut neat);
            genome2.mutate(&mut neat);
        }

        previous1 = Genome::distance(&genome1, &baby);
        previous2 = Genome::distance(&genome2, &baby);
    
        baby = Genome::crossover(&mut neat, &genome1, &genome2);

        let current1 = Genome::distance(&genome1, &baby);
        let current2 = Genome::distance(&genome2, &baby);

        assert!(previous1 > current1);
        assert_ne!(previous2, current2);
    }
}

#[test]
fn mutate() {
    let mut neat = Neat::new(5, 4, 90);

    let mut genome1 = neat.empty_genome();
    let mut genome2 = genome1.clone();

    assert_eq!(Genome::distance(&genome1, &genome2), 0.0);

    for _ in 0..10 {
        genome1.mutate(&mut neat);
        genome2.mutate(&mut neat);
    }

    let mut previous1 = genome1.clone();
    let mut previous2 = genome2.clone();

    for _ in 0..100 {
        for _ in 0..10 {
            genome1.mutate(&mut neat);
            genome2.mutate(&mut neat);
        }

        assert_ne!(genome1, previous1);
        assert_ne!(genome2, previous2);

        assert_ne!(genome1, genome2);

        previous1 = genome1.clone();
        previous2 = genome2.clone();
    }
}

#[test]
fn mutate2() {
    let mut neat = Neat::new(5, 4, 90);

    let mut genome1 = neat.empty_genome();
    let mut genome2 = genome1.clone();

    assert_eq!(Genome::distance(&genome1, &genome2), 0.0);

    let mut previous = Genome::distance(&genome1, &genome2);

    for _ in 0..5 {
        for _ in 0..100 {
            genome1.mutate(&mut neat);
            genome2.mutate(&mut neat);
        }
    
        let current = Genome::distance(&genome1, &genome2);
    
        assert_ne!(current, previous);

        previous = current;
    }
}

#[test]
fn mutate_link() {
    let mut neat = Neat::new(5, 4, 90);

    let mut genome = neat.empty_genome();

    assert_eq!(genome.nodes.len(), 9);
    assert_eq!(genome.connections.len(), 0);

    genome.mutate_link(&mut neat);

    assert_eq!(genome.nodes.len(), 9);
    assert_eq!(genome.connections.len(), 1);
    
    genome.mutate_link(&mut neat);
    
    assert_eq!(genome.nodes.len(), 9);
    assert_eq!(genome.connections.len(), 2);

    genome.mutate_link(&mut neat);
    
    assert_eq!(genome.nodes.len(), 9);
    assert_eq!(genome.connections.len(), 3);
}

#[test]
fn mutate_node() {
    let mut neat = Neat::new(2, 3, 90);

    let mut genome = neat.empty_genome();

    assert_eq!(genome.nodes.len(), 5);
    assert_eq!(genome.connections.len(), 0);

    genome.mutate_link(&mut neat);

    assert_eq!(genome.nodes.len(), 5);
    assert_eq!(genome.connections.len(), 1);
    
    genome.mutate_node(&mut neat);

    assert_eq!(genome.nodes.len(), 6);
    assert_eq!(genome.connections.len(), 2);

    genome.mutate_node(&mut neat);
    
    assert_eq!(genome.nodes.len(), 7);
    assert_eq!(genome.connections.len(), 3);
}

#[test]
fn mutate_node_with_disabled_link() {
    let mut neat = Neat::new(2, 3, 90);

    let mut genome = neat.empty_genome();

    assert_eq!(genome.nodes.len(), 5);
    assert_eq!(genome.connections.len(), 0);

    let node1 = neat.create_node(0.1, 0.5);
    let node2 = neat.create_node(0.9, 0.5);

    let mut connection = ConnectionGene::new(node1, node2);
    connection.enabled = false;

    genome.connections.add(connection);

    assert_eq!(genome.nodes.len(), 5);
    assert_eq!(genome.connections.len(), 1);
    assert!(!genome.get_connection(0).enabled);
    
    genome.mutate_node(&mut neat);

    assert_eq!(genome.nodes.len(), 6);
    assert_eq!(genome.connections.len(), 2);
    
    assert!(genome.get_connection(0).enabled);
    assert!(!genome.get_connection(1).enabled);

    genome.mutate_node(&mut neat);
    
    assert_eq!(genome.nodes.len(), 7);
    assert_eq!(genome.connections.len(), 3);
}

#[test]
fn mutate_weight_shift() {
    let mut neat = Neat::new(2, 3, 90);

    let mut genome = neat.empty_genome();

    genome.add_connection(&mut neat, 0, 2);
    assert_eq!(genome.get_connection(0).weight.parse(), 1.0);

    for _ in 0..10 {
        let weight = genome.get_connection(0).weight.parse();
        genome.mutate_weight_shift();
        let new_weight = genome.get_connection(0).weight.parse();
        let difference = (new_weight - weight).abs();
        assert!((0.0..neat::WEIGHT_SHIFT_STRENGTH).contains(&difference));
    }
}

#[test]
fn mutate_weight_random() {
    let mut neat = Neat::new(2, 3, 90);

    let mut genome = neat.empty_genome();

    genome.add_connection(&mut neat, 0, 2);
    assert_eq!(genome.get_connection(0).weight.parse(), 1.0);
    
    let mut previous = genome.get_connection(0).weight.parse();

    for _ in 0..10 {
        genome.mutate_weight_random();
    
        let current = genome.get_connection(0).weight.parse();
        assert_ne!(current, previous);
        previous = current;
    }
}

#[test]
fn mutate_link_toggle() {
    let mut neat = Neat::new(2, 3, 90);

    let mut genome = neat.empty_genome();

    genome.add_connection(&mut neat, 0, 2);
    genome.add_connection(&mut neat, 1, 2);
    
    let mut previous1 = genome.get_connection(0).enabled;
    let mut previous2 = genome.get_connection(1).enabled;

    for _ in 0..50 {
        genome.mutate_link_toggle();

        let current1 = genome.get_connection(0).enabled;
        let current2 = genome.get_connection(1).enabled;

        assert!(current1 != previous1 || current2 != previous2);
        
        previous1 = current1;
        previous2 = current2;
    }
}