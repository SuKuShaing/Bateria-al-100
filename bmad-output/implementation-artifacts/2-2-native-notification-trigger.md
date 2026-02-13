# Story 2.2: Native Notification Trigger

Status: done

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a User,
I want to see a visible notification from my OS when the battery is full,
So that I notice it even if I'm looking at another window.

## Acceptance Criteria

- [ ] **Given** The "Threshold Reached" event has fired
- [ ] **When** The notification logic executes
- [ ] **Then** A native system notification appears (Toast on Windows, Banner on Mac/Linux)
- [ ] **And** The title says "Battery Full"
- [ ] **And** The body says "Your battery is at [X]%. Unplug now to save health."
- [ ] **And** The system "Critical/Alarm" sound plays (if supported by OS/crate)

## Dev Notes

- **Dependencies:** `tauri-plugin-notification` (or `notify-rust` if we want pure Rust backend control, but Tauri v2 has a good plugin).
    - Let's use `tauri-plugin-notification` for better cross-platform support and potential future frontend control.
    - Actually, since we are in `battery.rs` (a backend thread), we might not have easy access to the Tauri AppHandle unless we passed it in.
    - **Architecture Decision:**
        - Option A: Pass `AppHandle` to `init_background_poll`.
        - Option B: Emit an event to Frontend, Frontend shows notification. (Adds latency, requires frontend running).
        - Option C: Use `notify-rust` crate directly in the backend thread.
    - **Decision:** Option A is best for Tauri integration. We already have `start_background_poll`... wait, `init_background_poll` currently takes nothing. I need to simple pass `AppHandle` to it.
- **Implementation:**
    - Add `tauri-plugin-notification` to `Cargo.toml`.
    - Initialize plugin in `lib.rs`.
    - Pass `AppHandle` to `battery::init_background_poll`.
    - Use `check_permission`? For backend-initiated notifications, often permissions are implied or handled by the OS for the app.
    - Construct and show notification.

## Tasks

- [ ] Initialize Workflow <!-- id: 85 -->
- [x] Add Notification Dependency <!-- id: 86 -->
- [x] Refactor Battery Module to accept AppHandle <!-- id: 87 -->
- [x] Implement Notification Trigger <!-- id: 88 -->
- [x] Verify Notification (Manual Test) <!-- id: 89 -->
- [x] Update Story Status <!-- id: 90 -->
