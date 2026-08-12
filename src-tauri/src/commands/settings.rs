use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};

fn default_dictation_hotkey() -> String { "Ctrl+Alt+D".to_string() }
fn default_command_hotkey()   -> String { "Ctrl+Alt+C".to_string() }
fn default_toggle_hotkey()    -> String { "Ctrl+Alt+S".to_string() }


#[derive(Serialize, Deserialize, Clone)]
pub struct SettingsData {
    #[serde(default)]
    pub openai_api_key: String,
    #[serde(default = "default_dictation_hotkey")]
    pub dictation_hotkey: String,
    #[serde(default = "default_command_hotkey")]
    pub command_hotkey: String,
    #[serde(default = "default_toggle_hotkey")]
    pub toggle_hotkey: String,
    #[serde(default)]
    pub selected_mic: Option<String>,
    #[serde(default)]
    pub personal_dictionary: Vec<String>,
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<SettingsData, String> {
    let config = state.config.lock().await;
    Ok(SettingsData {
        openai_api_key: config.openai_api_key.clone(),
        dictation_hotkey: config.dictation_hotkey.clone(),
        command_hotkey: config.command_hotkey.clone(),
        toggle_hotkey: config.toggle_hotkey.clone(),
        selected_mic: config.selected_mic.clone(),
        personal_dictionary: config.personal_dictionary.clone(),
    })
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    settings: SettingsData,
) -> Result<(), String> {
    let mut config = state.config.lock().await;
    config.openai_api_key = settings.openai_api_key;
    config.dictation_hotkey = settings.dictation_hotkey;
    config.command_hotkey = settings.command_hotkey;
    config.toggle_hotkey = settings.toggle_hotkey;
    config.selected_mic = settings.selected_mic;
    config.personal_dictionary = settings.personal_dictionary;

    // Persist to disk immediately
    if let Ok(data_dir) = app.path().app_data_dir() {
        crate::state::save_settings(&data_dir, &config);
    }
    Ok(())
}
