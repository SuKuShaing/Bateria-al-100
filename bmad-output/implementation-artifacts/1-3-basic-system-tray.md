# Story 1.3: Basic System Tray

Status: done

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a User,
I want to see an icon in the system tray,
so that I know the app is running and can interact with it.

## Acceptance Criteria

1. **Given** The app is launched
2. **When** The initialization completes
3. **Then** A battery icon appears in the OS System Tray
4. **And** Left-clicking the icon does nothing (for now, or opens empty window)
5. **And** Right-clicking the icon shows a context menu with "Quit"
6. **And** Clicking "Quit" terminates the application process completely

## Tasks / Subtasks

- [ ] Setup Tray Icon (AC: 1, 3)
    - [ ] Add `tauri-plugin-positioner` (optional) or use built-in `SystemTray`
    - [ ] Configure `tauri.conf.json` if needed for icons
    - [ ] Create icon files if missing (can use default Tauri icon for now)
- [ ] Implement Tray Menu (AC: 5, 6)
    - [ ] Create `src-tauri/src/modules/tray.rs`
    - [ ] Define `SystemTrayMenu` with "Quit" item
    - [ ] Attach menu to System Tray
- [ ] Handle Tray Events (AC: 4, 6)
    - [ ] Implement event handler for "Quit" (calls `app.exit(0)`)
    - [ ] Handle Left Click (Empty default or `window.show()`)
- [ ] Verify Implementation
    - [ ] Run `npm run tauri dev`
    - [ ] Check Tray Icon appearance
    - [ ] Verify context menu and Quit functionality

## Dev Notes

- **Tauri v2 API:** Tauri v2 uses `tauri::tray` module. It's slightly different from v1.
- **Reference:** Check Tauri v2 docs for `TrayIconBuilder`.
- **Architecture:** Continue using `modules` pattern. Add `tray.rs`.

### References

- [Architecture Decision Document](file:///d:/Progra/Proyectos_personales/Bateria-al-100/bmad-output/planning-artifacts/architecture.md)
- [Epic 1: Foundation](file:///d:/Progra/Proyectos_personales/Bateria-al-100/bmad-output/planning-artifacts/epics.md#Epic-1)

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
