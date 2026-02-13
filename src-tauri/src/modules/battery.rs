use starship_battery::Manager;
use std::thread;
use std::time::Duration;
use log::{info, error};

pub fn init_background_poll() {
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

        loop {
            match manager.batteries() {
                Ok(mut batteries) => {
                    // We typically only care about the first battery
                    if let Some(Ok(battery)) = batteries.next() {
                        let state = battery.state();
                        let charge = battery.state_of_charge();
                        
                        info!("[INFO] Battery: {:.0}% | State: {:?}", charge.value * 100.0, state);
                    } else {
                        info!("[INFO] No battery found");
                    }
                },
                Err(e) => error!("Failed to list batteries: {}", e),
            }

            // Poll every 5 seconds for debug/development (story says 60s, plan says 5s for dev)
            thread::sleep(Duration::from_secs(100));
        }
    });
}
