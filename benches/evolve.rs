use criterion::{criterion_group, criterion_main, Criterion};
use profqu_neat::Neat;

fn test() {
    let inputs: Vec<f32> = vec![rand::random(); 10];

    Neat::load_config_from_file("tests/config.txt");
    let mut neat = Neat::new(10, 1, 100);
    
    for _iteration in 0..10 {
        for mut client in neat.iter_clients() {
            let fitness = 1.0 + client.calculate(inputs.clone())[0];

            client.fitness = fitness.into();
        }

        neat.evolve();
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("evolve", |b| b.iter(test));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);