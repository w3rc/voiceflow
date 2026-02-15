use crate::platform::active_window::{classify_context, get_active_window};
use serde::Serialize;

#[derive(Serialize)]
pub struct ContextInfo {
    pub window_title: String,
    pub process_name: String,
    pub context_description: String,
}

#[tauri::command]
pub async fn get_context() -> Result<ContextInfo, String> {
    let info = get_active_window()?;
    let context = classify_context(&info);

    Ok(ContextInfo {
        window_title: info.title,
        process_name: info.process_name,
        context_description: context,
    })
}
