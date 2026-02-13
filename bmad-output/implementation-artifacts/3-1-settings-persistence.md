# Story 3.1: Settings Persistence (Rust)

Status: done

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a Developer,
I want to save user preferences to disk,
So that they don't have to re-configure the app every time it restarts.

## Acceptance Criteria

- [ ] **Given** The app is running
- [ ] **When** I save a `Settings` struct (threshold: u8, enabled: bool)
- [ ] **Then** It is serialized to JSON and written to the OS AppConfig directory
- [ ] **And** On app startup, the file is read and deserialized back into the struct
- [ ] **And** If the file doesn't exist, default settings (100% threshold) are used

## Dev Notes

- **Dependencies:** `serde`, `serde_json` (already in Cargo.toml), `tauri-plugin-store` (optional, or just use `std::fs` with `tauri::path::BaseDirectory`).
    - _Decision:_ Let's use standard `std::fs` + `serde_json` for simplicity and control, utilizing Tauri's path resolver to find the AppConfig dir.
    - Actually, `tauri-plugin-store` is very popular for this. But for a simple struct, `fs` is fine and less bloat.
- **Structure:**
    ```rust
    #[derive(Serialize, Deserialize)]
    pub struct AppSettings {
        pub threshold: u8,
        pub enabled: bool,
    }
    ```
- **Module:** `src-tauri/src/modules/config.rs`
- **Functions:**
    - `load_settings() -> AppSettings`
    - `save_settings(settings: &AppSettings) -> Result<()>`

## Tasks

- [ ] Initialize Workflow <!-- id: 99 -->
- [x] Create Config Module <!-- id: 100 -->
- [x] Implement Load/Save Logic <!-- id: 101 -->
- [x] Verify Consistency (Write/Read Test) <!-- id: 102 -->
- [x] Update Story Status <!-- id: 103 -->
