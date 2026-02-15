use std::sync::Arc;

#[derive(Default)]
pub struct RecordingState {
    pub is_recording: bool,
    pub audio_buffer: Vec<f32>,
    pub sample_rate: u32,
}

#[derive(Default)]
pub struct AppConfig {
    pub openai_api_key: String,
    pub dictation_hotkey: String,
    pub command_hotkey: String,
    pub selected_mic: Option<String>,
    pub personal_dictionary: Vec<String>,
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
                ..Default::default()
            })),
        }
    }
}
