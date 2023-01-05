use rand::{Rng, seq::SliceRandom, distributions::WeightedError};

pub struct RandomSelector<T> {
    objects: Vec<(T, f32)>,
    total_score: f32,
}

impl<T> RandomSelector<T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            total_score: 0.0
        }
    }

    pub fn add(&mut self, value: T, score: f32) {
        self.objects.push((value, score));
        self.total_score += score;
    }

    pub fn random_element(&self) -> Result<&(T, f32), WeightedError> {
        self.objects.choose_weighted(&mut rand::thread_rng(), |item| item.1)
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.total_score = 0.0;
    }
}