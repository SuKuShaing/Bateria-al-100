use tauri_plugin_autostart::MacosLauncher;
use tauri::{Manager, State, AppHandle};
use std::sync::Mutex;
use modules::config::AppSettings;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_app_settings(state: State<Mutex<AppSettings>>) -> AppSettings {
    state.lock().unwrap().clone()
}

#[tauri::command]
fn save_app_settings(app: AppHandle, state: State<Mutex<AppSettings>>, new_settings: AppSettings) -> Result<(), String> {
    let mut current_settings = state.lock().unwrap();
    *current_settings = new_settings.clone();
    
    // Save to disk
    let result = modules::config::save_settings(&app, &new_settings);

    // Update tray tooltip
    if let Some(tray) = app.tray_by_id("tray") {
        let _ = tray.set_tooltip(Some(format!("Te avisaré cuando la batería llegue al {}%", new_settings.threshold)));
    }

    result
}

mod modules;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec![])))
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_autostart::ManagerExt;
                let _ = app.handle().plugin(
                    tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec![])),
                );
                // Get the autostart manager
                let autostart_manager = app.autolaunch();
                if let Ok(enabled) = autostart_manager.is_enabled() {
                    if !enabled {
                        println!("Enabling autostart...");
                        let _ = autostart_manager.enable();
                    }
                }
            }

            // Story 3.1 - Load Settings
            let settings = modules::config::load_settings(app.handle());
            log::info!("Settings loaded: {:?}", settings);
            
            // Manage state with Mutex for interior mutability (Story 3.2)
            app.manage(Mutex::new(settings));

            modules::battery::init_background_poll(app.handle().clone());
            let handle = app.handle();
            modules::tray::create_tray(&handle)?;

            // Story 4.2 - Check for updates
            let handle_clone = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                use tauri_plugin_updater::UpdaterExt;
                if let Ok(updater) = handle_clone.updater() {
                     match updater.check().await {
                        Ok(Some(update)) => {
                            use tauri_plugin_notification::NotificationExt;
                            if let Err(e) = handle_clone
                                .notification()
                                .builder()
                                .title("Actualización disponible")
                                .body(&format!("Nueva versión {} disponible. Descárgala desde GitHub.", update.version))
                                .show()
                            {
                                log::error!("Failed to send update notification: {}", e);
                            }
                        }
                        Ok(None) => log::info!("No updates available"),
                        Err(e) => log::error!("Failed to check for updates: {}", e),
                    }
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![greet, get_app_settings, save_app_settings])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
