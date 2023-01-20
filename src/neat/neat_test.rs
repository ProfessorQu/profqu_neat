use crate::genome::NodeGene;

use super::*;

#[test]
fn new() {
    Neat::test_config();
    let neat = Neat::new(3, 3, 15);
    assert_eq!(neat.all_nodes.len(), 7);

    assert_eq!(neat.input_size, 3);
    assert_eq!(neat.output_size, 3);
    assert_eq!(neat.population_size, 15);
}

#[test]
fn inputs() {
    Neat::test_config();
    let neat = Neat::new(3, 3, 200);

    let x = neat.all_nodes[0].x;
    let y = neat.all_nodes[0].y;
    assert_eq!(x, 0.1);
    assert_eq!(y, 0.25);

    let x = neat.all_nodes[1].x;
    let y = neat.all_nodes[1].y;
    assert_eq!(x, 0.1);
    assert_eq!(y, 0.5);

    let x = neat.all_nodes[2].x;
    let y = neat.all_nodes[2].y;
    assert_eq!(x, 0.1);
    assert_eq!(y, 0.75);
}

#[test]
fn outputs() {
    Neat::test_config();
    let neat = Neat::new(3, 3, 40);

    let x = neat.all_nodes[3].x;
    let y = neat.all_nodes[3].y;
    assert_eq!(x, 0.1);
    assert_eq!(y, 0.9);

    let x = neat.all_nodes[4].x;
    let y = neat.all_nodes[4].y;
    assert_eq!(x, 0.9);
    assert_eq!(y, 0.25);

    let x = neat.all_nodes[5].x;
    let y = neat.all_nodes[5].y;
    assert_eq!(x, 0.9);
    assert_eq!(y, 0.5);

    let x = neat.all_nodes[6].x;
    let y = neat.all_nodes[6].y;
    assert_eq!(x, 0.9);
    assert_eq!(y, 0.75);
}

#[test]
fn empty_genome() {
    Neat::test_config();
    let mut neat = Neat::new(3, 3, 100);

    let genome = neat.empty_genome();

    for node in neat.all_nodes {
        assert!(genome.nodes.contains(&node));
    }
}

#[test]
fn create_node() {
    Neat::test_config();
    let mut neat = Neat::new(4, 5, 25);

    let node = neat.create_node(0.0, 1.0);

    assert_eq!(node.x, 0.0);
    assert_eq!(node.y, 1.0);
    assert_eq!(node.innovation_number, 11);

    let node = neat.create_node(0.5, 3.4);

    assert_eq!(node.x, 0.5);
    assert_eq!(node.y, 3.4);
    assert_eq!(node.innovation_number, 12);
}

#[test]
fn get_connection() {
    Neat::test_config();
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

        // let mut hasher = DefaultHasher::new();
        // connection.hash(&mut hasher);
        // connection2.hash(&mut hasher);
        // connection3.hash(&mut hasher);

        // // Test equality of hash codes of same and different connections
        // assert_eq!(connection.hash(&mut hasher), connection2.hash(&mut hasher));
        // assert_ne!(connection2.hash(&mut hasher), connection3.hash(&mut hasher));
    }
}
