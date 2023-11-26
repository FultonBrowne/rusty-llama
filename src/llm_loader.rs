use std::collections::HashMap;
use std::sync::Arc;
use llm::ModelParameters;
use llm::models::Llama;
use crate::models::ModelDefinition;


fn get_model(model_definition: &ModelDefinition) -> Arc<Llama>{
    let mp = ModelParameters::default();
    let llama = llm::load::<Llama>(
        std::path::Path::new(&model_definition.path),
        llm::TokenizerSource::Embedded,
        mp,
        llm::load_progress_callback_stdout
    )
        .unwrap_or_else(|err| panic!("Failed to load model: {err}"));
    Arc::new(llama)
}
pub fn load_models(models_list: Vec<ModelDefinition>) -> HashMap<String, Arc<Llama>> {
    let mut models = HashMap::new();
    for i in models_list {
        let stated_llama = get_model(&i);
        models.insert(i.name, stated_llama);
    }

    models
}