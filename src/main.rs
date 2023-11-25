mod generate;
mod routes;
mod models;

use std::fs;
use std::sync::Arc;
use llm::{ModelParameters};
use crate::models::Config;

//Enable Rocket
#[macro_use] extern crate rocket;

#[launch]
async fn rocket() -> _ {
    let config_data = fs::read_to_string("config.json").unwrap_or_else(|_| "{}".to_string());
    let config: Config = serde_json::from_str(&config_data).expect("failed to parse and/or assign default Json and config");

    println!("Models: {:?}", config.models);
    println!("Port: {}", config.port);
    println!("Use GPU: {}", config.use_gpu);
    println!("Setting up llama..");
    let mut mp = ModelParameters::default();
    mp.use_gpu = true;
    mp.gpu_layers = 16.into();
    let llama = llm::load::<llm::models::Llama>(
        std::path::Path::new("./models/llama.bin"),
        llm::TokenizerSource::Embedded,
        mp.clone(),
        llm::load_progress_callback_stdout
    )
        .unwrap_or_else(|err| panic!("Failed to load model: {err}"));
    let stated_llama = Arc::new(llama);
    rocket::build()
        .manage(stated_llama.clone())
        .mount("/api", routes![routes::gen, routes::ping])
}

