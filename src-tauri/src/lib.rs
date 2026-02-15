mod api;
mod audio;
mod commands;
mod config;
mod platform;
pub mod state;

use audio::recorder::Recorder;
use commands::audio::RecorderHandle;
use state::AppState;
use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .manage(AppState::default())
        .manage(RecorderHandle(Arc::new(std::sync::Mutex::new(Recorder::new()))))
        .invoke_handler(tauri::generate_handler![
            commands::audio::start_recording,
            commands::audio::stop_recording,
            commands::audio::list_audio_devices,
            commands::transcribe::transcribe_audio,
            commands::llm::cleanup_transcript,
            commands::llm::execute_voice_command,
            commands::inject::inject_text,
            commands::inject::get_selected_text,
            commands::context::get_context,
            commands::settings::get_settings,
            commands::settings::update_settings,
        ])
        .setup(|app| {
            // Build system tray menu
            let show_i = MenuItem::with_id(app, "show", "Settings", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            TrayIconBuilder::with_id("main-tray")
                .tooltip("VoiceFlow")
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            // Register global shortcuts
            let app_handle = app.handle().clone();
            let dictation_shortcut =
                Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyD);
            let command_shortcut =
                Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyC);

            app.global_shortcut().on_shortcuts(
                [dictation_shortcut, command_shortcut],
                move |_app, shortcut, event| {
                    let is_dictation = shortcut.matches(
                        Modifiers::CONTROL | Modifiers::ALT,
                        Code::KeyD,
                    );
                    let is_command = shortcut.matches(
                        Modifiers::CONTROL | Modifiers::ALT,
                        Code::KeyC,
                    );

                    let mode = if is_dictation {
                        "dictation"
                    } else if is_command {
                        "command"
                    } else {
                        return;
                    };

                    match event.state {
                        ShortcutState::Pressed => {
                            let _ = app_handle.emit("hotkey-pressed", mode);
                        }
                        ShortcutState::Released => {
                            let _ = app_handle.emit("hotkey-released", mode);
                        }
                    }
                },
            )?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running VoiceFlow");
}
