use std::{time::Duration, rc::Rc, cell::RefCell};

use criterion::{criterion_group, criterion_main, Criterion};
use profqu_neat::{Neat, genome::Genome, neat::Client};


fn test_crossover(vars: &mut (Neat, Genome, Genome)) {
    let mut neat = vars.0.clone();
    let genome1 = vars.1.clone();
    let genome2 = vars.2.clone();
    Genome::crossover(&mut neat, &genome1, &genome2);
}

fn test_calculate(client: &mut Rc<RefCell<Client>>) {
    let inputs = vec![rand::random(); 2];
    client.borrow_mut().calculate(inputs);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Genome mutation");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function(
        "crossover",
        |b| b.iter_batched_ref(
            || {
                Neat::load_config_from_file("benches/config.txt");
                let mut neat = Neat::new(10, 1, 100);
                let mut genome1 = neat.empty_genome();
                let mut genome2 = neat.empty_genome();
                
                for _iteration in 0..10 {
                    genome1.mutate(&mut neat);
                    genome2.mutate(&mut neat);
                }

                (neat, genome1, genome2)
            },
            test_crossover,
            criterion::BatchSize::SmallInput
        )
    );

    group.bench_function(
        "calculate",
        |b| b.iter_batched_ref(
            || {
                Neat::load_config_from_file("benches/config.txt");
                let mut neat = Neat::new(2, 1, 100);
                let client = neat.get_client(0);
                
                for _iteration in 0..10 {
                    client.borrow_mut().mutate(&mut neat);
                }
                
                client
            },
            test_calculate,
            criterion::BatchSize::SmallInput
        )
    );
    
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);