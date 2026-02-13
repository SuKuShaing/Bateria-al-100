# Story 2.3: Cool-down & Dismissal Logic

Status: done

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a User,
I want the app to stop nagging me if I've already seen the alert,
So that I don't get spammed every 60 seconds.

## Acceptance Criteria

- [ ] **Given** A notification has just been sent
- [ ] **When** The next polling cycle occurs (e.g., 1 min later) and battery is still 100%
- [ ] **Then** NO new notification is sent (Cool-down active)
- [ ] **But** If I unplug the charger (Discharging), the cool-down resets
- [ ] **And** If I plug it back in later, native notifications are re-enabled

## Dev Notes

- **Current Implementation:**
    - `battery.rs` already has a basic boolean latch: `mut already_alerted`.
    - Logic:
        - If `full` & `charging` & `!alerted` -> Notify, `alerted = true`.
        - Else if `discharging` or `level < 95` -> `alerted = false`.
- **Gap Analysis:**
    - The current latch IS the cool-down logic required!
    - It prevents spamming because `already_alerted` stays true until the state changes to Discharging or level drops below 95%.
    - So, technically, this story is **partially or mostly implemented** by the preventative latch I added in Story 2.1/2.2.
- **Verification:**
    - I just need to verify that this latch works as intended (i.e., doesn't spam).
    - Since checking this "live" takes time, I can review the code logic (Code Review) to confirm it satisfies the AC.
    - Code:
        ```rust
        if is_full && is_charging_or_full {
            if !already_alerted { ... already_alerted = true; }
        } else if state == Discharging || level < 95.0 {
            already_alerted = false;
        }
        ```
    - **Scenario 1:** At 100%, Charging.
        - Loop 1: Notify, `alerted=true`.
        - Loop 2 (100s later): `is_full` true, `!alerted` is false. NO NOTIFICATION. (Pass)
    - **Scenario 2:** Unplug (Discharging).
        - Loop 3: `state == Discharging`. `alerted` reset to false. (Pass)
    - **Scenario 3:** Plug back in at 100%.
        - Loop 4: `is_full` true, `!alerted` true. Notify. (Pass)
- **Conclusion:** The logic is ALREADY IMPLEMENTED. I just need to verify and close.

## Tasks

- [x] Initialize Workflow <!-- id: 91 -->
- [x] Verify Existing Logic (Audit) <!-- id: 92 -->
- [x] Update Story Status <!-- id: 93 -->
