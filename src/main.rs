mod generate;
mod routes;
mod models;
mod llm_loader;

use std::fs;

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
    let llamas = llm_loader::load_models(config.models, config.use_gpu);
    rocket::build()
        .manage(llamas)
        .mount("/api", routes![routes::gen, routes::model_info, routes::ping])
}

