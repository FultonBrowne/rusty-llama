extern crate serde;
extern crate serde_json;

use rocket::http::private::Array;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Status {
    status: i8
}

#[derive(Serialize, Deserialize)]
pub struct TokenJson {
    model: String,
    created_at: String,
    response: String,
    done: bool,
    context: Option<Vec<i32>>,
    total_duration: Option<i64>,
    load_duration: Option<i64>,
    prompt_eval_count: Option<i32>,
    prompt_eval_duration: Option<i64>,
    eval_count: Option<i32>,
    eval_duration: Option<i64>
}

#[derive(Serialize, Deserialize)]
pub struct GenerateIngest {
    model: String,
    prompt: String
    //TODO Support more params
}
