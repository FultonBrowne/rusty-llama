use std::sync::Arc;
use rocket::response::stream::TextStream;
use crate::models::GenerateIngest;
use crate::generate;
use rocket::serde::json::Json;
use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};
use rocket::State;
use rocket::tokio::sync::Mutex;


#[get("/")]
pub fn ping() -> &'static str {
    "Llama is online"
}

#[post("/generate", format = "json", data = "<data>")]
pub fn gen(data: Json<GenerateIngest>, state: &State<Arc<LLama>>) -> TextStream![String] {
    let llama = state;
    TextStream! {
        let model_options = ModelOptions {
        n_gpu_layers: 12,
        ..Default::default()
        };

        let predict_options = PredictOptions {
        tokens: 0,
        threads: 14,
        top_k: 90,
        top_p: 0.86,
        token_callback: Some(Box::new(|token| {
                print!("{}", token);
                //yield token;
                true
        })),
        ..Default::default()
        };
        //let llama_output = generate::llama_generate("Hello there".into(), llama.clone(), predict_options);
        yield "test\n".into();

    }
}

