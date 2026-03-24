use crate::state::app::AppState;
use ollama_rs::generation::chat::{request::ChatMessageRequest, ChatMessage};
use ollama_rs::history::ChatHistory;
use ollama_rs::{
    generation::completion::{request::GenerationRequest, GenerationContext},
    Ollama,
};
use tauri::AppHandle;
use tauri::Emitter;
use tauri::State;
use tokio_stream::StreamExt;

#[tauri::command]
pub fn is_ollama_installed(state: State<'_, AppState>) -> bool {
    state.ollama.ollama_client.is_some()
}

#[tauri::command]
pub async fn generate_stream(
    app: AppHandle,
    state: State<'_, AppState>,
    prompt: String,
    model: Option<String>,
) -> Result<(), String> {
    let client = match &state.ollama.ollama_client {
        Some(client) => client,
        None => {
            // optional: notify frontend
            let _ = app.emit("ollama://error", "Ollama is not running");
            return Err("Ollama is not available".into());
        }
    };

    let model = model
        .or_else(|| state.ollama.ollama_active_model.clone())
        .ok_or_else(|| "No model available".to_string())?;

    let mut context = state.context.lock().await.clone();

    let mut request = GenerationRequest::new(model, prompt);
    request.context = context.clone();

    let mut stream = client
        .generate_stream(request)
        .await
        .map_err(|e| e.to_string())?;

    while let Some(res) = stream.next().await {
        let res = res.map_err(|e| e.to_string())?;

        for ele in res {
            app.emit("ollama://stream", ele.response.clone())
                .map_err(|e| e.to_string())?;

            context = ele.context.or(context);
        }
    }

    *state.context.lock().await = context;

    Ok(())
}
