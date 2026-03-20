use tauri_plugin_autostart::MacosLauncher;
use tauri::{Manager, State, AppHandle, Emitter};
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

// Story 4.3 - Manual update check from frontend
#[tauri::command]
async fn check_update(app: AppHandle) -> Result<Option<(String, String)>, String> {
    use tauri_plugin_updater::UpdaterExt;
    if let Ok(updater) = app.updater() {
        match updater.check().await {
            Ok(Some(update)) => {
                // Construct basic fallback URL if we can't get it directly from update (updater abstracts the assets)
                let url = format!("https://sukushaing.github.io/Bateria-al-100/");
                
                Ok(Some((update.version.clone(), url)))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Updater not available".to_string())
    }
}

// Story 4.4 - Animated Glow State
#[tauri::command]
fn get_battery_state() -> Result<(f32, String), String> {
    use starship_battery::Manager;
    let manager = Manager::new().map_err(|e| e.to_string())?;
    let mut batteries = manager.batteries().map_err(|e| e.to_string())?;
    
    if let Some(Ok(battery)) = batteries.next() {
        let state = battery.state();
        let level = battery.state_of_charge().value * 100.0;
        
        let state_str = match state {
            starship_battery::State::Charging => "Charging",
            starship_battery::State::Discharging => "Discharging",
            starship_battery::State::Empty => "Empty",
            starship_battery::State::Full => "Full",
            starship_battery::State::Unknown => {
                // Heuristic for unknown state: if it's plugged in and near 100, might be full/charging
                if level >= 99.0 { "Full" } else { "Charging" }
            }
        };
        Ok((level, state_str.to_string()))
    } else {
        Err("No battery found".to_string())
    }
}

// Story 4.5 - Check and Request Notification Permissions (Frontend access)
#[tauri::command]
fn check_notification_permission(app: AppHandle) -> String {
    use tauri_plugin_notification::NotificationExt;
    if let Ok(state) = app.notification().permission_state() {
        match state {
            tauri::plugin::PermissionState::Granted => "Granted".to_string(),
            tauri::plugin::PermissionState::Denied => "Denied".to_string(),
            tauri::plugin::PermissionState::Prompt => "Prompt".to_string(),
            _ => "Unknown".to_string(),
        }
    } else {
        "Error".to_string()
    }
}

#[tauri::command]
fn request_notification_permission(app: AppHandle) -> String {
    use tauri_plugin_notification::NotificationExt;
    if let Ok(state) = app.notification().request_permission() {
        match state {
            tauri::plugin::PermissionState::Granted => "Granted".to_string(),
            tauri::plugin::PermissionState::Denied => "Denied".to_string(),
            tauri::plugin::PermissionState::Prompt => "Prompt".to_string(),
            _ => "Unknown".to_string(),
        }
    } else {
        "Error".to_string()
    }
}

mod modules;

#[cfg(windows)]
pub fn send_notification<R: tauri::Runtime>(_app: &tauri::AppHandle<R>, title: &str, body: &str) {
    use tauri_winrt_notification::{Duration, Sound, Toast};
    if let Err(e) = Toast::new(Toast::POWERSHELL_APP_ID)
        .title(title)
        .text1(body)
        .sound(Some(Sound::Default))
        .duration(Duration::Short)
        .show()
    {
        log::error!("Fallo al enviar notificación WinRT: {:?}", e);
    }
}

#[cfg(not(windows))]
pub fn send_notification<R: tauri::Runtime>(app: &tauri::AppHandle<R>, title: &str, body: &str) {
    use tauri_plugin_notification::NotificationExt;
    if let Err(e) = app.notification().builder().title(title).body(body).show() {
        log::error!("Fallo al enviar notificación nativa: {}", e);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            log::info!("Se intentó abrir una nueva instancia con args: {:?}, cwd: {:?}", argv, cwd);
            let _ = app.emit("single-instance", ());
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
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
                        log::info!("Enabling autostart...");
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
                            use tauri::plugin::PermissionState;

                            let mut has_permission = false;
                            
                            if let Ok(state) = handle_clone.notification().permission_state() {
                                match state {
                                    PermissionState::Granted => has_permission = true,
                                    PermissionState::Denied => has_permission = false,
                                    _ => {
                                        if let Ok(new_state) = handle_clone.notification().request_permission() {
                                            has_permission = new_state == PermissionState::Granted;
                                        }
                                    }
                                }
                            }

                            if has_permission {
                                crate::send_notification(
                                    &handle_clone,
                                    "Actualización disponible",
                                    &format!("Nueva versión {} disponible. Descárgala desde GitHub.", update.version),
                                );
                            } else {
                                log::warn!("Notification permission denied, unable to show update alert.");
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
        .invoke_handler(tauri::generate_handler![
            greet, 
            get_app_settings, 
            save_app_settings, 
            check_update, 
            get_battery_state,
            check_notification_permission,
            request_notification_permission
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
