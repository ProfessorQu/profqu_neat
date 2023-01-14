use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use profqu_neat::{Neat, genome::Genome};


fn test_mutate_link() {
    Neat::load_config_from_file("benches/config.txt");
    let mut neat = Neat::new(10, 1, 100);
    let mut genome = neat.empty_genome();

    for _iteration in 0..10 {
        genome.mutate_link(&mut neat);
    }
}

fn test_mutate_node() {
    Neat::load_config_from_file("benches/config.txt");
    let mut neat = Neat::new(10, 1, 100);
    let mut genome = neat.empty_genome();

    for _iteration in 0..10 {
        genome.mutate_node(&mut neat);
    }
}

fn test_mutate_weight_shift() {
    Neat::load_config_from_file("benches/config.txt");
    let mut neat = Neat::new(10, 1, 100);
    let mut genome = neat.empty_genome();

    for _iteration in 0..10 {
        genome.mutate_weight_shift();
    }
}

fn test_mutate_weight_random() {
    Neat::load_config_from_file("benches/config.txt");
    let mut neat = Neat::new(10, 1, 100);
    let mut genome = neat.empty_genome();

    for _iteration in 0..10 {
        genome.mutate_weight_random();
    }
}

fn test_mutate_link_toggle() {
    Neat::load_config_from_file("benches/config.txt");
    let mut neat = Neat::new(10, 1, 100);
    let mut genome = neat.empty_genome();

    for _iteration in 0..10 {
        genome.mutate_link_toggle();
    }
}

fn test_crossover() {
    Neat::load_config_from_file("benches/config.txt");
    let mut neat = Neat::new(10, 1, 100);
    let mut genome1 = neat.empty_genome();
    let mut genome2 = neat.empty_genome();

    for _iteration in 0..2 {
        genome1.mutate(&mut neat);
        genome2.mutate(&mut neat);
    }

    for _iteration in 0..10 {
        Genome::crossover(&mut neat, &genome1, &genome2);
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Genome mutation");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function(
        "mutate_link",
        |b| b.iter(test_mutate_link)
    );

    group.bench_function(
        "mutate_node",
        |b| b.iter(test_mutate_node)
    );

    group.bench_function(
        "mutate_weight_shift",
        |b| b.iter(test_mutate_weight_shift)
    );

    group.bench_function(
        "mutate_weight_random",
        |b| b.iter(test_mutate_weight_random)
    );

    group.bench_function(
        "mutate_link_toggle",
        |b| b.iter(test_mutate_link_toggle)
    );

    group.bench_function(
        "crossover",
        |b| b.iter(test_crossover)
    );
    
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);