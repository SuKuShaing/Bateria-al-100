use tauri_plugin_autostart::MacosLauncher;
// use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod modules;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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

            modules::battery::init_background_poll();
            let handle = app.handle();
            modules::tray::create_tray(&handle)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
