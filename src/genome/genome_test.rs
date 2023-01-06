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