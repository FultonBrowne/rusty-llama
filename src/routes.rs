use std::thread;
use std::sync::Arc;
use llama_cpp_rs::options::PredictOptions;
use rocket::response::stream::TextStream;
use crate::models::GenerateIngest;
use crate::generate;
use rocket::serde::json::Json;
use rocket::{State, tokio};
use rocket::tokio::sync::{mpsc, Mutex};
use serde_json::json;
use crate::generate::gen_options;


#[get("/")]
pub fn ping() -> &'static str {
    "Llama is online"
}

#[post("/generate", format = "json", data = "<data>")]
pub fn gen(data: Json<GenerateIngest>) -> TextStream![String] { // , state: &State<Arc<LLama>>
    let (tx, mut rx) = mpsc::channel(32);
    let predict_options = gen_options(tx);
    tokio::spawn(async move {
        generate::gen_test(data.prompt.clone(), predict_options);
    });
    TextStream! {
        //let llama_output = generate::llama_generate("Hello there".into(), llama.clone(), predict_options);
        yield "test\n".into();
        while let Some(token) = rx.recv().await {
            let j = json!({"message": token, "done": false}).to_string() + "\n";
            println!("{}", j);
            yield j;
        }
        yield "test\n".into();

    }
}

