use std::fs;
use once_cell::sync::OnceCell;

pub static CONFIG: OnceCell<Config> = OnceCell::new();

pub struct Config {
    /// The multiplier for the disjoint genes in the `distance` function
    pub mult_disjoint: f32,
    /// The multiplier for the excess genes in the `distance` function
    pub mult_excess: f32,
    /// The multiplier for the weight difference in the `distance` function
    pub mult_weight_diff: f32,

    /// The weight shifting strength when mutating
    pub weight_shift_strength: f32,
    /// The weight randomness strength when mutating
    pub weight_random_strength: f32,

    /// The probability of mutating a new link
    pub prob_mutate_link: f32,
    /// The probability of mutating a new node
    pub prob_mutate_node: f32,
    /// The probability of mutating and shifting a weight
    pub prob_mutate_weight_shift: f32,
    /// The probability of mutating and selecting a new random value for a weight
    pub prob_mutate_weight_random: f32,
    /// The probability of mutating and toggling a link
    pub prob_mutate_toggle_link: f32,

    /// The threshold for creating a new species
    pub species_threshold: f32,

    /// Determine the percentage of clients that will be killed
    pub kill_percentage: f32,
}

impl Config {
    pub fn from_vec(variables: Vec<f32>) -> Self {
        if variables.len() != 12 {
            panic!("variables should have length 12");
        }
        
        Self {
            mult_disjoint: variables[0],
            mult_excess: variables[1],
            mult_weight_diff: variables[2],
            
            weight_shift_strength: variables[3],
            weight_random_strength: variables[4],
            
            prob_mutate_link: variables[5],
            prob_mutate_node: variables[6],
            prob_mutate_weight_shift: variables[7],
            prob_mutate_weight_random: variables[8],
            prob_mutate_toggle_link: variables[9],
            
            species_threshold: variables[10],
            
            kill_percentage: variables[11],
        }
    }

    pub fn from_file(filename: &str) -> Self {
        let mut config = Config::init_zero();

        let content = fs::read_to_string(filename).expect("Error opening file");

        for line in content.lines() {
            let mut split = line.split(": ");

            while let Some(name) = split.next() {
                match name {
                    "mult_disjoint" => config.mult_disjoint = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "mult_excess" => config.mult_excess = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "mult_weight_diff" => config.mult_weight_diff = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    
                    "weight_shift_strength" => config.weight_shift_strength = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "weight_random_strength" => config.weight_random_strength = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    
                    "prob_mutate_link" => config.prob_mutate_link = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "prob_mutate_node" => config.prob_mutate_node = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "prob_mutate_weight_shift" => config.prob_mutate_weight_shift = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "prob_mutate_weight_random" => config.prob_mutate_weight_random = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "prob_mutate_toggle_link" => config.prob_mutate_toggle_link = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    
                    "species_threshold" => config.species_threshold = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "kill_percentage" => config.kill_percentage = split.next().expect("No number after parameter").parse().expect("No valid float supplied"),
                    "" => { },
                    _ => panic!("No recognized pattern")
                }
            }
        }

        config
    }

    pub fn init_zero() -> Self {
        Self {
            mult_disjoint: 0.0,
            mult_excess: 0.0,
            mult_weight_diff: 0.0,
            
            weight_shift_strength: 0.0,
            weight_random_strength: 0.0,
            
            prob_mutate_link: 0.0,
            prob_mutate_node: 0.0,
            prob_mutate_weight_shift: 0.0,
            prob_mutate_weight_random: 0.0,
            prob_mutate_toggle_link: 0.0,
            
            species_threshold: 0.0,
            
            kill_percentage: 0.0,
        }
    }

    pub fn global() -> &'static Self {
        CONFIG.get().expect("config is not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn from_file() {
        Config::from_file("tests/config.txt");

        assert_eq!(Config::global().mult_disjoint, 3.0);
        assert_eq!(Config::global().mult_excess, 2.0);
        assert_eq!(Config::global().mult_weight_diff, 4.0);

        assert_eq!(Config::global().weight_shift_strength, 0.3);
        assert_eq!(Config::global().weight_random_strength, 1.0);

        assert_eq!(Config::global().prob_mutate_link, 0.01);
        assert_eq!(Config::global().prob_mutate_node, 0.003);
        assert_eq!(Config::global().prob_mutate_weight_shift, 0.002);
        assert_eq!(Config::global().prob_mutate_weight_random, 0.002);
        assert_eq!(Config::global().prob_mutate_toggle_link, 0.0);

        assert_eq!(Config::global().species_threshold, 4.0);

        assert_eq!(Config::global().kill_percentage, 0.2);
    }
}
