use crate::api::gpt;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn cleanup_transcript(
    state: State<'_, AppState>,
    raw_text: String,
    context: String,
) -> Result<String, String> {
    let config = state.config.lock().await;

    if config.openai_api_key.is_empty() {
        return Err("OpenAI API key not configured".to_string());
    }

    gpt::cleanup_text(&config.openai_api_key, &raw_text, &context).await
}

#[tauri::command]
pub async fn execute_voice_command(
    state: State<'_, AppState>,
    selected_text: String,
    command: String,
) -> Result<String, String> {
    let config = state.config.lock().await;

    if config.openai_api_key.is_empty() {
        return Err("OpenAI API key not configured".to_string());
    }

    gpt::execute_command(&config.openai_api_key, &selected_text, &command).await
}
