use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use crate::error::AppResult;
use crate::state::AppState;
use tauri::Manager;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

#[cfg(target_os = "windows")]
use crate::platform::windows as win32;

#[tauri::command]
pub fn get_current_hotkey(state: tauri::State<'_, AppState>) -> AppResult<String> {
    Ok(state.current_hotkey.lock().clone())
}

#[tauri::command]
pub fn set_hotkey(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    hotkey: Option<String>,
) -> AppResult<String> {
    let current = state.current_hotkey.lock().clone();

    // Unregister current
    if let Err(e) = app.global_shortcut().unregister(current.as_str()) {
        eprintln!("Failed to unregister hotkey '{}': {}", current, e);
    }

    let new_hotkey = hotkey.unwrap_or_else(|| "CommandOrControl+8".to_string());

    // Register new
    register_hotkey(&app, &state, &new_hotkey)?;

    *state.current_hotkey.lock() = new_hotkey.clone();
    Ok(new_hotkey)
}

#[tauri::command]
pub fn pause_hotkey(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> AppResult<()> {
    let current = state.current_hotkey.lock().clone();
    if let Err(e) = app.global_shortcut().unregister(current.as_str()) {
        eprintln!("Failed to pause hotkey: {}", e);
    }
    Ok(())
}

#[tauri::command]
pub fn resume_hotkey(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> AppResult<()> {
    let current = state.current_hotkey.lock().clone();
    register_hotkey(&app, &state, &current)
}

/// Register the global hotkey with the launcher show/toggle handler.
/// Called from setup hook AND from set_hotkey/resume_hotkey commands.
pub fn register_hotkey(app: &tauri::AppHandle, _state: &AppState, hotkey: &str) -> AppResult<()> {
    let app_handle = app.clone();

    app.global_shortcut()
        .on_shortcut(hotkey, move |_app, _shortcut, _event| {
            // The event fires on both press and release — only act on press
            if _event.state == ShortcutState::Pressed {
                handle_hotkey_press(&app_handle);
            }
        })
        .map_err(|e| format!("Failed to register hotkey '{}': {}", hotkey, e))?;

    Ok(())
}

fn handle_hotkey_press(app: &tauri::AppHandle) {
    static LAST_TRIGGER: OnceLock<Mutex<Instant>> = OnceLock::new();
    let lock = LAST_TRIGGER.get_or_init(|| Mutex::new(Instant::now() - Duration::from_secs(1)));
    if let Ok(mut last) = lock.lock() {
        if last.elapsed() < Duration::from_millis(200) {
            return;
        }
        *last = Instant::now();
    }

    let launcher = match app.get_webview_window("launcher") {
        Some(w) => w,
        None => return,
    };

    // Toggle: if visible, hide and return
    if launcher.is_visible().unwrap_or(false) {
        let _ = launcher.hide();
        return;
    }

    // Get state for HWND storage
    let state = match app.try_state::<AppState>() {
        Some(s) => s,
        None => return,
    };

    // Capture the currently focused window (filter our own process)
    #[cfg(target_os = "windows")]
    {
        let own_pid = win32::get_current_process_id();
        if let Some(hwnd) = win32::capture_foreground_hwnd(own_pid) {
            *state.last_external_hwnd.lock() = Some(hwnd);

            // Position launcher on the correct monitor
            if let Some((x, y)) = win32::get_launcher_position(hwnd, 650, 400) {
                let _ = launcher.set_position(tauri::PhysicalPosition::new(x, y));
            }
        }
    }

    // Show and focus the launcher
    let _ = launcher.show();
    let _ = launcher.set_focus();
}
