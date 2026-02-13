# Story 3.3: Settings UI (Vanilla TS)

Status: done

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a User,
I want a simple menu to change the battery threshold,
So that I can customize when I get alerted.

## Acceptance Criteria

- [ ] **Given** The app window is open
- [ ] **When** I see the UI
- [ ] **Then** It displays the current threshold (loaded from backend)
- [ ] **And** It displays an "Enabled" checkbox
- [ ] **And** When I change values and click "Save", it persists to disk via the backend API
- [ ] **And** The UI is clean and follows the "High Aesthetics" rule (Dark mode, glassmorphism if possible, or at least clean CSS).

## Dev Notes

- **Tech Stack:** Vanilla Typescript + Vite (assumed).
- **Files:**
    - `index.html`: Main layout.
    - `src/main.ts`: Logic to call `invoke('get_app_settings')` and `invoke('save_app_settings')`.
    - `src/styles.css`: Styling.
- **Design:**
    - Centered card.
    - Title: "Configuración".
    - Slider (Range Input) for Threshold (50-100%).
    - Toggle Switch for "Activar Notificaciones".
    - "Guardar" Button with visual feedback (loading/success).

## Tasks

- [ ] Initialize Workflow <!-- id: 110 -->
- [x] Create UI Structure (HTML) <!-- id: 111 -->
- [x] Implement Logic (TS) <!-- id: 112 -->
- [x] Apply Styling (CSS) <!-- id: 113 -->
- [x] Verify End-to-End <!-- id: 114 -->
- [x] Update Story Status <!-- id: 115 -->
