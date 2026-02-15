use crate::audio::recorder::{samples_to_wav, Recorder};
use crate::state::AppState;
use std::sync::{Arc, Mutex};
use tauri::State;

pub struct RecorderHandle(pub Arc<Mutex<Recorder>>);

// SAFETY: Recorder contains cpal::Stream which is !Send, but we only
// access it through a Mutex and from the main thread context.
unsafe impl Send for RecorderHandle {}
unsafe impl Sync for RecorderHandle {}

#[tauri::command]
pub async fn start_recording(
    state: State<'_, AppState>,
    recorder: State<'_, RecorderHandle>,
) -> Result<(), String> {
    let recording_state = state.recording.clone();
    let device = {
        let config = state.config.lock().await;
        config.selected_mic.clone()
    };

    let mut rec = recorder
        .inner()
        .0
        .lock()
        .map_err(|e| format!("Recorder lock poisoned: {}", e))?;
    rec.start(recording_state, device.as_deref())?;

    Ok(())
}

#[tauri::command]
pub async fn stop_recording(
    state: State<'_, AppState>,
    recorder: State<'_, RecorderHandle>,
) -> Result<Vec<u8>, String> {
    {
        let mut rec = recorder
            .inner()
            .0
            .lock()
            .map_err(|e| format!("Recorder lock poisoned: {}", e))?;
        rec.stop();
    }

    let recording = state
        .recording
        .lock()
        .map_err(|e| format!("Recording lock poisoned: {}", e))?;

    if recording.audio_buffer.is_empty() {
        return Err("No audio recorded".to_string());
    }

    let wav_data = samples_to_wav(&recording.audio_buffer, recording.sample_rate)?;
    Ok(wav_data)
}

#[tauri::command]
pub async fn list_audio_devices() -> Result<Vec<String>, String> {
    Recorder::list_devices()
}
