use super::*;

#[test]
fn new() {
    let neat = Neat::new(3, 3, 15);
    assert_eq!(neat.all_nodes.len(), 6);

    assert_eq!(neat.input_size, 3);
    assert_eq!(neat.output_size, 3);
    assert_eq!(neat.population_size, 15);
}

#[test]
fn inputs() {
    let neat = Neat::new(3, 3, 200);

    let x: f32 = neat.all_nodes[0].x.into();
    let y: f32 = neat.all_nodes[0].y.into();
    assert_eq!(x, 0.1);
    assert_eq!(y, 0.25);
    
    let x: f32 = neat.all_nodes[1].x.into();
    let y: f32 = neat.all_nodes[1].y.into();
    assert_eq!(x, 0.1);
    assert_eq!(y, 0.5);
    
    let x: f32 = neat.all_nodes[2].x.into();
    let y: f32 = neat.all_nodes[2].y.into();
    assert_eq!(x, 0.1);
    assert_eq!(y, 0.75);
}

#[test]
fn outputs() {
    let neat = Neat::new(3, 3, 40);

    let x: f32 = neat.all_nodes[3].x.into();
    let y: f32 = neat.all_nodes[3].y.into();
    assert_eq!(x, 0.9);
    assert_eq!(y, 0.25);
    
    let x: f32 = neat.all_nodes[4].x.into();
    let y: f32 = neat.all_nodes[4].y.into();
    assert_eq!(x, 0.9);
    assert_eq!(y, 0.5);
    
    let x: f32 = neat.all_nodes[5].x.into();
    let y: f32 = neat.all_nodes[5].y.into();
    assert_eq!(x, 0.9);
    assert_eq!(y, 0.75);
}

#[test]
fn empty_genome() {
    let mut neat = Neat::new(3, 3, 100);

    let genome = neat.empty_genome();

    for node in neat.all_nodes {
        assert!(genome.nodes.contains(&node));
    }
}

#[test]
fn get_connection() {
    let mut neat = Neat::new(3, 3, 100);

    for i in 0..10 {
        let node1 = NodeGene::new(i * 2);
        let node2 = NodeGene::new(1 + i * 2);

        let connection = neat.get_connection(node1, node2);
        let connection2 = neat.get_connection(node1, node2);

        let connection3 = neat.get_connection(node2, node1);

        // Test innovation numbers of same connections
        assert_eq!(connection.innovation_number, i * 2 + 1);
        assert_eq!(connection.innovation_number, connection2.innovation_number);

        // Test innovation numbers of different connections
        assert_eq!(connection3.innovation_number, i * 2 + 2);
        assert_ne!(connection.innovation_number, connection3.innovation_number);

        // Test equality of same and different connections
        assert_eq!(connection, connection2);
        assert_ne!(connection, connection3);

        // Test equality of hash codes of same and different connections
        assert_eq!(connection.hash_code(), connection2.hash_code());
        assert_ne!(connection2.hash_code(), connection3.hash_code());
    }
}