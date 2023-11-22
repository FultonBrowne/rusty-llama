use std::io;
use std::io::Write;
use llama_cpp_rs::LLama;
use llama_cpp_rs::options::PredictOptions;

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