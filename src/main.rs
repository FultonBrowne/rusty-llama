mod generate;
mod routes;
mod models;
mod llama;

use std::io;
use std::io::Write;
use std::sync::Arc;
use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};
use rocket::tokio::sync::Mutex;
use crate::llama::LLamaWrapper;

//Enable Rocket
#[macro_use] extern crate rocket;

#[launch]
async fn rocket() -> _ {
    println!("Setting up llama..");
    let model_options = ModelOptions {
        n_gpu_layers: 12,
        ..Default::default()
    };

    let llama = LLamaWrapper::new(
        "./models/llama.bin".into(),
        model_options,
    ).expect("Failed to initialize LLama");

    //let mut input = String::new();
    rocket::build()
        .manage(Arc::new(llama))
        .mount("/api", routes![routes::gen, routes::ping])
}

