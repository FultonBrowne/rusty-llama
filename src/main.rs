mod generate;
mod routes;
mod models;

use std::sync::Arc;
use llm::{ModelParameters};

//Enable Rocket
#[macro_use] extern crate rocket;

#[launch]
async fn rocket() -> _ {
    println!("Setting up llama..");
    let mut mp = ModelParameters::default();
    mp.use_gpu = true;
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

