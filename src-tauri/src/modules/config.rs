use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub threshold: u8,
    pub enabled: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            threshold: 100,
            enabled: true,
        }
    }
}

pub fn get_config_path<R: Runtime>(app: &AppHandle<R>) -> Option<PathBuf> {
    app.path().app_config_dir().ok().map(|mut path| {
        path.push("settings.json");
        path
    })
}

pub fn load_settings<R: Runtime>(app: &AppHandle<R>) -> AppSettings {
    let config_path = get_config_path(app);

    if let Some(path) = config_path {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(settings) = serde_json::from_str(&content) {
                    return settings;
                }
            }
        }
    }

    // Default if loading fails or file doesn't exist
    AppSettings::default()
}

pub fn save_settings<R: Runtime>(app: &AppHandle<R>, settings: &AppSettings) -> Result<(), String> {
    let config_path = get_config_path(app).ok_or("Could not resolve app config directory")?;

    // Ensure directory exists
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let content = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(config_path, content).map_err(|e| e.to_string())?;

    Ok(())
}
