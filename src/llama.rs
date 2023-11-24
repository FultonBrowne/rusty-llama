use std::sync::RwLock;
use llama_cpp_rs::LLama;
use llama_cpp_rs::options::ModelOptions;

pub struct LLamaWrapper {
    pub(crate) llama: RwLock<LLama>,
}

impl LLamaWrapper {
    pub fn new(model: String, model_options: ModelOptions) -> Result<Self, String> {
        match LLama::new(model, &model_options) {
            Ok(llama) => Ok(LLamaWrapper { llama: RwLock::new(llama) }),
            Err(e) => Err(format!("Failed to initialize LLama: {}", e)),
        }
    }
    pub fn safe_read<F, R>(&self, f: F) -> Result<R, String>
        where
            F: FnOnce(&LLama) -> R,
    {
        let llama_guard = self.llama.read().map_err(|e| e.to_string())?;
        let llama_ref = &*llama_guard; // Dereference the guard to get a reference to LLama
        Ok(f(llama_ref))
    }
}

// TODO: test the living hell out of stuff cause unsafe code I don't want
unsafe impl Send for LLamaWrapper {}
unsafe impl Sync for LLamaWrapper {}