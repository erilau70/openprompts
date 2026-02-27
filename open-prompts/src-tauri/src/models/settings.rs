use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeneralSettings {
    pub auto_launch: bool,
    pub hotkey: String,
    pub editor_always_on_top: bool,
    pub welcome_screen_dismissed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppearanceSettings {
    pub theme: String,
    pub accent_color: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub general: GeneralSettings,
    pub appearance: AppearanceSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            general: GeneralSettings {
                auto_launch: false,
                hotkey: "CommandOrControl+8".to_string(),
                editor_always_on_top: true,
                welcome_screen_dismissed: false,
            },
            appearance: AppearanceSettings {
                theme: "dark".to_string(),
                accent_color: "avocado".to_string(),
            },
        }
    }
}
