use std::thread;
use std::sync::Arc;
use llm::models::Llama;
use rocket::response::stream::TextStream;
use crate::models::GenerateIngest;
use crate::generate;
use rocket::serde::json::Json;
use rocket::{State, tokio};
use rocket::tokio::sync::{mpsc, Mutex};
use serde_json::json;


#[get("/")]
pub fn ping() -> &'static str {
    "Llama is online"
}

#[post("/generate", format = "json", data = "<data>")]
pub fn gen(data: Json<GenerateIngest>, state: &State<Llama>) -> TextStream![String] {
    let llama = state.inner();
    let (tx, mut rx) = mpsc::channel(32);
   // tokio::spawn(async move {
        generate::llama_generate(data.prompt.clone(), llama, tx);
        // Drop the read_guard when done
   // });
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

