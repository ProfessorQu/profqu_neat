use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use profqu_neat::Neat;

fn test_evolve() {
    Neat::load_config_from_file("benches/config.txt");
    let mut neat = Neat::new(10, 1, 100);

    for _iteration in 0..10 {
        for mut client in neat.iter_clients() { client.fitness = 1.0; }
        neat.evolve();
    }
}

fn test_gen_species() {
    Neat::load_config_from_file("benches/config.txt");
    let mut neat = Neat::new(10, 1, 100);

    for _iteration in 0..10 {
        for mut client in neat.iter_clients() { client.fitness = 1.0; }
        neat.gen_species()
    }
}

fn test_kill() {
    Neat::load_config_from_file("benches/config.txt");
    let mut neat = Neat::new(10, 1, 100);

    for _iteration in 0..10 {
        for mut client in neat.iter_clients() { client.fitness = 1.0; }
        neat.kill()
    }
}

fn test_reproduce() {
    Neat::load_config_from_file("benches/config.txt");
    let mut neat = Neat::new(10, 1, 100);

    for _iteration in 0..10 {
        for mut client in neat.iter_clients() { client.fitness = 1.0; }
        neat.mutate()
    }
}

fn test_mutate() {
    Neat::load_config_from_file("benches/config.txt");
    let mut neat = Neat::new(10, 1, 100);

    for _iteration in 0..10 {
        for mut client in neat.iter_clients() { client.fitness = 1.0; }
        neat.mutate()
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("NEAT evolution");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function(
        "evolve",
        |b| b.iter(test_evolve)
    );

    group.bench_function(
        "gen_species",
        |b| b.iter(test_gen_species)
    );

    group.bench_function(
        "kill",
        |b| b.iter(test_kill)
    );
    group.bench_function(
        "reproduce",
        |b| b.iter(test_reproduce)
    );

    group.bench_function(
        "mutate",
        |b| b.iter(test_mutate)
    );

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);