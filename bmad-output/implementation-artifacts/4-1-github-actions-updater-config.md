# Story 4.1: GitHub Actions Updater Config

**Epic:** 4 - Auto-Updater & CI/CD
**Status:** Ready vs Dev

## Goal

Configure a GitHub Actions workflow to automatically build the Tauri application and release artifacts (Windows .exe/.msi + update signatures) whenever a new tag is pushed. This is the prerequisite for the auto-updater functionality.

## Context

Tauri's auto-updater relies on a release server or a public repository (like GitHub Releases) to host the update artifacts and the `latest.json` signature file. We need to set up the CI pipeline to generate these assets correctly using `tauri-action`.

## Functional Requirements

1.  **CI Pipeline:** A GitHub Action (`.github/workflows/release.yml`) that triggers on tag push (e.g., `v*`).
2.  **Build:** It must compile the Rust backend and JS frontend.
3.  **Signing:** It must support code signing (though for a personal project we might skip EV certs, Tauri requires a key for the _updater_ signature).
4.  **Release:** It must upload `bateria-al-cien_x.x.x_x64-setup.exe` and `latest.json` to GitHub Releases.

## Non-Functional Requirements

- **Security:** Updater keys must be stored in GitHub Secrets (`TAURI_PRIVATE_KEY`, `TAURI_KEY_PASSWORD`).

## Implementation Plan

1.  **Generate Keys:** Run `tauri signer generate -w ~/.tauri/myapp.key` (local) to get the pub/priv keys.
2.  **Configure Update:** Update `tauri.conf.json` to enable the `updater` plugin and point to the GitHub releases URL.
3.  **Create Workflow:** Create `.github/workflows/release.yml` using the official Tauri action template.
4.  **Documentation:** clear instructions on how to set the secrets in the GitHub repo.

## Acceptance Criteria

- [ ] `tauri.conf.json` has updater configured with a public key.
- [ ] `.github/workflows/release.yml` exists and is valid.
- [ ] Keys are generated (Private key provided to user to save in Secrets).
