use profqu_neat::Neat;

#[test]
fn test() {
    let inputs: Vec<Vec<f32>> = vec![
        vec![0.0, 0.0],
        vec![1.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 1.0]
    ];
    let outputs: Vec<f32> = inputs.iter().map(|input| (input[0] as i64 ^ input[1] as i64) as f32).collect();

    let mut neat = Neat::new(2, 1, 10);

    Neat::load_config_from_file("tests/config.txt");

    for _iteration in 0..2 {
        for mut client in neat.iter_clients() {
            let mut fitness = 4.0;

            for index in 0..inputs.len() {
                let result = client.calculate(inputs[index].clone())[0];

                fitness -= (result - outputs[index]).powf(2.0);
                
                println!("{:?}: {:?}\tgenome: {:?}", inputs[index], outputs[index], result);
            }
            
            println!("fitness: {:?}", fitness);

            client.fitness = fitness.into();
        }

        neat.evolve();
    }

    neat.print_species();
    let mut best = neat.best_client().expect("No clients");
    println!("Best: {:?}", best);

    for input in inputs {
        println!("{:?}: {:?}", input, best.calculate(input.clone()));
    }
}