document.addEventListener("DOMContentLoaded", () => {
    // --- 1. Scroll Animations (Intersection Observer) ---
    const observerOptions = {
        root: null,
        rootMargin: "0px",
        threshold: 0.1,
    };

    const observer = new IntersectionObserver((entries, observer) => {
        entries.forEach((entry) => {
            if (entry.isIntersecting) {
                entry.target.classList.add("visible");
                observer.unobserve(entry.target); // Animates only once
            }
        });
    }, observerOptions);

    document.querySelectorAll(".animate-up").forEach((el) => {
        observer.observe(el);
    });

    // --- 2. Battery Animation Logic ---
    const batteryWrapper = document.getElementById("battery-anim");

    if (batteryWrapper) {
        // Start animation shortly after load
        setTimeout(() => {
            batteryWrapper.classList.add("full");
        }, 500);
    }

    // --- 3. OS Detection Logic ---

    // Definimos las descargas disponibles (URLs de ejemplo, deben reemplazarse por las reales cuando existan releases)
    // Asumimos el esquema: app/repo/releases/latest/download/...
    const baseUrl =
        "https://github.com/SuKuShaing/Bateria-al-100/releases/latest/download/";

    const downloads = [
        {
            os: "Windows",
            id: "win",
            icon: "🪟",
            suffix: "Descargar para Windows",
            file: "bateriaAlCien_0.1.10_x64-setup.exe",
        },
        {
            os: "Mac",
            id: "mac",
            icon: "🍎",
            suffix: "Descargar para Mac",
            file: "bateriaAlCien_0.1.10_aarch64.dmg",
        },
        {
            os: "Linux",
            id: "linux",
            icon: "🐧",
            suffix: "Descargar para Linux",
            file: "bateriaAlCien_0.1.10_amd64.AppImage",
        },
    ];

    function detectOS() {
        const userAgent = window.navigator.userAgent.toLowerCase();

        if (userAgent.includes("win")) return "win";
        if (userAgent.includes("mac")) return "mac";
        if (userAgent.includes("linux") || userAgent.includes("x11"))
            return "linux";

        return "win"; // Fallback a Windows por defecto
    }

    const currentOsId = detectOS();

    // Configurar el botón principal
    const primaryOption = downloads.find((d) => d.id === currentOsId);
    const primaryBtn = document.getElementById("primary-btn");
    const primaryBtnText = document.getElementById("primary-btn-text");

    if (primaryBtn && primaryOption) {
        primaryBtn.href = baseUrl + primaryOption.file;
        primaryBtnText.textContent = primaryOption.suffix;
        primaryBtn.querySelector(".btn-icon").textContent = primaryOption.icon;
    }

    // Configurar el menú desplegable (las demás opciones)
    const dropdownMenu = document.getElementById("dropdown-menu");
    const otherOptions = downloads.filter((d) => d.id !== currentOsId);

    if (dropdownMenu) {
        otherOptions.forEach((opt) => {
            const link = document.createElement("a");
            link.href = baseUrl + opt.file;
            link.className = "dropdown-item";

            link.innerHTML = `
                <span class="dropdown-item-icon">${opt.icon}</span>
                <span>Descargar ${opt.file.split(".").pop().toUpperCase()}</span>
            `;

            dropdownMenu.appendChild(link);
        });
    }

    // --- 4. Dropdown Menu Toggle ---
    const dropdownToggle = document.getElementById("dropdown-toggle");
    const dropdown = document.querySelector(".dropdown");

    if (dropdownToggle && dropdown) {
        dropdownToggle.addEventListener("click", (e) => {
            e.preventDefault();
            dropdown.classList.toggle("active");
        });

        // Close when clicking outside
        document.addEventListener("click", (e) => {
            if (!dropdown.contains(e.target) && e.target !== dropdownToggle) {
                dropdown.classList.remove("active");
            }
        });
    }
});
