use crate::error::AppResult;
use crate::state::AppState;
use tauri::Manager;
use tauri_plugin_clipboard_manager::ClipboardExt;

#[cfg(target_os = "windows")]
use crate::platform::windows as win32;

/// Paste text and dismiss the launcher.
/// CRITICAL sequencing: clipboard → force_foreground → hide → wait_modifier_release → wait_focus → SendInput
#[tauri::command]
pub async fn paste_and_dismiss(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    text: String,
) -> AppResult<()> {
    // 1. Write to clipboard
    app.clipboard().write_text(&text).map_err(|e| e.to_string())?;

    // 2. Small delay for clipboard propagation
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;

    // 3. Get stored HWND
    let hwnd = { *state.last_external_hwnd.lock() };

    // 4. Restore focus BEFORE hiding (we need to be foreground process for SetForegroundWindow)
    #[cfg(target_os = "windows")]
    if let Some(hwnd) = hwnd {
        if win32::is_valid_window(hwnd) {
            win32::force_foreground(hwnd);
        }
    }

    // 5. Hide launcher
    if let Some(launcher) = app.get_webview_window("launcher") {
        let _ = launcher.hide();
    }

    // 6. Wait for modifier keys to be released (user may still hold Ctrl from Ctrl+/)
    #[cfg(target_os = "windows")]
    win32::wait_for_modifier_release(500);

    // 7. Wait for focus to settle
    #[cfg(target_os = "windows")]
    if let Some(hwnd) = hwnd {
        win32::wait_for_focus(hwnd, 200);
    }

    // 8. Simulate Ctrl+V
    #[cfg(target_os = "windows")]
    {
        if !win32::send_ctrl_v() {
            eprintln!("SendInput for Ctrl+V may have been blocked (UIPI or key state issue)");
        }
    }

    Ok(())
}

/// Dismiss launcher without pasting
#[tauri::command]
pub async fn dismiss_window(app: tauri::AppHandle) -> AppResult<()> {
    if let Some(launcher) = app.get_webview_window("launcher") {
        let _ = launcher.hide();
    }
    Ok(())
}

/// Copy text to clipboard only (no dismiss, no paste)
#[tauri::command]
pub async fn copy_to_clipboard(app: tauri::AppHandle, text: String) -> AppResult<()> {
    app.clipboard().write_text(&text).map_err(|e| e.to_string())?;
    Ok(())
}

/// Open the editor window. Create if not exists, otherwise show/focus.
#[tauri::command]
pub async fn open_editor_window(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> AppResult<()> {
    // Check if editor already exists
    if let Some(editor) = app.get_webview_window("editor") {
        let _ = editor.show();
        let _ = editor.set_focus();
        return Ok(());
    }

    // Create editor window programmatically
    let settings = crate::services::settings_service::load_settings(&state.paths)?;
    let always_on_top = settings.general.editor_always_on_top;

    let editor = tauri::WebviewWindowBuilder::new(
        &app,
        "editor",
        tauri::WebviewUrl::App("index.html".into()),
    )
    .title("OpenPrompts Editor")
    .inner_size(1000.0, 700.0)
    .min_inner_size(800.0, 600.0)
    .resizable(true)
    .decorations(true)
    .always_on_top(always_on_top)
    .skip_taskbar(false)
    .visible(true)
    .build()
    .map_err(|e| e.to_string())?;

    let _ = editor.set_focus();
    Ok(())
}

/// Close the editor window
#[tauri::command]
pub async fn close_editor_window(app: tauri::AppHandle) -> AppResult<()> {
    if let Some(editor) = app.get_webview_window("editor") {
        let _ = editor.close();
    }
    Ok(())
}

/// Quit the entire app process.
#[tauri::command]
pub fn quit_app(app: tauri::AppHandle) -> AppResult<()> {
    app.exit(0);
    Ok(())
}
