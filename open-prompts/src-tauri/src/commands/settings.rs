use crate::error::AppResult;
use crate::models::settings::AppSettings;
use crate::services::settings_service;
use crate::state::AppState;

#[tauri::command]
pub fn get_settings(state: tauri::State<'_, AppState>) -> AppResult<AppSettings> {
    settings_service::load_settings(&state.paths)
}

#[tauri::command]
pub fn save_settings(
    state: tauri::State<'_, AppState>,
    settings: AppSettings,
) -> AppResult<AppSettings> {
    settings_service::save_settings(&state.paths, &settings)?;
    Ok(settings)
}
