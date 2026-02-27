mod commands;
mod error;
mod models;
mod platform;
mod services;
mod state;

use tauri::Manager;
use services::storage::{self, StoragePaths};
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let paths = resolve_storage_paths().expect("Failed to resolve storage paths");
    let app_state = AppState::new(paths.clone());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
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

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
