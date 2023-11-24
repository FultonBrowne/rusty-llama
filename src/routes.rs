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
use crate::llama::LLamaWrapper;


#[get("/")]
pub fn ping() -> &'static str {
    "Llama is online"
}

#[post("/generate", format = "json", data = "<data>")]
pub fn gen(data: Json<GenerateIngest>, state: &State<Arc<LLamaWrapper>>) -> TextStream![String] {
    let (tx, mut rx) = mpsc::channel(32);
    let predict_options = gen_options(tx);
    let llama_wrapper_clone = Arc::clone(state.inner());
    tokio::spawn(async move {
        let read_guard = llama_wrapper_clone.llama.read().unwrap();
        generate::llama_generate(data.prompt.clone(), &*read_guard, predict_options)
        // Drop the read_guard when done
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

