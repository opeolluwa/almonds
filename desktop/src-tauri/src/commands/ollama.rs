use ollama_rs::generation::completion::request::GenerationRequest;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::State;
use tokio_stream::StreamExt;

use crate::state::app::AppState;

pub const AI_MODEL: &str = "phi3:mini";

#[derive(serde::Serialize, Clone)]
pub struct PullProgress {
    pub status: String,
    pub total: Option<u64>,
    pub completed: Option<u64>,
}

#[tauri::command]
pub fn is_ollama_installed(state: State<'_, AppState>) -> bool {
    state.ollama.ollama_client.is_some()
}

#[tauri::command]
pub async fn check_ai_model(state: State<'_, AppState>) -> Result<bool, String> {
    let client = match &state.ollama.ollama_client {
        Some(c) => c,
        None => return Ok(false),
    };
    match client.list_local_models().await {
        Ok(models) => Ok(models.iter().any(|m| m.name.starts_with(AI_MODEL))),
        Err(_) => Ok(false),
    }
}

#[tauri::command]
pub async fn pull_ai_model(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let client = match &state.ollama.ollama_client {
        Some(c) => c,
        None => return Err("AI service is not available".into()),
    };

    let mut stream = client
        .pull_model_stream(AI_MODEL.to_string(), false)
        .await
        .map_err(|e| e.to_string())?;

    while let Some(status) = stream.next().await {
        let status = status.map_err(|e| e.to_string())?;
        let _ = app.emit(
            "ai://pull-progress",
            PullProgress {
                status: status.message,
                total: status.total,
                completed: status.completed,
            },
        );
    }

    Ok(())
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
