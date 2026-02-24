use starship_battery::Manager;
use std::thread;
use log::{info, error};
use std::time::{Duration, Instant};
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

        let mut alert_count = 0;
        let mut last_alert_time: Option<Instant> = None;

        loop {
            // Get current threshold settings
            let (threshold, enabled) = {
                use tauri::Manager;
                // Story 3.2: Settings are now wrapped in Mutex
                match app.state::<std::sync::Mutex<crate::modules::config::AppSettings>>().lock() {
                    Ok(settings) => (settings.threshold, settings.enabled),
                    Err(e) => {
                        error!("Failed to lock settings mutex: {}", e);
                        (100, true) // Fallback
                    }
                }
            };

            match manager.batteries() {
                Ok(mut batteries) => {
                    // We typically only care about the first battery
                    if let Some(Ok(battery)) = batteries.next() {
                        let state = battery.state();
                        let charge = battery.state_of_charge();
                        let level = charge.value * 100.0;
                        
                        info!("[INFO] Battery: {:.0}% | State: {:?} | Threshold: {} | Enabled: {}", level, state, threshold, enabled);

                        if !enabled {
                            // If notifications are disabled, just reset latch if needed
                            match state {
                                starship_battery::State::Discharging => {
                                    alert_count = 0;
                                    last_alert_time = None;
                                },
                                _ => {}
                            }
                        } else {
                            // Threshold Logic (Refactored for Story 3)
                            // Use dynamic threshold
                            let is_full = level >= threshold as f32; 
                            // Windows sometimes reports State::Unknown when plugged in at 100%
                            let is_plugged_in = state != starship_battery::State::Discharging && state != starship_battery::State::Empty;

                            if is_full && is_plugged_in {
                                let mut should_alert = false;

                                if alert_count == 0 {
                                    should_alert = true;
                                } else if alert_count < 5 {
                                    if let Some(last_time) = last_alert_time {
                                        // Calculate required delay based on alert_count
                                        // 1st alert: immediate (count 0 -> 1)
                                        // 2nd alert: 5 mins (count 1 -> 2)
                                        // 3rd alert: 10 mins (count 2 -> 3)
                                        // 4th alert: 20 mins (count 3 -> 4)
                                        // 5th alert: 40 mins (count 4 -> 5)
                                        let delay_minutes = match alert_count {
                                            1 => 5,
                                            2 => 10,
                                            3 => 20,
                                            4 => 40,
                                            _ => 0,
                                        };

                                        if last_time.elapsed() >= Duration::from_secs(delay_minutes * 60) {
                                            should_alert = true;
                                        }
                                    }
                                }

                                if should_alert {
                                    info!("THRESHOLD REACHED: Battery is full at {:.0}%! (Alert {}/5)", level, alert_count + 1);
                                    
                                    // Story 2.2 - Trigger Notification
                                    let _ = app.notification()
                                        .builder()
                                        .title("Batería al 100")
                                        .body(format!("Tu batería está al {:.0}%. ¡Desconéctala para cuidar su salud!", level))
                                        .show();

                                    alert_count += 1;
                                    last_alert_time = Some(Instant::now());
                                }
                            } else if state == starship_battery::State::Discharging || level < (threshold as f32 - 5.0) {
                                // Reset latch if we are discharging OR if level drops significantly (5% hysteresis)
                                if alert_count > 0 {
                                    info!("Resetting alert latch (State: {:?}, Level: {:.0}%)", state, level);
                                    alert_count = 0;
                                    last_alert_time = None;
                                }
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
