use profqu_neat::Neat;

#[test]
fn max() {
    let inputs: Vec<f32> = vec![rand::random(); 10];

    Neat::load_config_from_file("tests/config.txt");
    let mut neat = Neat::new(10, 1, 100);
    
    for _iteration in 0..100 {
        for mut client in neat.iter_clients() {
            let fitness = 1.0 + client.calculate(inputs.clone())[0];

            client.fitness = fitness.into();
        }

        neat.evolve();
    }
    
    neat.print_species();
    let best = neat.best_client().expect("No clients");
    println!("Best: {:?}", best);
}

#[test]
#[ignore = "takes too long"]
fn xor_test() {
    let inputs: Vec<Vec<f32>> = vec![
        vec![0.0, 0.0],
        vec![1.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 1.0]
    ];
    let outputs: Vec<f32> = inputs.iter().map(|input| (input[0] as i64 ^ input[1] as i64) as f32).collect();

    Neat::load_config_from_file("tests/config.txt");
    let mut neat = Neat::new(2, 1, 300);

    for _iteration in 0..100 {
        for mut client in neat.iter_clients() {
            let mut fitness = 0.0;

            for index in 0..inputs.len() {
                let result = client.calculate(inputs[index].clone())[0];
                if (result >= 0.5 && outputs[index] == 1.0) || (result < 0.5 && outputs[index] == 0.0) {
                    if inputs[index][0] == 0.0 && inputs[index][1] == 0.0 {
                        fitness += 1.0;
                    }
                    else {
                        fitness += 1.02;
                    }
                }
            }

            client.fitness = fitness.into();
        }

        neat.evolve();
    }

    neat.print_species();
    let mut best = neat.best_client().expect("No clients");
    println!("Best: {:?}", best);

    let mut wrong = 0;
    for i in 0..inputs.len() {
        let mut result = best.calculate(inputs[i].clone())[0];
        println!("{:.0?}: {:.5?}\t\ttrue value: {:.0?}", inputs[i], result, outputs[i]);
        result = if result > 0.5 {
            1.0
        }
        else {
            0.0
        };

        if result != outputs[i] {
            wrong += 1;
        }
    }

    assert!(wrong <= 1);
}