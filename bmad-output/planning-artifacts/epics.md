stepsCompleted: [1, 2, 3, 4]
inputDocuments: ["prd.md", "architecture.md"]
workflowType: 'epics'
status: 'complete'
completedAt: '2026-02-12'

# Batería al 100 - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for Batería al 100, decomposing the requirements from the PRD, UX Design if it exists, and Architecture requirements into implementable stories.

## Requirements Inventory

### Functional Requirements

FR1: System can read the current battery charge percentage level from the OS.
FR2: System can read the current charging status (Plugged In / Discharging).
FR3: System can poll battery status at a fixed interval (every 60-100 seconds).
FR4: User can receive a native OS notification when battery level >= configured threshold.
FR5: System will play the "Critical/Alarm" system sound when triggering a notification.
FR6: System will prevent notification spam (implementing a cool-down period after first alert).
FR7: Notification will automatically dismiss if the user unplugs the charger.
FR8: User can open a Settings UI from the System Tray.
FR9: User can set a custom battery percentage threshold (default: 100%).
FR10: System must persist user configuration to disk (survives restarts).
FR11: User defaults to 100% threshold if no configuration exists.
FR12: System runs as a background process with no taskbar presence (System Tray only).
FR13: User can interact with a Tray Icon to see quick status or open menu.
FR14: User can Right-Click Tray Icon to access "Exit" option.
FR15: System auto-starts on OS login (enabled by default).
FR16: System can check for updates against GitHub Releases in the background.
FR17: User is notified if a new version is available for download.

### NonFunctional Requirements

NFR1: Memory footprint should target **< 50MB RAM** when idle in the tray.
NFR2: CPU usage should be negligible (**< 0.1%**) during idle polling states.
NFR3: App startup time (to tray ready) should be **< 2 seconds**.
NFR4: System must run continuously for **7+ days** without crashing or leaking memory.
NFR5: System must correctly resume polling after the computer wakes from Sleep/Hibernate.
NFR6: System must handle "Missing Battery" states (e.g., desktops or hardware error) gracefully without crashing.
NFR7: Application must NOT make any network requests other than checking GitHub for updates.
NFR8: No user data or analytics shall be collected or transmitted (Zero Telemetry).
NFR9: **Windows:** Support Windows 10 and 11.
NFR10: **Linux:** Support standard glibc-based distros (target Ubuntu 22.04 LTS as baseline).
NFR11: **macOS:** Support macOS 12 (Monterey) and newer (Apple Silicon native & Intel).

### Additional Requirements

- **Starter Template:** Initialize using `create-tauri-app --template vanilla-ts` (Vanilla TS + Rust).
- **Persistence:** Use raw Rust `fs` + `serde_json` for settings storage.
- **Notifications:** Implementation must use `tauri-plugin-notification`.
- **Auto-Start:** Implementation must use `tauri-plugin-autostart`.
- **Logging:** Implement file-based logging using `log` + `env_logger`.
- **API Bridge:** All Rust structs sent to frontend must use `#[serde(rename_all = "camelCase")]`.
- **Command Naming:** Frontend invoke strings must match Rust function names (snake_case).
- **Events:** Use kebab-case for event names (e.g., `battery-level-changed`).
- **Error Handling:** Backend commands must return `Result<T, String>`.

### FR Coverage Map

- **Epic 1 (Foundation):** FR1, FR2, FR3, FR12, FR13, FR14, FR15
- **Epic 2 (Notifications):** FR4, FR5, FR6, FR7
- **Epic 3 (Settings):** FR8, FR9, FR10, FR11
- **Epic 4 (Updates):** FR16, FR17

## Epic List

### Epic 1: System Tray & Battery Monitoring Foundation

**Goal:** The application runs silently in the background, auto-starts, and reliably tracks battery state.
**User Value:** "I install it, and I know it's running and watching."
**FRs Covered:** FR1, FR2, FR3, FR12, FR13, FR14, FR15

### Story 1.1: Project Initialization & Rust Logging

As a Developer,
I want to initialize the project with the correct stack and logging,
So that I have a stable foundation for building features.

**Acceptance Criteria:**

**Given** The developer has Node.js and Rust installed
**When** I run the `create-tauri-app` command with the `vanilla-ts` template
**Then** A new project directory is created with `src-tauri` (Rust) and `src` (TypeScript) folders
**And** I can run `npm run tauri dev` to see the default greeting
**And** The `log` and `env_logger` crates are added to `Cargo.toml`
**And** Running the app outputs logs to the terminal/file

### Story 1.2: Battery State Polling (Rust)

As a User,
I want the system to check my battery level in the background,
So that it knows when to trigger alerts without me checking manually.

**Acceptance Criteria:**

**Given** The app is running in the background
**When** The polling interval (e.g., 60 seconds) elapses
**Then** The Rust backend reads the current battery percentage using `system_stat` or `battery` crate
**And** The current percentage and charging status are logged to the console
**And** The loop repeats indefinitely while the app is running

### Story 1.3: Basic System Tray

As a User,
I want to see an icon in the system tray,
So that I know the app is running and can interact with it.

**Acceptance Criteria:**

**Given** The app is launched
**When** The initialization completes
**Then** A battery icon appears in the OS System Tray
**And** Left-clicking the icon does nothing (for now, or opens empty window)
**And** Right-clicking the icon shows a context menu with "Quit"
**And** Clicking "Quit" terminates the application process completely

### Story 1.4: Auto-Start Configuration

As a User,
I want the app to start automatically when I turn on my computer,
So that I don't have to remember to launch it every day.

**Acceptance Criteria:**

**Given** The app is installed and running
**When** I restart my computer and log in
**Then** The application process starts automatically in the background
**And** The System Tray icon appears without user intervention
**And** The implementation uses `tauri-plugin-autostart`

### Epic 2: The Critical Notification System

**Goal:** The user is alerted via native OS notifications when the hardcoded threshold (100%) is reached.
**User Value:** "It actually warns me now."
**FRs Covered:** FR4, FR5, FR6, FR7

### Story 2.1: Threshold Logic & Event Emission

As a Developer,
I want the backend to detect when the battery crosses the threshold,
So that I can trigger downstream actions like notifications.

**Acceptance Criteria:**

**Given** The battery polling loop is running
**When** Unfortunately, the battery level is >= 100% (hardcoded for now)
**And** The status is "Charging" or "Full"
**Then** An internal event or flag is raised indicating "Threshold Reached"
**And** The logic handles edge cases (e.g., already at 100% when app starts)

### Story 2.2: Native Notification Trigger

As a User,
I want to see a visible notification from my OS when the battery is full,
So that I notice it even if I'm looking at another window.

**Acceptance Criteria:**

**Given** The "Threshold Reached" event has fired
**When** The notification logic executes
**Then** A native system notification appears (Toast on Windows, Banner on Mac/Linux)
**And** The title says "Battery Full"
**And** The body says "Your battery is at [X]%. Unplug now to save health."
**And** The system "Critical/Alarm" sound plays (if supported by OS/crate)

### Story 2.3: Cool-down & Dismissal Logic

As a User,
I want the app to stop nagging me if I've already seen the alert,
So that I don't get spammed every 60 seconds.

**Acceptance Criteria:**

**Given** A notification has just been sent
**When** The next polling cycle occurs (e.g., 1 min later) and battery is still 100%
**Then** NO new notification is sent (Cool-down active)
**But** If I unplug the charger (Discharging), the cool-down resets
**And** If I plug it back in later, native notifications are re-enabled

### Epic 3: User Configuration & Settings

**Goal:** The user can open a UI to customize the threshold and settings persist across restarts.
**User Value:** "I can control it."
**FRs Covered:** FR8, FR9, FR10, FR11

### Story 3.1: Settings Persistence (Rust)

As a Developer,
I want to save user preferences to disk,
So that they don't have to re-configure the app every time it restarts.

**Acceptance Criteria:**

**Given** The app is running
**When** I save a `Settings` struct (threshold: u8, enabled: bool)
**Then** It is serialized to JSON and written to the OS AppConfig directory
**And** On app startup, the file is read and deserialized back into the struct
**And** If the file doesn't exist, default settings (100% threshold) are used

### Story 3.2: The "Serde Bridge" & API Commands

As a Developer,
I want to expose Rust functions to the frontend with consistent naming,
So that I can call them easily from TypeScript without naming conflicts.

**Acceptance Criteria:**

**Given** The Rust backend commands `get_settings` and `save_settings`
**When** They are exposed to Tauri
**Then** The return structs use `#[serde(rename_all = "camelCase")]`
**And** I can invoke them from the frontend using the exact snake_case names
**And** Data flows correctly in both directions (Rust <-> TS)

### Story 3.3: Settings UI (Vanilla TS)

As a User,
I want a simple menu to change the battery threshold,
So that I can customize when I get alerted.

**Acceptance Criteria:**

**Given** I click the "Settings" or "Open" option in the Tray
**When** The window opens
**Then** I see a clean HTML form with a Number Input (or Slider) for "Threshold %"
**And** I see a "Save" button
**And** The UI is lightweight (no heavy framework execution)

### Story 3.4: Connect UI to Backend

As a User,
I want my changes in the UI to actually save,
So that the app remembers what I picked.

**Acceptance Criteria:**

**Given** The Settings UI is open
**When** I change the threshold and click "Save"
**Then** The frontend calls `invoke('save_settings', ...)`
**And** The backend confirms the save
**And** The polling loop immediately uses the new threshold
**And** The UI shows a brief "Saved" confirmation

### Epic 4: Update Infrastructure

**Goal:** The system self-checks for updates to ensure longevity.
**User Value:** "It stays up to date."
**FRs Covered:** FR16, FR17

### Story 4.1: GitHub Actions & Updater Config

As a Developer,
I want to automate the build and release process,
So that I don't have to manually compile binaries for every OS.

**Acceptance Criteria:**

**Given** I push a tag starting with `v*` to the main branch
**When** The GitHub Action triggers
**Then** It builds the Tauri app for Windows (and other targets)
**And** It creates a GitHub Release draft with the artifacts
**And** The `tauri.conf.json` is configured with the public key and update endpoint

### Story 4.2: Update Check & Notification

As a User,
I want to know when a new version is available,
So that I can get the latest features and bug fixes.

**Acceptance Criteria:**

**Given** The app launches
**When** It performs the background update check
**Then** If a new version is found, a native notification appears: "Update Available"
**And** Clicking the notification opens the GitHub Releases page (or starts download if configured)
**And** If up to date, it does nothing silently
