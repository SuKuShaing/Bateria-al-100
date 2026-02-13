---
stepsCompleted:
    [
        "step-01-init",
        "step-02-discovery",
        "step-03-success",
        "step-04-journeys",
        "step-05-domain",
        "step-06-innovation",
        "step-07-project-type",
        "step-08-scoping",
        "step-09-functional",
        "step-10-nonfunctional",
        "step-11-polish",
    ]
inputDocuments: ["product-brief-Batería-al-100-2026-02-12.md"]
workflowType: "prd"
---

# Product Requirements Document - Batería al 100

**Author:** Seba
**Date:** 2026-02-12

## 1. Project Classification

- **Project Type:** Desktop Application (Tauri)
- **Domain:** General Utility / System Tool
- **Complexity:** Low
- **Project Context:** Greenfield (New Project)

## 2. High-Level Definition

### Core Requirements

- **Offline First:** 100% local execution, no internet connection required.
- **Cross-Platform Base:** Rust-based backend (Tauri) to support Windows, macOS, and Linux, with initial focus on Windows.
- **Tray-Only Interface:** Application lives in the system tray. No persistent main window.
- **Critical Notifications:** Use native OS priority notifications (Critical/Alarm levels) to ensure user notices the alert.

## 3. Success Criteria

### User Success

- **Peace of Mind:** User trusts they will be notified before overcharging occurs, without constantly checking the battery icon.
- **Non-Intrusive:** The app is invisible (System Tray only) until it's needed (notification).
- **Control:** User can easily change the threshold (e.g., to 80%) if they decide to optimize for battery health.

### Business Success (Personal Project Context)

- **Daily Usage:** You use it every time you charge your laptop.
- **Reliability:** It never misses a high-charge event while running.

### Technical Success

- **Resource Efficiency:** Uses negligible RAM/CPU (Rust backend).
- **Battery API Accuracy:** Correctly reads charge level on Windows (and potentially Linux/Mac).

### Measurable Outcomes

- **Outcome 1:** Detection of 100% charge triggers notification within 1 minute.
- **Outcome 2:** Memory usage stays under 50MB (target).

## 4. Product Scope & Roadmap

### MVP Strategy

- **Approach:** "Install and Forget" Utility. The app works immediately upon installation with sensible defaults, requiring zero interaction unless the user wants to change settings.
- **Target Audience:** Users who want to preserve battery health or avoid overcharging without active monitoring.

### MVP Feature Set (Phase 1)

- **Core Capabilities:**
    - **Cross-Platform:** Windows, macOS, Linux (Ubuntu/Debian).
    - **Background Service:** Precise battery polling.
    - **System Tray App:** Icon with context menu (Open, Exit).
    - **Notification:** Native system alert when Battery Level >= Threshold.
    - **Configuration:** Set custom threshold (saved to disk).
    - **Auto-Start:** Enabled by default on installation.
    - **Updater:** Silent check against GitHub Releases.

### Roadmap (Post-MVP)

**Phase 2: Refinement (Growth)**

- **Low Battery Alert:** Notify when battery is low (e.g. 20%) to plugin.
- **Sound Customization:** User can select custom audio files for alerts.
- **Theme Support:** Dark/Light mode for Settings window.

**Phase 3: Intelligence (Vision)**

- **Battery Health Analytics:** Logs history of charge cycles (if user opts-in).
- **Smart Estimation:** "Time to Full" or "Time to Empty" estimates in tray tooltip.
- **Charge Limiter:** Integration with hardware/BIOS to stop charging (if supported).

### Risks & Mitigation

- **Technical Risk (Linux):** Battery API consistency across distros. _Mitigation: Focus on `upower` (standard) first, ignore niche setups for MVP._
- **UX Risk:** Notification spam. _Mitigation: Implement "Cool-down" logic so it doesn't notify every minute once threshold is reached._

## 5. User Journeys

### Journey 1: The Deep Work Session (Primary)

**Persona:** Seba, Developer.
**Goal:** Unplug immediately at 100% without breaking concentration.
**Narrative:** Seba plugs in his laptop at 15% and dives into code. He completely forgets about the battery. An hour later, a **native system alert** pops up: _"Battery Full (100%)"_ accompanied by the system critical sound. He notices it instantly, unplugs the cable, and keeps coding.

### Journey 2: The Battery Health Optimizer (Configuration)

**Persona:** Alex, Hardware Enthusiast.
**Goal:** Limit charge to 80% to extend battery lifespan.
**Narrative:** Alex installs the app. He sees the icon in the System Tray. He **clicks the icon**, and a small popover appears. He changes the default "100" to "80". He clicks away, and the popover closes. Later, when his battery hits 80%, he gets the notification to unplug.

### Journey 3: The "Do Not Disturb" User (Edge Case)

**Persona:** Sarah, Library Worker.
**Goal:** Get notified without making noise.
**Narrative:** Sarah is in a quiet zone. She relies on Windows "Focus Assist" or Notification settings. She configures the app's notification to be "Silent" in Windows settings. When the battery hits 100%, the notification **slides in visually** but does not make a sound, respecting her system preferences.

### Journey Requirements Summary

- **Background Service:** Continuous battery polling.
- **System Tray Integration:** Interactable icon (click to open).
- **Popover UI:** Lightweight configuration interface.
- **Settings Persistence:** Save/load threshold preference.
- **Native Notification:** Use OS notification system (critical level).
- **Audio Control:** Trigger system sounds associated with notifications.
- **OS Respect:** Adhere to Do Not Disturb / Focus Assist states.

## 6. Technical Constraints & Architecture

### Project Overview

- **Type:** Native Desktop Utility (Tauri).
- **Core Architecture:** Rust backend for performance/system access, Web frontend for UI (Settings).

### Platform & Deployment

- **Platform Support (MVP):**
    - **Windows:** 10/11 (MSI/NSIS).
    - **macOS:** Universal Binary (DMG).
    - **Linux:** Ubuntu/Debian (DEB).
- **Update Mechanism:**
    - **Source:** GitHub Releases.
    - **Behavior:** Silent background check.
    - **Infrastructure:** Public GitHub repository (Serverless).

### Implementation Constraints

- **System Integration:**
    - **Tray:** Must support right-click (Exit) and left-click (Open UI).
    - **Auto-Start:** Launch on OS login enabled by default.
- **Data & Privacy:**
    - **Persistence:** Local JSON config file.
    - **Privacy:** Zero telemetry. No user data storage.

## 7. Functional Requirements

### Battery Monitoring

- **FR1:** System can read the current battery charge percentage level from the OS.
- **FR2:** System can read the current charging status (Plugged In / Discharging).
- **FR3:** System can poll battery status at a fixed interval (every 60-100 seconds).

### Notification System

- **FR4:** User can receive a native OS notification when battery level >= configured threshold.
- **FR5:** System will play the "Critical/Alarm" system sound when triggering a notification.
- **FR6:** System will prevent notification spam (implementing a cool-down period after first alert).
- **FR7:** Notification will automatically dismiss if the user unplugs the charger.

### User Configuration

- **FR8:** User can open a Settings UI from the System Tray.
- **FR9:** User can set a custom battery percentage threshold (default: 100%).
- **FR10:** System must persist user configuration to disk (survives restarts).
- **FR11:** User defaults to 100% threshold if no configuration exists.

### System Integration

- **FR12:** System runs as a background process with no taskbar presence (System Tray only).
- **FR13:** User can interact with a Tray Icon to see quick status or open menu.
- **FR14:** User can Right-Click Tray Icon to access "Exit" option.
- **FR15:** System auto-starts on OS login (enabled by default).

### Updates & Maintenance

- **FR16:** System can check for updates against GitHub Releases in the background.
- **FR17:** User is notified if a new version is available for download.

## 8. Non-Functional Requirements

### Performance & Efficiency

- **NFR1:** Memory footprint should target **< 50MB RAM** when idle in the tray.
- **NFR2:** CPU usage should be negligible (**< 0.1%**) during idle polling states.
- **NFR3:** App startup time (to tray ready) should be **< 2 seconds**.

### Reliability & Stability

- **NFR4:** System must run continuously for **7+ days** without crashing or leaking memory.
- **NFR5:** System must correctly resume polling after the computer wakes from Sleep/Hibernate.
- **NFR6:** System must handle "Missing Battery" states (e.g., desktops or hardware error) gracefully without crashing.

### Privacy & Security

- **NFR7:** Application must NOT make any network requests other than checking GitHub for updates.
- **NFR8:** No user data or analytics shall be collected or transmitted (Zero Telemetry).

### Compatibility

- **NFR9:** **Windows:** Support Windows 10 and 11.
- **NFR10:** **Linux:** Support standard glibc-based distros (target Ubuntu 22.04 LTS as baseline).
- **NFR11:** **macOS:** Support macOS 12 (Monterey) and newer (Apple Silicon native & Intel).
