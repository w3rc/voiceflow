use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;

#[derive(Default)]
pub struct RecordingState {
    pub is_recording: bool,
    pub audio_buffer: Vec<f32>,
    pub sample_rate: u32,
}

fn default_dictation_hotkey() -> String { "Ctrl+Alt+D".to_string() }
fn default_command_hotkey()   -> String { "Ctrl+Alt+C".to_string() }
fn default_toggle_hotkey()    -> String { "Ctrl+Alt+S".to_string() }

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AppConfig {
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

pub fn save_settings(data_dir: &Path, config: &AppConfig) {
    if let Ok(json) = serde_json::to_string_pretty(config) {
        let _ = std::fs::create_dir_all(data_dir);
        let _ = std::fs::write(data_dir.join("settings.json"), json);
    }
}

pub fn load_settings(data_dir: &Path) -> Option<AppConfig> {
    let json = std::fs::read_to_string(data_dir.join("settings.json")).ok()?;
    let mut cfg: AppConfig = serde_json::from_str(&json).ok()?;
    // Fill in any missing fields with defaults
    if cfg.dictation_hotkey.is_empty() { cfg.dictation_hotkey = default_dictation_hotkey(); }
    if cfg.command_hotkey.is_empty()   { cfg.command_hotkey   = default_command_hotkey(); }
    if cfg.toggle_hotkey.is_empty()    { cfg.toggle_hotkey    = default_toggle_hotkey(); }
    Some(cfg)
}

pub struct AppState {
    pub recording: Arc<std::sync::Mutex<RecordingState>>,
    pub config: Arc<tokio::sync::Mutex<AppConfig>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            recording: Arc::new(std::sync::Mutex::new(RecordingState {
                sample_rate: 16000,
                ..Default::default()
            })),
            config: Arc::new(tokio::sync::Mutex::new(AppConfig {
                dictation_hotkey: "Ctrl+Alt+D".to_string(),
                command_hotkey: "Ctrl+Alt+C".to_string(),
                toggle_hotkey: "Ctrl+Alt+S".to_string(),
                ..Default::default()
            })),
        }
    }
}
