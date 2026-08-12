/// Global hotkey integration for Linux / GNOME Wayland.
///
/// Uses GNOME's custom keyboard shortcut system (gsettings) which is the
/// only reliable mechanism on GNOME Wayland. The shortcut runs a tiny
/// shell snippet that writes a command to a named pipe; a background thread
/// in this process reads the pipe and emits the corresponding Tauri event.
use std::io::{BufRead, BufReader};
use tauri::{AppHandle, Emitter};

const HOTKEY_PIPE: &str = "/tmp/voiceflow-hotkey";
const GNOME_SCHEMA: &str = "org.gnome.settings-daemon.plugins.media-keys";
const GNOME_KEYBINDING_PATH: &str = "/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/voiceflow-toggle/";

pub fn start(app: AppHandle) {
    register_gnome_shortcut();
    start_pipe_listener(app);
}

/// Register (or update) the GNOME custom keybinding for Ctrl+Alt+S.
/// Idempotent — safe to call on every startup.
fn register_gnome_shortcut() {
    // Read the current list of custom keybinding paths.
    let current = std::process::Command::new("gsettings")
        .args(["get", GNOME_SCHEMA, "custom-keybindings"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();
    let current = current.trim().to_string();

    // Add our path to the list if it is not already present.
    let entry = format!("'{GNOME_KEYBINDING_PATH}'");
    if !current.contains(&entry) {
        let new_list = if current == "@as []" || current == "[]" {
            format!("['{GNOME_KEYBINDING_PATH}']")
        } else {
            let inner = current.trim_start_matches('[').trim_end_matches(']');
            format!("[{inner}, '{GNOME_KEYBINDING_PATH}']")
        };
        let _ = std::process::Command::new("gsettings")
            .args(["set", GNOME_SCHEMA, "custom-keybindings", &new_list])
            .status();
    }

    // Write the three properties for the keybinding.
    let custom_schema = format!("{GNOME_SCHEMA}.custom-keybinding:{GNOME_KEYBINDING_PATH}");
    // The command runs in a shell; single-quote the inner argument so that
    // g_shell_parse_argv splits it correctly into ["sh", "-c", "echo …"].
    let command = format!("sh -c 'echo toggle > {HOTKEY_PIPE}'");

    let _ = std::process::Command::new("gsettings")
        .args(["set", &custom_schema, "name", "VoiceFlow Toggle"])
        .status();
    let _ = std::process::Command::new("gsettings")
        .args(["set", &custom_schema, "command", &command])
        .status();
    let _ = std::process::Command::new("gsettings")
        .args(["set", &custom_schema, "binding", "<Control><Alt>s"])
        .status();

    log::info!("hotkeys: GNOME custom shortcut registered for Ctrl+Alt+S");
}

/// Spawn a thread that owns the named-pipe lifetime and re-opens it after
/// every EOF (each invocation of the GNOME shortcut closes its end).
fn start_pipe_listener(app: AppHandle) {
    std::thread::spawn(move || {
        // Remove any stale pipe from a previous run then create a fresh one.
        let _ = std::fs::remove_file(HOTKEY_PIPE);
        let status = std::process::Command::new("mkfifo")
            .arg(HOTKEY_PIPE)
            .status();
        if status.map_or(true, |s| !s.success()) {
            log::error!("hotkeys: failed to create pipe {HOTKEY_PIPE}");
            return;
        }
        log::info!("hotkeys: pipe listener ready at {HOTKEY_PIPE}");

        loop {
            // Opening a FIFO for reading blocks until a writer connects.
            // When the GNOME shortcut script finishes (closes its end) the
            // reader gets EOF, the inner loop ends, and we re-open for the
            // next invocation.
            match std::fs::File::open(HOTKEY_PIPE) {
                Ok(file) => {
                    for line in BufReader::new(file).lines().flatten() {
                        log::info!("hotkeys: pipe received '{line}'");
                        match line.trim() {
                            "toggle"    => { let _ = app.emit("hotkey-toggle",   "dictation"); }
                            "dictation" => { let _ = app.emit("hotkey-pressed",  "dictation"); }
                            "command"   => { let _ = app.emit("hotkey-pressed",  "command");   }
                            _ => {}
                        }
                    }
                }
                Err(e) => {
                    log::warn!("hotkeys: pipe open error: {e}");
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
        }
    });
}
