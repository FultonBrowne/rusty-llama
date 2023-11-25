use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use chrono::prelude::*;

use llm::models::Llama;
use rocket::{State, tokio};
use rocket::http::Status;
use rocket::response::stream::TextStream;
use rocket::serde::json::Json;
use serde_json::json;

use crate::generate;
use crate::models::{ModelInfoJson, ModelInfoQuery, Query, TokenJson};

fn time_string() -> String {
    let now = Utc::now();
    now.format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string()
}
#[get("/")]
pub fn ping() -> &'static str {
    "Llama is online"
}

#[post("/show", format = "json", data = "<data>")]
pub fn model_info(data: Json<ModelInfoQuery>, state: &State<HashMap<String, Arc<Llama>>>) -> Result<Json<ModelInfoJson>, Status> {
    // as of now this is just a place holder
    if state.inner().contains_key(&*data.name) {
        Ok(Json::from(ModelInfoJson { license: "".to_string(), modelfile: "".to_string(), parameters: "".to_string(), template: "".to_string() }))
    } else {
        Err(Status::NotFound)
    }

}

#[post("/generate", format = "json", data = "<data>")]
pub async fn gen(data: Json<Query>, state: &State<HashMap<String, Arc<Llama>>>) -> TextStream![String] {
    let start = Instant::now();
    let model = String::from(&data.model);
    let (tx, rx) = flume::unbounded();
    let cloned_state = state.inner().get(&model).expect("Model was not defined on application startup").clone();
    let t = tokio::spawn(async move {
        let llama = cloned_state;
        generate::llama_generate(data.prompt.clone(), &llama, tx)
   });
    let load_duration = start.elapsed();
    TextStream! {
        while let Ok(token) = rx.recv() {
            let j = json!(TokenJson{
                model: model.to_string(),
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
        let model_out = t.await.expect("Llama thread failed to await");
        let stats = model_out.interface_stats;
        let j = json!(TokenJson{
                model: model.to_string(),
                created_at: time_string(),
                done: true,
                response: String::from(""),
                context: model_out.context.into(),
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

