use std::collections::HashSet;
use std::hash::Hash;
use rand::seq::SliceRandom;

use crate::genome::Gene;

#[cfg(test)]
#[path ="random_hash_set_test.rs"]
mod random_hash_set_test;

/// A hashset with some data that can get a random item
#[derive(Clone)]
pub struct RandomHashSet<T> where T: Eq + Hash + Clone + Gene + Copy {
    set: HashSet<T>,
    pub data: Vec<T>,
}

impl<T> RandomHashSet<T> where T: Eq + Hash + Clone + Gene + Copy {
    /// Create a new hash set
    /// ```rust
    /// use profqu_neat::genome::NodeGene;
    /// use profqu_neat::data_structures::RandomHashSet;
    /// 
    /// let mut set: RandomHashSet<NodeGene> = RandomHashSet::new();
    /// ```
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
            data: Vec::new(),
        }
    }

    /// Test if the set contains a value
    /// ```rust
    /// use profqu_neat::genome::NodeGene;
    /// use profqu_neat::data_structures::RandomHashSet;
    /// 
    /// let mut set: RandomHashSet<NodeGene> = RandomHashSet::new();
    /// let node1 = NodeGene::new(0);
    /// 
    /// set.add(node1);
    /// 
    /// assert!(set.contains(&node1));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }

    /// Get a value at an index
    /// ```rust
    /// use profqu_neat::genome::NodeGene;
    /// use profqu_neat::data_structures::RandomHashSet;
    /// 
    /// let mut set = RandomHashSet::<NodeGene>::new();
    /// 
    /// let node1 = NodeGene::new(0);
    /// set.add(node1);
    /// 
    /// assert_eq!(set.get(0), Some(&node1));
    /// 
    /// let node2 = NodeGene::new(1);
    /// set.add(node2);
    /// 
    /// assert_eq!(set.get(0), Some(&node1));
    /// assert_eq!(set.get(1), Some(&node2));
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Get a random element from the set
    /// ```rust
    /// use profqu_neat::genome::NodeGene;
    /// use profqu_neat::data_structures::RandomHashSet;
    /// 
    /// let mut set = RandomHashSet::<NodeGene>::new();
    /// 
    /// let mut node1 = NodeGene::new(0);
    /// set.add(node1);
    /// 
    /// let mut node2 = NodeGene::new(1);
    /// set.add(node2);
    /// 
    /// let element = set.random_element();
    /// 
    /// assert!(
    ///     element == Some(&mut node1)
    ///     || element == Some(&mut node2)
    /// );
    /// ```
    pub fn random_element(&mut self) -> Option<&mut T> {
        self.data.choose_mut(&mut rand::thread_rng())
    }

    /// Get the length of this hash set
    /// ```rust
    /// use profqu_neat::genome::NodeGene;
    /// use profqu_neat::data_structures::RandomHashSet;
    /// 
    /// let mut set = RandomHashSet::<NodeGene>::new();
    /// assert_eq!(set.len(), 0);
    /// 
    /// let mut node1 = NodeGene::new(0);
    /// set.add(node1);
    /// 
    /// assert_eq!(set.len(), 1);
    /// 
    /// let mut node2 = NodeGene::new(1);
    /// set.add(node2);
    /// 
    /// assert_eq!(set.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.set.len()
    }
    /// Get the length of this hash set
    /// ```rust
    /// use profqu_neat::genome::NodeGene;
    /// use profqu_neat::data_structures::RandomHashSet;
    /// 
    /// let mut set = RandomHashSet::<NodeGene>::new();
    /// assert!(set.is_empty());
    /// 
    /// let mut node1 = NodeGene::new(0);
    /// set.add(node1);
    /// 
    /// assert!(!set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    /// Add a new element to the set
    /// ```rust
    /// use profqu_neat::genome::NodeGene;
    /// use profqu_neat::data_structures::RandomHashSet;
    /// 
    /// let mut set: RandomHashSet<NodeGene> = RandomHashSet::new();
    /// let node1 = NodeGene::new(42);
    /// 
    /// set.add(node1);
    /// 
    /// assert!(set.contains(&node1));
    /// ```
    pub fn add(&mut self, value: T) {
        if !self.contains(&value) {
            self.set.insert(value);
            self.data.push(value);
        }
    }

    /// Add something to the hash set according to it's innovation number
    /// Add a new element to the set
    /// ```rust
    /// use profqu_neat::genome::NodeGene;
    /// use profqu_neat::data_structures::RandomHashSet;
    /// 
    /// let mut set = RandomHashSet::<NodeGene>::new();
    /// 
    /// let node = NodeGene::new(3);
    /// set.add_sorted(node.clone());
    /// 
    /// assert_eq!(set.get(0), Some(&node));
    /// 
    /// let node2 = NodeGene::new(1);
    /// set.add_sorted(node2.clone());
    /// 
    /// assert_eq!(set.get(0), Some(&node2));
    /// assert_eq!(set.get(1), Some(&node));
    /// ```
    pub fn add_sorted(&mut self, value: T) {
        let pos = self.data.binary_search_by(
            |probe| probe.get_innovation_number()
                            .cmp(&value.get_innovation_number())
        ).unwrap_or_else(|e| e);

        self.data.insert(pos, value);
        self.set.insert(value);
    }

    /// Clear the entire set
    /// ```rust
    /// use profqu_neat::genome::NodeGene;
    /// use profqu_neat::data_structures::RandomHashSet;
    /// 
    /// let mut set = RandomHashSet::<NodeGene>::new();
    /// assert_eq!(set.len(), 0);
    /// 
    /// let mut node1 = NodeGene::new(0);
    /// set.add(node1);
    /// 
    /// assert_eq!(set.len(), 1);
    /// 
    /// let mut node2 = NodeGene::new(1);
    /// set.add(node2);
    /// 
    /// assert_eq!(set.len(), 2);
    /// 
    /// set.clear();
    /// 
    /// assert_eq!(set.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.set.clear();
        self.data.clear();
    }

    /// Remove an item at a certain index
    /// ```rust
    /// use profqu_neat::genome::NodeGene;
    /// use profqu_neat::data_structures::RandomHashSet;
    /// 
    /// let mut set = RandomHashSet::<NodeGene>::new();
    /// assert_eq!(set.len(), 0);
    /// 
    /// let mut node1 = NodeGene::new(0);
    /// set.add(node1);
    /// 
    /// assert!(set.contains(&node1));
    /// 
    /// let mut node2 = NodeGene::new(1);
    /// set.add(node2);
    /// 
    /// assert!(set.contains(&node1));
    /// assert!(set.contains(&node2));
    /// 
    /// set.remove_index(0);
    /// 
    /// assert!(!set.contains(&node1));
    /// assert!(set.contains(&node2));
    /// ```
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

    /// Remove an item with a certain value
    /// ```rust
    /// use profqu_neat::genome::NodeGene;
    /// use profqu_neat::data_structures::RandomHashSet;
    /// 
    /// let mut set = RandomHashSet::<NodeGene>::new();
    /// assert_eq!(set.len(), 0);
    /// 
    /// let mut node1 = NodeGene::new(0);
    /// set.add(node1);
    /// 
    /// assert!(set.contains(&node1));
    /// 
    /// let mut node2 = NodeGene::new(1);
    /// set.add(node2);
    /// 
    /// assert!(set.contains(&node1));
    /// assert!(set.contains(&node2));
    /// 
    /// set.remove_value(&node2);
    /// 
    /// assert!(set.contains(&node1));
    /// assert!(!set.contains(&node2));
    /// ```
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

impl<T> Default for RandomHashSet<T> where T: Eq + Hash + Clone + Gene + Copy {
    fn default() -> Self {
        Self::new()
    }
}