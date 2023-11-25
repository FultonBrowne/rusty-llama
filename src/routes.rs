use std::sync::Arc;
use std::time::Instant;

use chrono::prelude::*;

use llm::models::Llama;
use rocket::{State, tokio};
use rocket::response::stream::TextStream;
use rocket::serde::json::Json;
use serde_json::json;

use crate::generate;
use crate::models::{GenerateIngest, TokenJson};

fn time_string() -> String {
    let now = Utc::now();
    now.format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string()
}
#[get("/")]
pub fn ping() -> &'static str {
    "Llama is online"
}

#[post("/generate", format = "json", data = "<data>")]
pub async fn gen(data: Json<GenerateIngest>, state: &State<Arc<Llama>>) -> TextStream![String] {
    let start = Instant::now();
    let (tx, rx) = flume::unbounded();
    let cloned_state = state.inner().clone();
    let t = tokio::spawn(async move {
        let llama = cloned_state;
        generate::llama_generate(data.prompt.clone(), &llama, tx)
   });
    let load_duration = start.elapsed();
    TextStream! {
        while let Ok(token) = rx.recv() {
            let j = json!(TokenJson{
                model: "TODO".to_string(),
                created_at: time_string(),
                done: false,
                response: token,
                context: None,
                total_duration: None,
                load_duration: None,
                prompt_eval_count: None,
                prompt_eval_duration: None,
                eval_count: None,
                eval_duration: None
            }).to_string() + "\n";
            println!("{}", j);
            yield j;
        }
        // wrap it all up
        let stats = t.await.expect("Llama thread failed to await");
        let j = json!(TokenJson{
                model: "TODO".to_string(),
                created_at: time_string(),
                done: true,
                response: String::from(""),
                context: None,
                total_duration: start.elapsed().as_nanos().into(),
                load_duration: load_duration.as_nanos().into(),
                prompt_eval_count: stats.prompt_tokens.into(),
                prompt_eval_duration: stats.feed_prompt_duration.as_nanos().into(),
                eval_count: stats.predict_tokens.into(),
                eval_duration: stats.predict_duration.as_nanos().into()
            }).to_string() + "\n";
        yield j;
    }
}

