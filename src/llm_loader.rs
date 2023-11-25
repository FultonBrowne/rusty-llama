use std::collections::HashMap;
use std::sync::Arc;
use llm::ModelParameters;
use llm::models::Llama;

pub fn load_models(models_list: Vec<String>, use_gpu: bool) -> HashMap<String, Arc<Llama>> {
    let mut models = HashMap::new();
    for i in models_list {
        let mut mp = ModelParameters::default();
        mp.use_gpu = use_gpu;
        mp.gpu_layers = 16.into();
        let llama = llm::load::<Llama>(
            std::path::Path::new("./models/llama.bin"),
            llm::TokenizerSource::Embedded,
            mp.clone(),
            llm::load_progress_callback_stdout
        ) // we need to build something that works with ollama's model loader
            .unwrap_or_else(|err| panic!("Failed to load model: {err}"));
        let stated_llama = Arc::new(llama);
        models.insert(i, stated_llama);

    }

    models
}