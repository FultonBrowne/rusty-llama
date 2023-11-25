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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_eval_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_eval_duration: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_duration: Option<u128>
}

#[derive(Serialize, Deserialize)]
pub struct GenerateIngest {
    pub model: String,
    pub prompt: String
    //TODO Support more params
}
