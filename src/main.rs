use std::io;
use std::io::Write;
use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};

fn main() {
    let model_options = ModelOptions {
        n_gpu_layers: 12,
        ..Default::default()
    };

    let llama = LLama::new(
        "./models/llama.bin".into(),
        &model_options,
    )
        .unwrap();

    let predict_options = PredictOptions {
        tokens: 0,
        threads: 14,
        top_k: 90,
        top_p: 0.86,
        token_callback: Some(Box::new(|token| {
            print!("{}", token);
            io::stdout().flush().expect("If your seeing this reconsider life choices");
            true
        })),
        ..Default::default()
    };


    llama
        .predict(
            "what are the national animals of india".into(),
            predict_options,
        )
        .unwrap();
}
