use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Runtime,
};

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i])?;

    let _tray = TrayIconBuilder::with_id("tray")
        // .icon(app.default_window_icon().unwrap().clone()) // Use app icon
        // Use a default icon or the app icon. 
        // For now, let's rely on the default app icon if possible, or tauri's icon handling.
        // In v2, icon is mandatory for creation usually.
        // Let's try attempting to load the app icon or a default. 
        // Actually, let's use the app's default icon.
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            match event.id.as_ref() {
                "quit" => {
                    log::info!("Quit menu item clicked");
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|_tray, event| {
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    ..
                } => {
                    log::info!("Tray icon left clicked");
                    // Here we could toggle the window visibility
                    // let app = tray.app_handle();
                    // if let Some(window) = app.get_window("main") {
                    //     let _ = window.show();
                    //     let _ = window.set_focus();
                    // }
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}
