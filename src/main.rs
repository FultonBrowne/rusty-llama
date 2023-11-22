mod generate;
mod routes;
mod models;

use std::io;
use std::io::Write;
use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};

//Enable Rocket
#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    println!("Setting up llama..");
    let model_options = ModelOptions {
        n_gpu_layers: 12,
        ..Default::default()
    };

    let llama = LLama::new(
        "./models/llama.bin".into(),
        &model_options,
    )
        .unwrap();

    let mut input = String::new();
    loop {
        // Clear the previous input
        input.clear();

        // Prompt the user
        print!("You: ");
        io::stdout().flush().unwrap();

        // Read user input
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            println!("Goodbye!");
            break;
        }

        let llama_output = generate::llama_generate(
            input.clone(),
            llama.clone(),
            generate::gen_options()
        );
        println!("{}", llama_output); //TODO: switch this to a log based output
    }
    rocket::build().mount("/", routes![index])
}

