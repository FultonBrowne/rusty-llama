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
    pub context: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_duration: Option<u128>,
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
pub struct Query {
    pub model: String,
    pub prompt: String,
    pub stream: Option<bool>,
    pub options: Option<Options>,
}

#[derive(Serialize, Deserialize)]
pub struct Options {
    pub num_keep: Option<u32>,
    pub seed: Option<u32>,
    pub num_predict: Option<u32>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
    pub tfs_z: Option<f32>,
    pub typical_p: Option<f32>,
    pub repeat_last_n: Option<u32>,
    pub temperature: Option<f32>,
    pub repeat_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub mirostat: Option<u32>,
    pub mirostat_tau: Option<f32>,
    pub mirostat_eta: Option<f32>,
    pub penalize_newline: Option<bool>,
    pub stop: Option<Vec<String>>,
    pub numa: Option<bool>,
    pub num_ctx: Option<u32>,
    pub num_batch: Option<u32>,
    pub num_gqa: Option<u32>,
    pub num_gpu: Option<u32>,
    pub main_gpu: Option<u32>,
    pub low_vram: Option<bool>,
    pub f16_kv: Option<bool>,
    pub logits_all: Option<bool>,
    pub vocab_only: Option<bool>,
    pub use_mmap: Option<bool>,
    pub use_mlock: Option<bool>,
    pub embedding_only: Option<bool>,
    pub rope_frequency_base: Option<f32>,
    pub rope_frequency_scale: Option<f32>,
    pub num_thread: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_models")]
    pub models: Vec<String>,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_use_gpu")]
    pub use_gpu: bool,
}

fn default_models() -> Vec<String> {
    vec!["llama2".to_string()]
}

fn default_port() -> u16 {
    8000
}

fn default_use_gpu() -> bool {
    false
}