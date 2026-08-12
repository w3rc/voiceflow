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
    // x-win can panic on Wayland if the GNOME Shell extension is missing.
    // Run in a blocking thread and catch any panic so the app keeps running.
    let result = tokio::task::spawn_blocking(|| {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(get_active_window))
    })
    .await
    .map_err(|e| format!("Task error: {e:?}"))?;

    match result {
        Ok(Ok(info)) => {
            let context = classify_context(&info);
            Ok(ContextInfo {
                window_title: info.title,
                process_name: info.process_name,
                context_description: context,
            })
        }
        Ok(Err(_)) | Err(_) => Err("Could not detect active window".to_string()),
    }
}
