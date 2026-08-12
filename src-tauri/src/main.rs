// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "linux")]
    {
        // Force dark GTK theme so the native window chrome is dark on Linux/GNOME.
        std::env::set_var("GTK_THEME", "Adwaita:dark");
        // Force X11/XWayland mode so XGrabKey global shortcuts work system-wide.
        // Without this, on a Wayland session the grab only fires for X11 windows,
        // not native Wayland apps. XWayland forwards the grab to Mutter which
        // intercepts the key even when native Wayland apps are focused.
        if std::env::var("GDK_BACKEND").is_err() {
            std::env::set_var("GDK_BACKEND", "x11");
        }
        if std::env::var("DISPLAY").is_err() {
            std::env::set_var("DISPLAY", ":0");
        }
    }

    voiceflow_lib::run()
}
