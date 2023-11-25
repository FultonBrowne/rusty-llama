use std::io;
use std::io::Write;
use std::sync::Arc;
use flume::Sender;
use llm::{InferenceFeedback, Model, Prompt};
use llm::models::Llama;
use rocket::futures::task::SpawnExt;

/*
pub(crate) fn gen_options(tx:Sender<String>) -> PredictOptions {
    return PredictOptions {
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
 */

pub fn llama_generate(input: String, llama: &Llama, tx:Sender<String>) {
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
    );
}
