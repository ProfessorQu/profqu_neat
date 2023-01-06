use std::collections::HashSet;
use std::hash::Hash;
use rand::seq::SliceRandom;

use crate::genome::gene::Gene;

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
    pub fn random_element(&mut self) -> Option<&mut T> {
        self.data.choose_mut(&mut rand::thread_rng())
    }

    /// Get the length of this hash set
    pub fn len(&self) -> usize {
        self.set.len()
    }

    /// Add a new element to the set
    pub fn add(&mut self, value: T) {
        if !self.contains(&value) {
            self.set.insert(value);
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