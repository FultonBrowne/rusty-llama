mod generate;
mod routes;
mod models;

use std::io;
use std::io::Write;
use std::sync::Arc;
use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};
use rocket::tokio::sync::Mutex;

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

    //let mut input = String::new();
    rocket::build()
        .manage(Arc::new(llama.clone()))
        .mount("/api", routes![routes::gen, routes::ping])
}

