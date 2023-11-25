extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Status {
    status: i8
}

#[derive(Serialize, Deserialize)]
pub struct TokenJson {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
    pub context: Option<Vec<i32>>,
    pub total_duration: Option<i64>,
    pub load_duration: Option<i64>,
    pub prompt_eval_count: Option<i32>,
    pub prompt_eval_duration: Option<i64>,
    pub eval_count: Option<i32>,
    pub eval_duration: Option<i64>
}

#[derive(Serialize, Deserialize)]
pub struct GenerateIngest {
    pub model: String,
    pub prompt: String
    //TODO Support more params
}
