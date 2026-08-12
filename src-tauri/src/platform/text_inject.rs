use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::thread;
use std::time::Duration;

use super::clipboard;

/// Inject text into the currently focused application.
/// On Linux: uses wl-copy/xclip for clipboard (no arboard read hang),
/// then xdotool/enigo to simulate Ctrl+V.
pub fn inject_text(text: &str) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    return inject_linux(text);

    #[cfg(not(target_os = "linux"))]
    inject_default(text)
}

#[cfg(target_os = "linux")]
fn inject_linux(text: &str) -> Result<(), String> {
    // Write to clipboard without reading first (avoids Wayland clipboard-owner hang)
    set_clipboard_linux(text)?;
    thread::sleep(Duration::from_millis(100));
    simulate_paste_linux()
}

#[cfg(target_os = "linux")]
fn set_clipboard_linux(text: &str) -> Result<(), String> {
    // Try wl-copy (native Wayland, most reliable on GNOME/Wayland)
    if std::process::Command::new("wl-copy")
        .arg(text)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        return Ok(());
    }

    // Try xclip (X11 / XWayland)
    use std::io::Write;
    if let Ok(mut child) = std::process::Command::new("xclip")
        .args(["-selection", "clipboard"])
        .stdin(std::process::Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
        if child.wait().map(|s| s.success()).unwrap_or(false) {
            return Ok(());
        }
    }

    // Fallback: arboard write-only (no read, so no Wayland hang)
    arboard::Clipboard::new()
        .map_err(|e| format!("Clipboard error: {e}"))?
        .set_text(text.to_string())
        .map_err(|e| format!("Clipboard write error: {e}"))
}

#[cfg(target_os = "linux")]
fn simulate_paste_linux() -> Result<(), String> {
    // Try xdotool (works via XWayland on Wayland sessions)
    // --clearmodifiers ensures held modifier keys don't interfere
    if std::process::Command::new("xdotool")
        .args(["key", "--clearmodifiers", "ctrl+v"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        return Ok(());
    }

    // Fallback: enigo
    simulate_paste_enigo()
}

fn simulate_paste_enigo() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to create Enigo instance: {e}"))?;
    enigo.key(Key::Control, Direction::Press)
        .map_err(|e| format!("Failed to press Ctrl: {e}"))?;
    enigo.key(Key::Unicode('v'), Direction::Click)
        .map_err(|e| format!("Failed to press V: {e}"))?;
    enigo.key(Key::Control, Direction::Release)
        .map_err(|e| format!("Failed to release Ctrl: {e}"))?;
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn inject_default(text: &str) -> Result<(), String> {
    let saved_clipboard = clipboard::read_clipboard().unwrap_or_default();
    clipboard::write_clipboard(text)?;
    thread::sleep(Duration::from_millis(50));
    simulate_paste_enigo()?;
    thread::sleep(Duration::from_millis(100));
    if !saved_clipboard.is_empty() {
        let _ = clipboard::write_clipboard(&saved_clipboard);
    }
    Ok(())
}

/// Get currently selected text by simulating Ctrl+C and reading clipboard.
pub fn get_selected_text() -> Result<String, String> {
    #[cfg(target_os = "linux")]
    return get_selected_text_linux();

    #[cfg(not(target_os = "linux"))]
    get_selected_text_default()
}

#[cfg(target_os = "linux")]
fn wl_paste() -> String {
    std::process::Command::new("wl-paste")
        .arg("--no-newline")
        .output()
        .ok()
        .and_then(|o| if o.status.success() { String::from_utf8(o.stdout).ok() } else { None })
        .unwrap_or_default()
}

#[cfg(target_os = "linux")]
fn get_selected_text_linux() -> Result<String, String> {
    // Save current clipboard
    let saved = wl_paste();

    // Clear clipboard so we can detect if Ctrl+C copies anything
    let _ = std::process::Command::new("wl-copy").arg("").status();
    thread::sleep(Duration::from_millis(50));

    // Simulate Ctrl+C with xdotool (no portal dialog)
    let copied = std::process::Command::new("xdotool")
        .args(["key", "--clearmodifiers", "ctrl+c"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !copied {
        // xdotool not available, fall back to enigo (may trigger portal)
        simulate_copy_enigo()?;
    }

    thread::sleep(Duration::from_millis(150));

    let selected = wl_paste();

    // Restore saved clipboard
    if !saved.is_empty() {
        let _ = std::process::Command::new("wl-copy").arg(&saved).status();
    }

    if selected.is_empty() {
        Err("No text selected".to_string())
    } else {
        Ok(selected)
    }
}

#[cfg(target_os = "linux")]
fn simulate_copy_enigo() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to create Enigo instance: {e}"))?;
    enigo.key(Key::Control, Direction::Press)
        .map_err(|e| format!("Failed to press Ctrl: {e}"))?;
    enigo.key(Key::Unicode('c'), Direction::Click)
        .map_err(|e| format!("Failed to press C: {e}"))?;
    enigo.key(Key::Control, Direction::Release)
        .map_err(|e| format!("Failed to release Ctrl: {e}"))?;
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn get_selected_text_default() -> Result<String, String> {
    let saved_clipboard = clipboard::read_clipboard().unwrap_or_default();

    clipboard::write_clipboard("")?;
    thread::sleep(Duration::from_millis(50));

    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to create Enigo instance: {e}"))?;
    enigo.key(Key::Control, Direction::Press)
        .map_err(|e| format!("Failed to press Ctrl: {e}"))?;
    enigo.key(Key::Unicode('c'), Direction::Click)
        .map_err(|e| format!("Failed to press C: {e}"))?;
    enigo.key(Key::Control, Direction::Release)
        .map_err(|e| format!("Failed to release Ctrl: {e}"))?;

    thread::sleep(Duration::from_millis(100));

    let selected = clipboard::read_clipboard().unwrap_or_default();

    if !saved_clipboard.is_empty() {
        let _ = clipboard::write_clipboard(&saved_clipboard);
    }

    if selected.is_empty() {
        Err("No text selected".to_string())
    } else {
        Ok(selected)
    }
}
