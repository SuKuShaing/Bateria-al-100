# Story 1.2: Battery State Polling (Rust)

Status: done

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a User,
I want the system to check my battery level in the background,
so that it knows when to trigger alerts without me checking manually.

## Acceptance Criteria

1. **Given** The app is running in the background
2. **When** The polling interval (e.g., 60 seconds) elapses
3. **Then** The Rust backend reads the current battery percentage using `system_stat` or `battery` crate
4. **And** The current percentage and charging status are logged to the console
5. **And** The loop repeats indefinitely while the app is running

## Tasks / Subtasks

- [ ] Add Battery Crate Dependency (AC: 3)
    - [ ] Add `battery` or `system_stat` to `src-tauri/Cargo.toml` (Research determines best crate for Windows)
    - [ ] Run `cargo check` to verify dependency
- [ ] Implement Polling Logic (AC: 1, 2, 5)
    - [ ] Create a new module `src-tauri/src/modules/battery.rs` (if using module structure) or add to `lib.rs`
    - [ ] Implement an infinite loop (tokio::spawn or std::thread)
    - [ ] Set interval to 60 seconds (use a shorter interval like 5s for debug mode)
- [ ] Read and Log Battery Status (AC: 3, 4)
    - [ ] Read battery state (percentage, charging status)
    - [ ] Log: `[INFO] Battery: X% | State: Y` using the `log` crate
- [ ] Verify Implementation
    - [ ] Run `npm run tauri dev`
    - [ ] Check console for periodic updates

## Dev Notes

- **Crate Selection:** checking `system_stat` vs `battery`. `battery` crate is often preferred but unmaintained; `system_stat` is simpler. actually `starship-battery` is a common choice or just `windows` crate calls if needed. Let's start with `system_stat` or `battery` as per Epics. _Update: `battery` crate is archived. `system_stat` might be better or `starship-battery`._
- **Concurrency:** Tauri is async (Tokio). Use `tokio::spawn` for the background task to avoid blocking the main thread.
- **Architecture:**
    - Keep it clean. Create `src-tauri/src/battery_monitor.rs`.
    - Call init function from `lib.rs`.

### References

- [Architecture Decision Document](file:///d:/Progra/Proyectos_personales/Bateria-al-100/bmad-output/planning-artifacts/architecture.md)
- [Epic 1: Foundation](file:///d:/Progra/Proyectos_personales/Bateria-al-100/bmad-output/planning-artifacts/epics.md#Epic-1)

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
