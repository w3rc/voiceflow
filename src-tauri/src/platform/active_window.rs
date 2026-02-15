pub struct WindowInfo {
    pub title: String,
    pub process_name: String,
}

pub fn get_active_window() -> Result<WindowInfo, String> {
    let active = x_win::get_active_window();

    match active {
        Ok(window) => Ok(WindowInfo {
            title: window.title,
            process_name: window.info.exec_name,
        }),
        Err(e) => Err(format!("Failed to get active window: {:?}", e)),
    }
}

/// Classify the active window context for GPT tone matching.
pub fn classify_context(info: &WindowInfo) -> String {
    let process = info.process_name.to_lowercase();
    let title = info.title.to_lowercase();

    if process.contains("slack") || process.contains("discord") || process.contains("telegram")
        || process.contains("whatsapp") || title.contains("slack") || title.contains("discord")
    {
        "casual chat/messaging — use informal, conversational tone".to_string()
    } else if process.contains("outlook") || process.contains("thunderbird")
        || process.contains("gmail") || title.contains("mail") || title.contains("email")
        || title.contains("compose")
    {
        "email — use professional, polished tone with proper salutations".to_string()
    } else if process.contains("code") || process.contains("vim") || process.contains("emacs")
        || process.contains("sublime") || process.contains("idea") || process.contains("studio")
    {
        "code editor — use technical, precise language; preserve code terminology".to_string()
    } else if process.contains("docs") || process.contains("word") || process.contains("notion")
        || process.contains("libreoffice")
    {
        "document editor — use clear, well-structured prose".to_string()
    } else if process.contains("terminal") || process.contains("konsole")
        || process.contains("alacritty") || process.contains("kitty")
    {
        "terminal — use concise, technical language".to_string()
    } else {
        "general text input — use clear, neutral tone".to_string()
    }
}
