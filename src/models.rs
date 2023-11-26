extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

/* So by models I meant data models -- Silly me */
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

fn default_port() -> u16 {
    8000
}

fn default_use_gpu() -> bool {
    false
}

#[derive(Serialize, Deserialize)]
pub struct ModelInfoQuery{
    pub name: String
}
#[derive(Serialize, Deserialize)]
pub struct ModelInfoJson{
    pub license: String,
    pub modelfile: String,
    pub parameters: String,
    pub template:String
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub models: Vec<ModelDefinition>,
}

/* The model config and definition all based of Ollama's Modelfile so we can fully support them */
#[derive(Serialize, Deserialize)]
pub struct ModelConfig {
    pub mirostat: Option<i32>,       // default: 0
    pub mirostat_eta: Option<f32>,   // default: 0.1
    pub mirostat_tau: Option<f32>,   // default: 5.0
    pub num_ctx: Option<i32>,        // default: 2048
    pub num_gqa: Option<i32>,        // no default specified
    pub num_gpu: Option<i32>,        // no default specified
    pub num_thread: Option<i32>,     // no default specified
    pub repeat_last_n: Option<i32>,  // default: 64
    pub repeat_penalty: Option<f32>, // default: 1.1
    pub temperature: Option<f32>,    // default: 0.8
    pub seed: Option<i32>,           // default: 0
    pub stop: Option<String>,        // no default specified
    pub tfs_z: Option<f32>,          // default: 1
    pub num_predict: Option<i32>,    // default: 128
    pub top_k: Option<i32>,          // default: 40
    pub top_p: Option<f32>,          // default: 0.9
}

impl Default for ModelConfig {
    fn default() -> Self {
        ModelConfig {
            mirostat: Some(0),
            mirostat_eta: Some(0.1),
            mirostat_tau: Some(5.0),
            num_ctx: Some(2048),
            num_gqa: None,
            num_gpu: None,
            num_thread: None,
            repeat_last_n: Some(64),
            repeat_penalty: Some(1.1),
            temperature: Some(0.8),
            seed: Some(0),
            stop: None,
            tfs_z: Some(1.0),
            num_predict: Some(128),
            top_k: Some(40),
            top_p: Some(0.9),
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct ModelDefinition {
    pub path: String,
    pub name: String,
    pub config: ModelConfig
}
