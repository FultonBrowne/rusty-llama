mod generate;
mod routes;
mod models;

use std::io;
use std::io::Write;
use std::sync::Arc;
use rocket::tokio::sync::Mutex;
use llm::{Model, ModelParameters};

//Enable Rocket
#[macro_use] extern crate rocket;

#[launch]
async fn rocket() -> _ {
    println!("Setting up llama..");
    let mut mp = ModelParameters::default();
    mp.use_gpu = true;
    let llama = llm::load::<llm::models::Llama>(
        // path to GGML file
        std::path::Path::new("./models/llama.bin"),
        // llm::TokenizerSource
        llm::TokenizerSource::Embedded,
        // llm::ModelParameters
        mp.clone(),
        // load progress callback
        llm::load_progress_callback_stdout
    )
        .unwrap_or_else(|err| panic!("Failed to load model: {err}"));

    //let mut input = String::new();
    rocket::build()
        .manage(llama)
        .mount("/api", routes![routes::gen, routes::ping])
}

