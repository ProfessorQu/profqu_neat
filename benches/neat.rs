use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use profqu_neat::Neat;

fn test_evolve(neat: &mut Neat) {
    for mut client in neat.iter_clients() {
        client.fitness = 1.0;
    }
    neat.evolve();
}

fn test_gen_species(neat: &mut Neat) {
    for mut client in neat.iter_clients() {
        client.fitness = 1.0;
    }
    neat.gen_species()
}

fn test_kill(neat: &mut Neat) {
    for mut client in neat.iter_clients() {
        client.fitness = 1.0;
    }
    neat.kill()
}

fn test_remove_extinct_species(neat: &mut Neat) {
    for mut client in neat.iter_clients() {
        client.fitness = 1.0;
    }
    neat.remove_extinct_species()
}

fn test_reproduce(neat: &mut Neat) {
    for mut client in neat.iter_clients() {
        client.fitness = 1.0;
    }
    neat.reproduce()
}

fn test_mutate(neat: &mut Neat) {
    for mut client in neat.iter_clients() {
        client.fitness = 1.0;
    }
    neat.mutate()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("NEAT evolution");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("evolve", |b| {
        b.iter_batched_ref(
            || {
                Neat::load_config_from_file("benches/config.txt");
                Neat::new(10, 1, 100)
            },
            test_evolve,
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("gen_species", |b| {
        b.iter_batched_ref(
            || {
                Neat::load_config_from_file("benches/config.txt");
                Neat::new(10, 1, 100)
            },
            test_gen_species,
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("kill", |b| {
        b.iter_batched_ref(
            || {
                Neat::load_config_from_file("benches/config.txt");
                Neat::new(10, 1, 100)
            },
            test_kill,
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("remove_extinct_species", |b| {
        b.iter_batched_ref(
            || {
                Neat::load_config_from_file("benches/config.txt");
                Neat::new(10, 1, 100)
            },
            test_remove_extinct_species,
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("reproduce", |b| {
        b.iter_batched_ref(
            || {
                Neat::load_config_from_file("benches/config.txt");
                let mut neat = Neat::new(10, 1, 100);
                neat.gen_species();
                neat
            },
            test_reproduce,
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("mutate", |b| {
        b.iter_batched_ref(
            || {
                Neat::load_config_from_file("benches/config.txt");
                Neat::new(10, 1, 100)
            },
            test_mutate,
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
