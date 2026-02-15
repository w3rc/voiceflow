use enigo::{Enigo, Keyboard, Settings, Key, Direction};
use std::thread;
use std::time::Duration;

use super::clipboard;

/// Inject text into the currently focused application.
/// Strategy: save clipboard → write text to clipboard → Ctrl+V → restore clipboard.
pub fn inject_text(text: &str) -> Result<(), String> {
    // Save current clipboard contents
    let saved_clipboard = clipboard::read_clipboard().unwrap_or_default();

    // Write our text to clipboard
    clipboard::write_clipboard(text)?;

    // Small delay to ensure clipboard is ready
    thread::sleep(Duration::from_millis(50));

    // Simulate Ctrl+V
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to create Enigo instance: {}", e))?;

    enigo
        .key(Key::Control, Direction::Press)
        .map_err(|e| format!("Failed to press Ctrl: {}", e))?;
    enigo
        .key(Key::Unicode('v'), Direction::Click)
        .map_err(|e| format!("Failed to press V: {}", e))?;
    enigo
        .key(Key::Control, Direction::Release)
        .map_err(|e| format!("Failed to release Ctrl: {}", e))?;

    // Wait for paste to complete
    thread::sleep(Duration::from_millis(100));

    // Restore original clipboard
    if !saved_clipboard.is_empty() {
        let _ = clipboard::write_clipboard(&saved_clipboard);
    }

    Ok(())
}

/// Get currently selected text by simulating Ctrl+C and reading clipboard.
pub fn get_selected_text() -> Result<String, String> {
    let saved_clipboard = clipboard::read_clipboard().unwrap_or_default();

    // Clear clipboard first to detect if copy succeeds
    clipboard::write_clipboard("")?;
    thread::sleep(Duration::from_millis(50));

    // Simulate Ctrl+C
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to create Enigo instance: {}", e))?;

    enigo
        .key(Key::Control, Direction::Press)
        .map_err(|e| format!("Failed to press Ctrl: {}", e))?;
    enigo
        .key(Key::Unicode('c'), Direction::Click)
        .map_err(|e| format!("Failed to press C: {}", e))?;
    enigo
        .key(Key::Control, Direction::Release)
        .map_err(|e| format!("Failed to release Ctrl: {}", e))?;

    thread::sleep(Duration::from_millis(100));

    let selected = clipboard::read_clipboard().unwrap_or_default();

    // Restore original clipboard
    if !saved_clipboard.is_empty() {
        let _ = clipboard::write_clipboard(&saved_clipboard);
    }

    if selected.is_empty() {
        Err("No text selected".to_string())
    } else {
        Ok(selected)
    }
}
