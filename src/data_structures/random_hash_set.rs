use std::collections::HashSet;
use std::hash::Hash;
use rand::seq::SliceRandom;

/// A hashset with some data that can get a random item
#[derive(Clone)]
pub struct RandomHashSet<T> where T: Eq + Hash + Clone {
    set: HashSet<T>,
    pub data: Vec<T>,
}

impl<T> RandomHashSet<T> where T: Eq + Hash + Clone {
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