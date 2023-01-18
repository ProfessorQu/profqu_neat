# profqu_neat

A crate that implements the NEAT algorithm
Created according to a [tutorial](https://www.youtube.com/playlist?list=PLgomWLYGNl1fcL0o4exBShNeCC5tc6s9C)
from [Finn Eggers](https://www.youtube.com/@finneggers6612).
I tried to implement NEAT from the official github [repository](https://github.com/f3270/NEAT),
but I couldn't figure out how to do it, so I used Finn's implementation.

Then I looked on Youtube and found Finn Eggers and his tutorial really helped me with creating this library.

Doesn't allow recurrent connections in the networks.

## Examples

```rust
use profqu_neat::Neat;

// Load config and create new Neat
Neat::load_config_from_file("src/config.txt");
let mut neat = Neat::new(10, 1, 1000);

// Create inputs
let input: Vec<f32> = vec![rand::random(); 10];

// Try to evolve the clients
for _iteration in 0..200 {
    for mut client in neat.iter_clients() {
        let fitness = client.calculate(&input)[0];
        client.fitness = fitness;
    }
    neat.evolve();
}

/// Get the best client
let best = neat.best_client().expect("Failed to get client");

/// Print all the data
neat.print_species();
println!("Best: {best:?}");

assert!(best.fitness > 0.8);
```
