use crate::genome::{NodeGene, ConnectionGene};

use super::*;

#[test]
fn new() {
    let set = RandomHashSet::<NodeGene>::new();
    
    assert_eq!(set.data.len(), 0);
    assert_eq!(set.set.len(), 0);

    let set = RandomHashSet::<ConnectionGene>::new();
    
    assert_eq!(set.data.len(), 0);
    assert_eq!(set.set.len(), 0);
}

#[test]
fn add_remove() {
    let mut set = RandomHashSet::<NodeGene>::new();

    // ----- Add first node -----
    let node1 = NodeGene::new(0);

    set.add(node1);

    assert_eq!(set.data.len(), 1);
    assert_eq!(set.set.len(), 1);
    assert_eq!(set.len(), set.data.len());

    assert!(set.contains(&node1));

    // ----- Add second node -----
    let node2 = NodeGene::new(1);

    set.add(node2);

    assert_eq!(set.data.len(), 2);
    assert_eq!(set.set.len(), 2);
    assert_eq!(set.len(), set.data.len());

    assert!(set.contains(&node1));
    assert!(set.contains(&node2));

    // ----- Remove by index -----
    set.remove_index(0);
    
    assert_eq!(set.data.len(), 1);
    assert_eq!(set.set.len(), 1);
    assert_eq!(set.len(), set.data.len());

    assert!(!set.contains(&node1));
    assert!(set.contains(&node2));

    // Add the node back in
    set.add(node1);

    // ----- Remove by value -----
    set.remove_value(&node2);

    assert_eq!(set.data.len(), 1);
    assert_eq!(set.set.len(), 1);
    assert_eq!(set.len(), set.data.len());

    assert!(set.contains(&node1));
    assert!(!set.contains(&node2));

    // Add the node back in
    set.add(node2);

    // ----- Clear -----
    set.clear();

    assert_eq!(set.len(), 0);
}

#[test]
fn get() {
    let mut set = RandomHashSet::<NodeGene>::new();

    // ----- Add first node -----
    let node1 = NodeGene::new(0);
    set.add(node1);

    assert_eq!(set.get(0), Some(&node1));
    
    // ----- Add second node -----
    let node2 = NodeGene::new(1);
    set.add(node2);

    assert_eq!(set.get(0), Some(&node1));
    assert_eq!(set.get(1), Some(&node2));
}

#[test]
fn add_sorted() {
    let mut set = RandomHashSet::<NodeGene>::new();

    // ----- Add first node -----
    let node1 = NodeGene::new(3);
    set.add_sorted(node1);

    assert_eq!(set.get(0), Some(&node1));
    
    // ----- Add second node -----
    let node2 = NodeGene::new(1);
    set.add_sorted(node2);

    assert_eq!(set.get(0), Some(&node2));
    assert_eq!(set.get(1), Some(&node1));

    let node3 = NodeGene::new(2);
    set.add_sorted(node3);
    
    assert_eq!(set.get(0), Some(&node2));
    assert_eq!(set.get(1), Some(&node3));
    assert_eq!(set.get(2), Some(&node1));
}

#[test]
fn random_element() {
    let mut set = RandomHashSet::<NodeGene>::new();

    // ----- Add first node -----
    let node1 = NodeGene::new(3);
    set.add(node1);
    
    // ----- Add second node -----
    let node2 = NodeGene::new(1);
    set.add(node2);

    let node3 = NodeGene::new(2);
    set.add(node3);

    let element = set.random_element().expect("No elements in set");
    // Use ^ (exclusive or) to test that only one has the same innovation number
    assert!(
        (element.innovation_number == node1.innovation_number)
        ^ (element.innovation_number == node2.innovation_number)
        ^ (element.innovation_number == node3.innovation_number)
    );
}