use starship_battery::Manager;
use std::thread;
use std::time::Duration;
use log::{info, error};
use tauri::{AppHandle, Runtime};
use tauri_plugin_notification::NotificationExt;

pub fn init_background_poll<R: Runtime>(app: AppHandle<R>) {
    // Spawn a standard thread for the loop since starship_battery might be blocking or we just want simple loop
    // Tauri's async runtime is also an option, but a dedicated thread is safe for simple polling.
    // Let's use std::thread for simplicity unless async is required by starship_battery (it's not usually).
    thread::spawn(move || {
        info!("Battery polling thread started");
        
        let manager = match Manager::new() {
            Ok(m) => m,
            Err(e) => {
                error!("Failed to initialize battery manager: {}", e);
                return;
            }
        };

        let mut already_alerted = false;

        loop {
            match manager.batteries() {
                Ok(mut batteries) => {
                    // We typically only care about the first battery
                    if let Some(Ok(battery)) = batteries.next() {
                        let state = battery.state();
                        let charge = battery.state_of_charge();
                        let level = charge.value * 100.0;
                        
                        info!("[INFO] Battery: {:.0}% | State: {:?}", level, state);

                        // Threshold Logic (Story 2.1)
                        // Hardcoded to 100% for now. Use 95.0 for reset to avoid flickering.
                        let is_full = level >= 100.0; 
                        let is_charging_or_full = state == starship_battery::State::Charging || state == starship_battery::State::Full;

                        if is_full && is_charging_or_full {
                            if !already_alerted {
                                info!("THRESHOLD REACHED: Battery is full at {:.0}%!", level);
                                
                                // Story 2.2 - Trigger Notification
                                let _ = app.notification()
                                    .builder()
                                    .title("Batería al 100")
                                    .body(format!("Tu batería está al {:.0}%. ¡Desconéctala para cuidar su salud!", level))
                                    .show();

                                already_alerted = true;
                            }
                        } else if state == starship_battery::State::Discharging || level < 95.0 {
                            // Reset latch if we are discharging OR if level drops significantly
                            if already_alerted {
                                info!("Resetting alert latch (State: {:?}, Level: {:.0}%)", state, level);
                                already_alerted = false;
                            }
                        }
                    } else {
                        info!("[INFO] No battery found");
                    }
                },
                Err(e) => error!("Failed to list batteries: {}", e),
            }

            // Poll every 100 seconds
            thread::sleep(Duration::from_secs(100));
        }
    });
}
