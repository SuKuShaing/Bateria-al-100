# Story 3.2: API Commands (Rust -> TS)

Status: done

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a Frontend Developer,
I want to call Rust functions to get and save settings,
So that I can build the UI without knowing Rust details.

## Acceptance Criteria

- [ ] **Given** The Rust backend
- [ ] **When** I invoke `get_app_settings`
- [ ] **Then** It returns the current settings struct (JSON)
- [ ] **And** When I invoke `save_app_settings` with a new JSON object
- [ ] **Then** It updates the state AND writes to disk
- [ ] **And** The keys in JSON are `camelCase` (e.g., `threshold`, `enabled`) matching the TS interface.

## Dev Notes

- **Struct Annotation:** Ensure `#[serde(rename_all = "camelCase")]` is on `AppSettings` struct in `config.rs`.
- **Commands:**

    ```rust
    #[tauri::command]
    pub fn get_app_settings(state: State<AppSettings>) -> AppSettings {
        state.inner().clone() // Or similar, might need Mutex/RwLock if mutable
    }

    #[tauri::command]
    pub fn save_app_settings(app: AppHandle, state: State<SafeSettings>, new_settings: AppSettings) -> Result<(), String> {
         // update state
         // save to disk
    }
    ```

- **State Management:**
    - Tauri `manage` adds state immutably by default? Or interior mutability is needed?
    - Usually `Mutex<AppSettings>` is better for runtime updates.
    - _Decision:_ Wrap `AppSettings` in `std::sync::Mutex` when managing it: `app.manage(Mutex::new(settings))`.

## Tasks

- [ ] Initialize Workflow <!-- id: 104 -->
- [x] Update Struct with Serde Rename <!-- id: 105 -->
- [x] Implement Tauri Commands <!-- id: 106 -->
- [x] Register Commands in lib.rs <!-- id: 107 -->
- [x] Verify Invoke (e.g. from a temp script or console) <!-- id: 108 -->
- [x] Update Story Status <!-- id: 109 -->
