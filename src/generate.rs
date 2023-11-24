use std::io;
use std::io::Write;
use llama_cpp_rs::LLama;
use llama_cpp_rs::options::{ModelOptions, PredictOptions};
use tokio::sync::mpsc::Sender;

pub(crate) fn gen_options(tx:Sender<String>) -> PredictOptions {
    return PredictOptions {
        tokens: 0,
        threads: 14,
        top_k: 90,
        top_p: 0.86,
        token_callback: Some(Box::new(move |token| {
            let tx_clone = tx.clone();
            tokio::spawn(async move {
                tx_clone.send(token).await.expect("Failed to send token");
            });
            true
        })),
        ..Default::default()
    };
}

pub fn llama_generate(input: String, llama: LLama, predict_options: PredictOptions) -> String{
    return llama.predict(
            input.clone(),
            predict_options,
        )
        .unwrap();
}

pub fn gen_test(input: String, predict_options: PredictOptions){
    let model_options = ModelOptions {
        n_gpu_layers: 12,
        ..Default::default()
    };
    let llama = LLama::new(
        "./models/llama.bin".into(),
        &model_options,
    ).unwrap();
    llama_generate(input, llama, predict_options);
}