use std::{collections::HashSet, hash::Hash, fmt::Display};
use rand::{self, seq::SliceRandom};

/// A has
pub struct RandomHashSet<T> where T: Eq + Hash + Copy {
    set: HashSet<T>,
    data: Vec<T>,
}

impl<T> RandomHashSet<T> where T: Eq + Hash + Copy {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
            data: Vec::new(),
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }

    pub fn random_element(&self) -> Option<&T> {
        self.data.choose(&mut rand::thread_rng())
    }

    fn len(&self) -> usize {
        self.set.len()
    }

    pub fn add(&mut self, value: T) {
        if !self.contains(&value) {
            self.set.insert(value);
            self.data.push(value);
        }
    }

    pub fn clear(&mut self) {
        self.set.clear();
        self.data.clear();
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

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

    pub fn remove_value(&mut self, value: &T) -> bool {
        match self.set.remove(value) {
            true => {
                self.data.remove(
                    self.data.iter().position(|&v| v == *value).expect("Failed to find value in self.data")
                );
                true
            },
            false => false
        }
    }
}