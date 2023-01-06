use rand::{seq::SliceRandom, distributions::WeightedError};

/// A random selector with weights (A.K.A. scores)
pub struct RandomSelector<T> {
    objects: Vec<(T, f32)>,
    total_score: f32,
}

impl<T> RandomSelector<T> {
    /// Create a new random selector
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            total_score: 0.0
        }
    }

    /// Add a new value to the objects
    pub fn add(&mut self, value: T, score: f32) {
        self.objects.push((value, score));
        self.total_score += score;
    }

    /// Choose a random element according to the scores
    pub fn random_element(&self) -> Result<&(T, f32), WeightedError> {
        self.objects.choose_weighted(
            &mut rand::thread_rng(),
            |item| item.1
        )
    }

    /// Reset the selector
    pub fn clear(&mut self) {
        self.objects.clear();
        self.total_score = 0.0;
    }
}