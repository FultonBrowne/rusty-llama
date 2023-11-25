use std::os::unix::fs::symlink;
use std::sync::Arc;
use std::thread;

use llm::models::Llama;
use rocket::response::stream::TextStream;
use crate::models::GenerateIngest;
use crate::generate;
use rocket::serde::json::Json;
use rocket::{State, tokio};
use serde_json::json;


#[get("/")]
pub fn ping() -> &'static str {
    "Llama is online"
}

#[post("/generate", format = "json", data = "<data>")]
pub async fn gen(data: Json<GenerateIngest>, state: &State<Arc<Llama>>) -> TextStream![String] {
    let (tx, mut rx) = flume::unbounded();
    let cloned_state = state.inner().clone();
    let t = tokio::spawn(async move {
        let llama = cloned_state;
        generate::llama_generate(data.prompt.clone(), &llama, tx);
        // Drop the read_guard when done
   });
    TextStream! {
        //let llama_output = generate::llama_generate("Hello there".into(), llama.clone(), predict_options);
        while let Ok(token) = rx.recv() {
            let j = json!({"message": token, "done": false}).to_string() + "\n";
            println!("{}", j);
            yield j;
        }
        t.await.expect("hehehe");
    }
}

