use std::io;
use std::io::Write;
use llama_cpp_rs::LLama;
use llama_cpp_rs::options::{ModelOptions, PredictOptions};

pub(crate) fn gen_options() -> PredictOptions {
    let predict_options = PredictOptions {
        tokens: 0,
        threads: 14,
        top_k: 90,
        top_p: 0.86,
        token_callback: Some(Box::new(|token| {
            print!("{}", token);
            io::stdout().flush().expect("If you're seeing this reconsider life choices");
            true
        })),
        ..Default::default()
    };
    return predict_options;
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