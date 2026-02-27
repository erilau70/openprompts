use crate::services::storage::StoragePaths;
use parking_lot::Mutex;

pub struct AppState {
    pub paths: StoragePaths,
    pub last_external_hwnd: Mutex<Option<isize>>,
    pub current_hotkey: Mutex<String>,
}

impl AppState {
    pub fn new(paths: StoragePaths) -> Self {
        Self {
            paths,
            last_external_hwnd: Mutex::new(None),
            current_hotkey: Mutex::new("CommandOrControl+8".to_string()),
        }
    }
}
