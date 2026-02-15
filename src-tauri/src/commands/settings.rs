use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize, Clone)]
pub struct SettingsData {
    pub openai_api_key: String,
    pub dictation_hotkey: String,
    pub command_hotkey: String,
    pub selected_mic: Option<String>,
    pub personal_dictionary: Vec<String>,
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<SettingsData, String> {
    let config = state.config.lock().await;
    Ok(SettingsData {
        openai_api_key: config.openai_api_key.clone(),
        dictation_hotkey: config.dictation_hotkey.clone(),
        command_hotkey: config.command_hotkey.clone(),
        selected_mic: config.selected_mic.clone(),
        personal_dictionary: config.personal_dictionary.clone(),
    })
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    settings: SettingsData,
) -> Result<(), String> {
    let mut config = state.config.lock().await;
    config.openai_api_key = settings.openai_api_key;
    config.dictation_hotkey = settings.dictation_hotkey;
    config.command_hotkey = settings.command_hotkey;
    config.selected_mic = settings.selected_mic;
    config.personal_dictionary = settings.personal_dictionary;
    Ok(())
}
