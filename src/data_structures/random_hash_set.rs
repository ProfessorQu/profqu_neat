use std::collections::HashSet;
use std::hash::Hash;
use rand::seq::SliceRandom;

use crate::genome::gene::Gene;

/// A hashset with some data that can get a random item
#[derive(Clone)]
pub struct RandomHashSet<T> where T: Eq + Hash + Clone + Gene {
    set: HashSet<T>,
    pub data: Vec<T>,
}

impl<T> RandomHashSet<T> where T: Eq + Hash + Clone + Gene {
    /// Create a new hash set
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
            data: Vec::new(),
        }
    }

    /// Test if the set contains a value
    pub fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }

    /// Get a random element from the set
    pub fn random_element(&self) -> Option<&T> {
        self.data.choose(&mut rand::thread_rng())
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    /// Add a new element to the set
    pub fn add(&mut self, value: T) {
        if !self.contains(&value) {
            self.set.insert(value.clone());
            self.data.push(value.clone());
        }
    }

    /// Add something to the hash set in the correct position
    pub fn add_sorted(&mut self, value: T) {
        let pos = self.data.binary_search_by(
            |probe| probe.get_innovation_number()
                            .cmp(&value.get_innovation_number())
        ).unwrap_or_else(|e| e);

        self.data.insert(pos, value.clone());
        self.set.insert(value);
    }

    /// Clear the entire set
    pub fn clear(&mut self) {
        self.set.clear();
        self.data.clear();
    }

    /// Get a value at an index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Remove an item at a certain index
    pub fn remove_index(&mut self, index: usize) -> bool {
        match self.data.get(index) {
            Some(result) => {
                self.set.remove(result);
                self.data.remove(index);
                true
            },
            None => false
        }
    }

    /// Remove an item at a certain value
    pub fn remove_value(&mut self, value: &T) -> bool {
        match self.set.remove(value) {
            true => {
                self.data.remove(
                    self.data
                            .iter()
                            .position(|v| v == value)
                            .expect("Failed to find value in self.data")
                );
                true
            },
            false => false
        }
    }
}

#[cfg(test)]
mod tests {
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
        let node = NodeGene::new(0);

        set.add(node.clone());

        assert_eq!(set.data.len(), 1);
        assert_eq!(set.set.len(), 1);
        assert_eq!(set.len(), set.data.len());

        assert!(set.contains(&node));

        // ----- Add second node -----
        let node2 = NodeGene::new(1);

        set.add(node2.clone());

        assert_eq!(set.data.len(), 2);
        assert_eq!(set.set.len(), 2);
        assert_eq!(set.len(), set.data.len());

        assert!(set.contains(&node));
        assert!(set.contains(&node2));

        // ----- Remove by index -----
        set.remove_index(0);
        
        assert_eq!(set.data.len(), 1);
        assert_eq!(set.set.len(), 1);
        assert_eq!(set.len(), set.data.len());

        assert!(!set.contains(&node));
        assert!(set.contains(&node2));

        // Add the node back in
        set.add(node.clone());

        // ----- Remove by value -----
        set.remove_value(&node2);

        assert_eq!(set.data.len(), 1);
        assert_eq!(set.set.len(), 1);
        assert_eq!(set.len(), set.data.len());

        assert!(set.contains(&node));
        assert!(!set.contains(&node2));

        // Add the node back in
        set.add(node2.clone());

        // ----- Clear -----
        set.clear();

        assert_eq!(set.len(), 0);
    }

    #[test]
    fn get() {
        let mut set = RandomHashSet::<NodeGene>::new();

        // ----- Add first node -----
        let node = NodeGene::new(0);
        set.add(node.clone());

        assert_eq!(set.get(0), Some(&node));
        
        // ----- Add second node -----
        let node2 = NodeGene::new(1);
        set.add(node2.clone());

        assert_eq!(set.get(0), Some(&node));
        assert_eq!(set.get(1), Some(&node2));
    }

    #[test]
    fn add_sorted() {
        let mut set = RandomHashSet::<NodeGene>::new();

        // ----- Add first node -----
        let node = NodeGene::new(3);
        set.add_sorted(node.clone());

        assert_eq!(set.get(0), Some(&node));
        
        // ----- Add second node -----
        let node2 = NodeGene::new(1);
        set.add_sorted(node2.clone());

        assert_eq!(set.get(0), Some(&node2));
        assert_eq!(set.get(1), Some(&node));

        let node3 = NodeGene::new(2);
        set.add_sorted(node3.clone());
        
        assert_eq!(set.get(0), Some(&node2));
        assert_eq!(set.get(1), Some(&node3));
        assert_eq!(set.get(2), Some(&node));
    }

    #[test]
    fn random_element() {
        let mut set = RandomHashSet::<NodeGene>::new();

        // ----- Add first node -----
        let node = &NodeGene::new(3);
        set.add_sorted(node.clone());
        
        // ----- Add second node -----
        let node2 = &NodeGene::new(1);
        set.add_sorted(node2.clone());

        let node3 = &NodeGene::new(2);
        set.add_sorted(node3.clone());

        let mut choices = Vec::new();

        for i in 0..50 {
            let element = set.random_element().expect("No elements in set");
            assert!(
                (element.innovation_number == node.innovation_number)
                ^ (element.innovation_number == node2.innovation_number)
                ^ (element.innovation_number == node3.innovation_number)
            );

            choices.push(element);
        }

        assert!(choices.contains(&node));
        assert!(choices.contains(&node2));
        assert!(choices.contains(&node3));
    }
}