
use flume::Sender;
use llm::{InferenceFeedback, InferenceStats, Model};
use llm::models::Llama;


pub fn llama_generate(input: String, llama: &Llama, tx:Sender<String>) -> InferenceStats {
    let mut session = llama.start_session(Default::default());
    let res = session.infer::<std::convert::Infallible>(
        llama,
        &mut rand::thread_rng(),
        // the prompt to use for text generation, as well as other
        // inference parameters
        &llm::InferenceRequest {
            prompt: (&input).into(),
            parameters: &Default::default(),
            play_back_previous_tokens: false,
            maximum_token_count: None,
        },
        &mut Default::default(),
         |r| match r {
            llm::InferenceResponse::PromptToken(t) | llm::InferenceResponse::InferredToken(t) => {
                tx.send(t).expect("Failed to send token to the main thread (llama_generate)");
                Ok(InferenceFeedback::Continue)
            }
            _ => Ok(InferenceFeedback::Continue),
        }
    ).expect("Llama generation gave an error (generate.rs");
    res
}
