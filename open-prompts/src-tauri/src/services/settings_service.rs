use crate::error::{map_err, AppResult};
use crate::models::settings::AppSettings;
use crate::services::storage::{atomic_write, StoragePaths};
use std::fs;

pub fn load_settings(paths: &StoragePaths) -> AppResult<AppSettings> {
    if !paths.settings_path.exists() {
        let settings = AppSettings::default();
        save_settings(paths, &settings)?;
        return Ok(settings);
    }

    let data = fs::read_to_string(&paths.settings_path).map_err(map_err)?;
    match serde_json::from_str::<AppSettings>(&data) {
        Ok(settings) => Ok(settings),
        Err(_) => {
            let settings = AppSettings::default();
            save_settings(paths, &settings)?;
            Ok(settings)
        }
    }
}

pub fn save_settings(paths: &StoragePaths, settings: &AppSettings) -> AppResult<()> {
    let json = serde_json::to_string_pretty(settings).map_err(map_err)?;
    atomic_write(&paths.settings_path, json.as_bytes())
}
