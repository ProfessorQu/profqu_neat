use std::fs;

pub struct Config {
    /// The multiplier for the disjoint genes in the `distance` function
    pub MULT_DISJOINT: f32,
    /// The multiplier for the excess genes in the `distance` function
    pub MULT_EXCESS: f32,
    /// The multiplier for the weight difference in the `distance` function
    pub MULT_WEIGHT_DIFF: f32,

    /// The weight shifting strength when mutating
    pub WEIGHT_SHIFT_STRENGTH: f32,
    /// The weight randomness strength when mutating
    pub WEIGHT_RANDOM_STRENGTH: f32,

    /// The probability of mutating a new link
    pub PROB_MUTATE_LINK: f32,
    /// The probability of mutating a new node
    pub PROB_MUTATE_NODE: f32,
    /// The probability of mutating and shifting a weight
    pub PROB_MUTATE_WEIGHT_SHIFT: f32,
    /// The probability of mutating and selecting a new random value for a weight
    pub PROB_MUTATE_WEIGHT_RANDOM: f32,
    /// The probability of mutating and toggling a link
    pub PROB_MUTATE_TOGGLE_LINK: f32,

    /// The threshold for creating a new species
    pub SPECIES_THRESHOLD: f32,

    /// Determine the percentage of clients that will be killed
    pub KILL_PERCENTAGE: f32,
}

impl Config {
    pub fn new(MULT_DISJOINT: f32, MULT_EXCESS: f32, MULT_WEIGHT_DIFF: f32,
            WEIGHT_RANDOM_STRENGTH: f32, WEIGHT_SHIFT_STRENGTH: f32,
            PROB_MUTATE_LINK: f32, PROB_MUTATE_NODE: f32,
            PROB_MUTATE_WEIGHT_SHIFT: f32, PROB_MUTATE_WEIGHT_RANDOM: f32,
            PROB_MUTATE_TOGGLE_LINK: f32,
            SPECIES_THRESHOLD: f32,
            KILL_PERCENTAGE: f32) -> Self {
        Self {
            MULT_DISJOINT,
            MULT_EXCESS,
            MULT_WEIGHT_DIFF,
            
            WEIGHT_SHIFT_STRENGTH,
            WEIGHT_RANDOM_STRENGTH,
            
            PROB_MUTATE_LINK,
            PROB_MUTATE_NODE,
            PROB_MUTATE_WEIGHT_SHIFT,
            PROB_MUTATE_WEIGHT_RANDOM,
            PROB_MUTATE_TOGGLE_LINK,
            
            SPECIES_THRESHOLD,
            
            KILL_PERCENTAGE,
        }
    }

    pub fn init_zero() -> Self {
        Self {
            MULT_DISJOINT: 0.0,
            MULT_EXCESS: 0.0,
            MULT_WEIGHT_DIFF: 0.0,
            
            WEIGHT_SHIFT_STRENGTH: 0.0,
            WEIGHT_RANDOM_STRENGTH: 0.0,
            
            PROB_MUTATE_LINK: 0.0,
            PROB_MUTATE_NODE: 0.0,
            PROB_MUTATE_WEIGHT_SHIFT: 0.0,
            PROB_MUTATE_WEIGHT_RANDOM: 0.0,
            PROB_MUTATE_TOGGLE_LINK: 0.0,
            
            SPECIES_THRESHOLD: 0.0,
            
            KILL_PERCENTAGE: 0.0,
        }
    }

    pub fn from_file(filename: String) -> Self {
        let mut config = Config::init_zero();

        let content = fs::read_to_string(filename).expect("Error opening file");

        for line in content.lines() {
            let mut split = line.split(": ");

            while let Some(name) = split.next() {
                match name {
                    "MULT_DISJOINT" => config.MULT_DISJOINT = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "MULT_EXCESS" => config.MULT_EXCESS = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "MULT_WEIGHT_DIFF" => config.MULT_WEIGHT_DIFF = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    
                    "WEIGHT_SHIFT_STRENGTH" => config.WEIGHT_SHIFT_STRENGTH = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "WEIGHT_RANDOM_STRENGTH" => config.WEIGHT_RANDOM_STRENGTH = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    
                    "PROB_MUTATE_LINK" => config.PROB_MUTATE_LINK = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "PROB_MUTATE_NODE" => config.PROB_MUTATE_NODE = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "PROB_MUTATE_WEIGHT_SHIFT" => config.PROB_MUTATE_WEIGHT_SHIFT = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "PROB_MUTATE_WEIGHT_RANDOM" => config.PROB_MUTATE_WEIGHT_RANDOM = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "PROB_MUTATE_TOGGLE_LINK" => config.PROB_MUTATE_TOGGLE_LINK = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    
                    "SPECIES_THRESHOLD" => config.SPECIES_THRESHOLD = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "KILL_PERCENTAGE" => config.KILL_PERCENTAGE = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    _ => panic!("No recognized pattern")
                }
            }
        }

        config
    }
}
