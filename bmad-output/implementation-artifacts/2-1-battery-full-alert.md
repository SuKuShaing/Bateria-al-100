# Story 2.1: Threshold Logic & Event Emission (Rust)

Status: done

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a Developer,
I want the backend to detect when the battery crosses the threshold,
So that I can trigger downstream actions like notifications.

## Acceptance Criteria

- [ ] **Given** The battery polling loop is running
- [ ] **When** Unfortunately, the battery level is >= 100% (hardcoded for now)
- [ ] **And** The status is "Charging" or "Full"
- [ ] **Then** An internal event or flag is raised indicating "Threshold Reached"
- [ ] **And** The logic handles edge cases (e.g., already at 100% when app starts)

## Dev Notes

- **File:** `src-tauri/src/modules/battery.rs`
- **Logic:**
    - Update `init_background_poll` to maintain state.
    - Check if `charge_percent >= 100.0` and `status == Charging | Full`.
    - **Debounce:** Ensure we don't spam. Maybe just log "Threshold reached" once per crossing.
    - **Extension:** In implementation, I'll probably add a helper function `check_threshold(battery)`.

## Tasks

- [ ] Initialize Workflow <!-- id: 80 -->
- [x] Implement Threshold Logic (Hardcoded 100%) <!-- id: 81 -->
    - [x] Verify Logic (Log "Threshold Reached") <!-- id: 82 -->
    - [ ] Update Story Status <!-- id: 83 -->
