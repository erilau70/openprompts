mod commands;
mod error;
mod models;
mod platform;
mod services;
mod state;

use services::storage::{self, StoragePaths};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let paths = resolve_storage_paths().expect("Failed to resolve storage paths");
    let app_state = AppState::new(paths.clone());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Data commands
            commands::data::get_index,
            commands::data::get_folders,
            commands::data::get_prompt,
            commands::data::save_prompt,
            commands::data::delete_prompt,
            commands::data::add_folder,
            commands::data::rename_folder,
            commands::data::delete_folder,
            commands::data::search_prompts,
            commands::data::record_usage,
            // Settings commands
            commands::settings::get_settings,
            commands::settings::save_settings,
            // Window commands
            commands::windows::paste_and_dismiss,
            commands::windows::dismiss_window,
            commands::windows::copy_to_clipboard,
            commands::windows::open_editor_window,
            commands::windows::close_editor_window,
            commands::windows::quit_app,
            // Hotkey commands
            commands::hotkey::get_current_hotkey,
            commands::hotkey::set_hotkey,
            commands::hotkey::pause_hotkey,
            commands::hotkey::resume_hotkey,
        ])
        .setup(|app| {
            // Use the paths from the managed state
            let paths =
                resolve_storage_paths().expect("Failed to resolve storage paths in setup");

            // Ensure storage directories exist
            storage::ensure_storage_dirs(&paths).expect("Failed to create storage directories");

            // Load index and seed if needed
            let mut index = services::index_service::load_index(&paths)
                .expect("Failed to load prompt index");

            if let Err(e) = services::seed_service::seed_if_needed(&paths, &mut index) {
                eprintln!("Warning: Failed to seed sample prompts: {}", e);
            }

            // Register global hotkey
            let state = app.state::<AppState>();
            let hotkey = state.current_hotkey.lock().clone();
            if let Err(e) = commands::hotkey::register_hotkey(&app.handle(), &state, &hotkey) {
                eprintln!("Warning: Failed to register hotkey '{}': {}", hotkey, e);
            }

            // Build tray menu
            let open_launcher = MenuItem::with_id(
                app,
                "tray_open_launcher",
                "Open Launcher",
                true,
                None::<&str>,
            )?;
            let open_editor = MenuItem::with_id(
                app,
                "tray_open_editor",
                "Open Editor",
                true,
                None::<&str>,
            )?;
            let quit = MenuItem::with_id(app, "tray_quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_launcher, &open_editor, &quit])?;

            let tray_icon = app.default_window_icon().cloned();

            let mut tray_builder = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("OpenPrompts")
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "tray_open_launcher" => show_launcher_window(app),
                    "tray_open_editor" => show_editor_window(app),
                    "tray_quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        show_launcher_window(&tray.app_handle());
                    }
                });

            if let Some(icon) = tray_icon {
                tray_builder = tray_builder.icon(icon);
            }

            let _tray = tray_builder.build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn show_launcher_window(app: &tauri::AppHandle) {
    if let Some(launcher) = app.get_webview_window("launcher") {
        let _ = launcher.show();
        let _ = launcher.set_focus();
    }
}

fn show_editor_window(app: &tauri::AppHandle) {
    if let Some(editor) = app.get_webview_window("editor") {
        let _ = editor.show();
        let _ = editor.set_focus();
        return;
    }

    let state = match app.try_state::<AppState>() {
        Some(s) => s,
        None => return,
    };

    let settings = match services::settings_service::load_settings(&state.paths) {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("Failed to load settings for editor window: {}", e);
            return;
        }
    };

    let always_on_top = settings.general.editor_always_on_top;
    match tauri::WebviewWindowBuilder::new(app, "editor", tauri::WebviewUrl::App("index.html".into()))
        .title("OpenPrompts Editor")
        .inner_size(1000.0, 700.0)
        .min_inner_size(800.0, 600.0)
        .resizable(true)
        .decorations(true)
        .always_on_top(always_on_top)
        .skip_taskbar(false)
        .visible(true)
        .build()
    {
        Ok(editor) => {
            let _ = editor.set_focus();
        }
        Err(e) => {
            eprintln!("Failed to open editor window from tray: {}", e);
        }
    }
}

fn resolve_storage_paths() -> Result<StoragePaths, String> {
    let home = dirs::home_dir().ok_or("Could not determine home directory")?;
    let root = home.join(".openprompt");
    let prompts_dir = root.join("prompts");
    let index_path = root.join("index.json");
    let settings_path = root.join("settings.json");

    Ok(StoragePaths {
        root,
        prompts_dir,
        index_path,
        settings_path,
    })
}
