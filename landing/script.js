import { osIcons } from "./js/icons.js";

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
            icon: osIcons.win,
            suffix: "Descargar para Windows",
            url: "https://apps.microsoft.com/detail/9NKTXXC7PJHR",
        },
        {
            os: "Mac",
            id: "mac",
            icon: osIcons.mac,
            suffix: "Descargar para Mac",
            file: "BateriaAlCien_0.1.14_aarch64.dmg",
        },
        {
            os: "Linux",
            id: "linux",
            icon: osIcons.linux,
            suffix: "Descargar para Linux",
            file: "BateriaAlCien_0.1.14_amd64.AppImage",
        },
    ];

    function detectOS() {
        try {
            const userAgent = (
                window.navigator.userAgent ||
                window.navigator.vendor ||
                window.opera ||
                ""
            ).toLowerCase();

            if (userAgent.includes("win")) return "win";
            if (userAgent.includes("mac")) return "mac";
            if (userAgent.includes("linux") || userAgent.includes("x11"))
                return "linux";

            // Si es Android, iOS, o no se reconoce, fuerza Windows por defecto
            return "win";
        } catch (error) {
            // Fallback a Windows en caso de que no se logre detectar o haya un error
            return "win";
        }
    }

    const currentOsId = detectOS();

    // Configurar el botón principal
    const primaryOption = downloads.find((d) => d.id === currentOsId);
    const primaryBtn = document.getElementById("primary-btn");
    const primaryBtnText = document.getElementById("primary-btn-text");

    if (primaryBtn && primaryOption) {
        primaryBtn.href = primaryOption.url || (baseUrl + primaryOption.file);
        primaryBtnText.textContent = primaryOption.suffix;
        primaryBtn.querySelector(".btn-icon").innerHTML = primaryOption.icon;
    }

    // Configurar imagen de notificación dinámica
    const notificationImg = document.getElementById("notification-img");
    if (notificationImg) {
        if (currentOsId === "mac") {
            notificationImg.src = "img/Notificación Macos.webp";
        } else if (currentOsId === "linux") {
            notificationImg.src = "img/Notificación Linux.webp";
        } else {
            notificationImg.src = "img/Notificación Windows.webp";
        }
    }

    // Configurar el menú desplegable (las demás opciones)
    const dropdownMenu = document.getElementById("dropdown-menu");
    const otherOptions = downloads.filter((d) => d.id !== currentOsId);

    if (dropdownMenu) {
        otherOptions.forEach((opt) => {
            const link = document.createElement("a");
            link.href = opt.url || (baseUrl + opt.file);
            link.className = "dropdown-item";
            link.target = "_blank";

            const btnText = opt.url 
                ? "Descargar de Microsoft Store" 
                : `Descargar ${opt.file.split(".").pop().toUpperCase()}`;

            link.innerHTML = `
                <span class="dropdown-item-icon">${opt.icon}</span>
                <span>${btnText}</span>
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
