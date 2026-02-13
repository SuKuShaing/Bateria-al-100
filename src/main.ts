import { invoke } from "@tauri-apps/api/core";

interface AppSettings {
    threshold: number;
    enabled: boolean;
}

let thresholdSlider: HTMLInputElement | null;
let thresholdInput: HTMLInputElement | null;
let enabledToggle: HTMLInputElement | null;
let saveBtn: HTMLButtonElement | null;
let statusMsg: HTMLElement | null;

async function loadSettings() {
    try {
        const settings = await invoke<AppSettings>("get_app_settings");
        if (thresholdSlider)
            thresholdSlider.value = settings.threshold.toString();
        if (thresholdInput)
            thresholdInput.value = settings.threshold.toString();
        if (enabledToggle) enabledToggle.checked = settings.enabled;
    } catch (error) {
        console.error("Failed to load settings:", error);
    }
}

async function saveSettings() {
    if (!thresholdSlider || !enabledToggle || !saveBtn || !statusMsg) return;

    const newSettings: AppSettings = {
        threshold: parseInt(thresholdSlider.value),
        enabled: enabledToggle.checked,
    };

    saveBtn.disabled = true;
    saveBtn.textContent = "Guardando...";

    try {
        await invoke("save_app_settings", { newSettings });

        // UI Feedback
        statusMsg.textContent = "✔ Guardado correctamente";
        statusMsg.className = "status-text success";
        statusMsg.classList.remove("hidden");

        setTimeout(() => {
            statusMsg?.classList.add("hidden");
        }, 2000);
    } catch (error) {
        console.error("Failed to save:", error);
        statusMsg.textContent = "❌ Error al guardar";
        statusMsg.className = "status-text error";
        statusMsg.classList.remove("hidden");
    } finally {
        saveBtn.disabled = false;
        saveBtn.textContent = "Guardar Cambios";
    }
}

window.addEventListener("DOMContentLoaded", () => {
    thresholdSlider = document.querySelector("#threshold-slider");
    thresholdInput = document.querySelector("#threshold-input");
    enabledToggle = document.querySelector("#enabled-toggle");
    saveBtn = document.querySelector("#save-btn");
    statusMsg = document.querySelector("#status-msg");

    let lastValidValue = 100;

    // Initial Load
    loadSettings().then(() => {
        if (thresholdSlider) lastValidValue = parseInt(thresholdSlider.value);
    });

    // 1. Slider -> Input
    thresholdSlider?.addEventListener("input", (e) => {
        const val = (e.target as HTMLInputElement).value;
        if (thresholdInput) thresholdInput.value = val;
        lastValidValue = parseInt(val);
    });

    // 2. Input -> Slider (while typing)
    thresholdInput?.addEventListener("input", (e) => {
        let val = parseInt((e.target as HTMLInputElement).value);
        if (!isNaN(val)) {
            // Visual clamp for slider, but allow typing (e.g. "1")
            let clampedForSlider = Math.max(10, Math.min(100, val));
            if (thresholdSlider)
                thresholdSlider.value = clampedForSlider.toString();
        }
    });

    // 3. Input Blur (Validation & Revert)
    thresholdInput?.addEventListener("blur", (e) => {
        let val = parseInt((e.target as HTMLInputElement).value);

        if (isNaN(val) || val < 10 || val > 100) {
            // Invalid? Revert to last valid
            if (thresholdInput)
                thresholdInput.value = lastValidValue.toString();
            if (thresholdSlider)
                thresholdSlider.value = lastValidValue.toString();
        } else {
            // Valid? Update lastValid and force formatting
            lastValidValue = val;
            if (thresholdInput) thresholdInput.value = val.toString();
            if (thresholdSlider) thresholdSlider.value = val.toString();
        }
    });

    saveBtn?.addEventListener("click", () => {
        saveSettings();
    });
});
