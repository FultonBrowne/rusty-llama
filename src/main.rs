use std::io;
use std::io::Write;
use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};

fn gen_options() -> PredictOptions {
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
    return predict_options;
}
fn main() {
    println!("Setting up llama..");
    let model_options = ModelOptions {
        n_gpu_layers: 12,
        ..Default::default()
    };

    let llama = LLama::new(
        "./models/llama.bin".into(),
        &model_options,
    )
        .unwrap();

    let mut input = String::new();
    loop {
        // Clear the previous input
        input.clear();

        // Prompt the user
        print!("You: ");
        io::stdout().flush().unwrap();

        // Read user input
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            println!("Goodbye!");
            break;
        }

        llama
            .predict(
                input.clone(),
                gen_options(),
            )
            .unwrap();
    }
}

