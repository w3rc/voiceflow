use crate::api::whisper;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn transcribe_audio(
    state: State<'_, AppState>,
    wav_data: Vec<u8>,
) -> Result<String, String> {
    let config = state.config.lock().await;

    if config.openai_api_key.is_empty() {
        return Err("OpenAI API key not configured. Please set it in Settings.".to_string());
    }

    let prompt = if config.personal_dictionary.is_empty() {
        None
    } else {
        Some(config.personal_dictionary.join(", "))
    };

    whisper::transcribe(
        &config.openai_api_key,
        wav_data,
        prompt.as_deref(),
    )
    .await
}
