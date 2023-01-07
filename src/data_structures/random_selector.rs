use rand::{seq::SliceRandom, distributions::WeightedError};

/// A random selector with weights (A.K.A. scores)
pub struct RandomSelector<T> {
    objects: Vec<(T, f32)>,
    total_score: f32,
}

impl<T> RandomSelector<T> {
    /// Create a new random selector
    /// ```rust
    /// use profqu_neat::genome::NodeGene;
    /// use profqu_neat::data_structures::RandomSelector;
    /// 
    /// let mut selector: RandomSelector<NodeGene> = RandomSelector::new();
    /// ```
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            total_score: 0.0
        }
    }

    /// Add a new value to the objects
    /// ```rust
    /// use profqu_neat::genome::NodeGene;
    /// use profqu_neat::data_structures::RandomSelector;
    /// 
    /// let mut selector: RandomSelector<NodeGene> = RandomSelector::new();
    /// 
    /// for i in 0..10 {
    ///     assert_eq!(selector.len(), i);
    /// 
    ///     let score = rand::random();
    ///     let node = NodeGene::new(rand::random());
    ///     selector.add(node, score);
    /// }
    /// 
    /// assert_eq!(selector.len(), 10);
    /// ```
    pub fn add(&mut self, value: T, score: f32) {
        self.objects.push((value, score));
        self.total_score += score;
    }

    /// Choose a random element according to the scores
    /// ```rust
    /// use profqu_neat::genome::NodeGene;
    /// use profqu_neat::data_structures::RandomSelector;
    /// 
    /// let mut selector: RandomSelector<NodeGene> = RandomSelector::new();
    /// let node1 = NodeGene::new(1);
    /// let node2 = NodeGene::new(5);
    /// 
    /// selector.add(node1, 1.0);
    /// selector.add(node2, 5.0);
    /// 
    /// let mut results = Vec::new();
    /// 
    /// for _ in 0..100 {
    ///     results.push(selector.random_element().unwrap());
    /// }
    /// 
    /// // Check if the number of occurences of node1 is less than those of node2
    /// assert!(results.iter().filter(|element| element.0 == node1).count() <
    ///         results.iter().filter(|element| element.0 == node2).count())
    /// ```
    pub fn random_element(&self) -> Result<&(T, f32), WeightedError> {
        self.objects.choose_weighted(
            &mut rand::thread_rng(),
            |item| item.1
        )
    }

    /// Get the length of this dataset
    /// ```rust
    /// use profqu_neat::genome::NodeGene;
    /// use profqu_neat::data_structures::RandomSelector;
    /// 
    /// let mut selector: RandomSelector<NodeGene> = RandomSelector::new();
    /// 
    /// for i in 0..10 {
    ///     assert_eq!(selector.len(), i);
    /// 
    ///     let score = 42.0;
    ///     let node = NodeGene::new(2);
    ///     selector.add(node, score);
    /// }
    /// 
    /// assert_eq!(selector.len(), 10);
    /// ```
    pub fn len(&self) -> usize {
        self.objects.len()
    }


    /// Test if this dataset is empty
    /// ```rust
    /// use profqu_neat::genome::NodeGene;
    /// use profqu_neat::data_structures::RandomSelector;
    /// 
    /// let mut selector: RandomSelector<NodeGene> = RandomSelector::new();
    /// 
    /// assert!(selector.is_empty());
    /// 
    /// let score = 42.0;
    /// let node = NodeGene::new(2);
    /// selector.add(node, score);
    /// 
    /// assert!(!selector.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// Reset the selector and all data in it
    pub fn clear(&mut self) {
        self.objects.clear();
        self.total_score = 0.0;
    }
}

impl<T> Default for RandomSelector<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::genome::NodeGene;

    use super::*;

    #[test]
    fn new() {
        let selector: RandomSelector<NodeGene> = RandomSelector::new();

        assert_eq!(selector.len(), 0);
    }

    #[test]
    fn add() {
        let mut selector: RandomSelector<NodeGene> = RandomSelector::new();
        let node1 = NodeGene::new(42);

        for _ in 0..10 {
            let score = rand::random();
            selector.add(node1, score);

            assert!(selector.objects.contains(&(node1, score)));
        }
    }

    #[test]
    fn total_score() {
        let mut total_score = 0.0;
        
        let mut selector: RandomSelector<NodeGene> = RandomSelector::new();

        for _ in 0..100 {
            let score = rand::thread_rng().gen_range(0.0..10.0);
            total_score += score;
            selector.add(NodeGene::new(rand::random()), score);

            assert_eq!(selector.total_score, total_score);
        }
    }

    #[test]
    fn random_element() {
        let mut selector: RandomSelector<NodeGene> = RandomSelector::new();
        let node1 = NodeGene::new(1);
        let node2 = NodeGene::new(5);

        selector.add(node1, 1.0);
        selector.add(node2, 5.0);

        let mut results = Vec::new();

        for _ in 0..100 {
            results.push(selector.random_element().unwrap());
        }

        // Check if the number of occurences of node1 is less than those of node2
        assert!(results.iter().filter(|element| element.0 == node1).count() <
                results.iter().filter(|element| element.0 == node2).count())
    }
}