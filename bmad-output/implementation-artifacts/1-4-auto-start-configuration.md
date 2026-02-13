# Story 1.4: Auto-Start Configuration

Status: done

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a User,
I want the app to start automatically when I turn on my computer,
so that I don't have to remember to launch it every day.

## Acceptance Criteria

1. **Given** The app is installed and running
2. **When** I restart my computer and log in
3. **Then** The application process starts automatically in the background
4. **And** The System Tray icon appears without user intervention
5. **And** The implementation uses `tauri-plugin-autostart`

## Tasks / Subtasks

- [ ] Add Autostart Plugin (AC: 5)
    - [ ] Add `tauri-plugin-autostart` to `Cargo.toml`
    - [ ] Run `cargo tauri add autostart` (or manual config)
    - [ ] Configure permissions in `capabilities/default.json` (allow `autostart:allow-enable`)
- [ ] Implement Auto-Enable Logic (AC: 1, 3)
    - [ ] In `modules/mod.rs` or `lib.rs`, allow enabling autostart
    - [ ] Ideally, enable it on first run or verify it's enabled in `.setup()`
    - [ ] _Decision:_ For MVP, force enable on every startup or check if enabled and enable it.
- [ ] Verify Implementation (AC: 2, 4)
    - [ ] Build release version (autostart often doesn't work well in dev mode) or test dev behavior
    - [ ] Restart session (or use task manager to see startup items)

## Dev Notes

- **Tauri v2:** Uses `tauri-plugin-autostart`. Needs `npm install @tauri-apps/plugin-autostart` for JS side if needed, but we can do it purely in Rust or just enable it via Rust.
- **Permissions:** Crucial step. Needs `autostart:default` or specific permissions in `src-tauri/capabilities`.

### References

- [Architecture Decision Document](file:///d:/Progra/Proyectos_personales/Bateria-al-100/bmad-output/planning-artifacts/architecture.md)
- [Epic 1: Foundation](file:///d:/Progra/Proyectos_personales/Bateria-al-100/bmad-output/planning-artifacts/epics.md#Epic-1)

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
