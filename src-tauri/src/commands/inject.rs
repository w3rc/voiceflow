use crate::platform::text_inject;

#[tauri::command]
pub async fn inject_text(text: String) -> Result<(), String> {
    // Run on a blocking thread since enigo uses synchronous APIs
    tokio::task::spawn_blocking(move || text_inject::inject_text(&text))
        .await
        .map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
pub async fn get_selected_text() -> Result<String, String> {
    tokio::task::spawn_blocking(text_inject::get_selected_text)
        .await
        .map_err(|e| format!("Task join error: {}", e))?
}
