<p align="center">
  <img src="src-tauri/icons/128x128.png" alt="Batería al 100 Logo" width="128"/>
</p>
<h1 align="center">Batería al 100</h1>

<p align="center">
  <strong>Cuida la salud de tu batería.</strong>
  <br />
  <em>Cree una aplicación de escritorio muy ligera, rápida y silenciosa que te avisa cuando tu dispositivo está cargado, previniendo el desgaste prematuro de la batería.</em>
  <br />
  <em>La hice porque quiero cuidar mi batería y alargar al máximo Su vida útil</em>
</p>

---

## 📥 Descargas e Instalación

Descarga la última versión para tu sistema operativo desde la sección de **Releases**:

- **🪟 Windows (10/11):** [⬇️ Descargar .exe](https://github.com/SuKuShaing/Bateria-al-100/releases/latest/download/bateriaAlCien_0.1.4_x64-setup.exe) / [⬇️ Descargar .msi](https://github.com/SuKuShaing/Bateria-al-100/releases/latest/download/bateriaAlCien_0.1.4_x64_en-US.msi)
- **🍎 macOS (Apple Silicon / Intel):** [⬇️ Descargar .dmg](https://github.com/SuKuShaing/Bateria-al-100/releases/latest/download/bateriaAlCien_0.1.4_aarch64.dmg)
- **🐧 Linux (Ubuntu/Debian):** [⬇️ Descargar .deb](https://github.com/SuKuShaing/Bateria-al-100/releases/latest/download/bateriaAlCien_0.1.4_amd64.deb) / [⬇️ Descargar AppImage](https://github.com/SuKuShaing/Bateria-al-100/releases/latest/download/bateriaAlCien_0.1.4_amd64.AppImage)

> **Nota para Windows:** Al no contar con un certificado de firma digital de pago, es posible que Windows SmartScreen muestre una advertencia de "Editor Desconocido" al instalar. Haz clic en **"Más información"** y luego en **"Ejecutar de todas formas"**.

---

## ⚡ Características

- **🔔 Notificaciones Inteligentes:** Recibe alertas nativas en tu sistema operativo cuando tu batería alcanza un nivel configurable (ej. 100%, 80%).
- **⏰ Recordatorios Escalonados:** Si ignoras el primer aviso, recibirás recordatorios a los 5, 10, 20 y 40 minutos para asegurar que desconectes tu equipo.
- **🤫 Operación Silenciosa:** Se ejecuta en segundo plano (System Tray / Área de Notificación) sin estorbar tu flujo de trabajo.
- **🚀 Inicio Automático:** Configurable para arrancar junto con tu sistema operativo.
- **🔄 Actualizaciones Automáticas:** Siempre tendrás la última versión gracias al sistema de Auto-Update integrado (vía GitHub Releases).
- **🌙 Diseño Premium:** Interfaz de usuario moderna, modo oscuro por defecto y efectos Glassmorphism.
- **💻 Multiplataforma:** Disponible para Windows, macOS y Linux.

---

## 🛠️ Tecnologías Utilizadas

Esta aplicación fue construida utilizando un stack moderno para garantizar máximo rendimiento y un bajo consumo de recursos (ideal para no gastar batería mientras la monitoreamos):

- **[Tauri v2](https://v2.tauri.app/):** Framework para construir aplicaciones de escritorio usando tecnologías web, con un backend en Rust de alto rendimiento.
- **[Rust](https://www.rust-lang.org/):** Backend hiper-rápido para el monitoreo del sistema (`starship-battery`), manejo de la bandeja del sistema, notificaciones y configuración.
- **[TypeScript](https://www.typescriptlang.org/) & [Vite](https://vitejs.dev/):** Frontend ultraligero sin frameworks pesados, solo Vanilla TypeScript y CSS.
- **GitHub Actions:** Pipeline CI/CD automatizado para compilación cruzada en Windows/Mac/Linux y generación de releases.

---

## 👨‍💻 Desarrollo

Si deseas clonar el repositorio y modificar la aplicación:

### Prerrequisitos

- [Node.js](https://nodejs.org/) (v16+)
- [Rust](https://www.rust-lang.org/) (stable)
- Dependencias del sistema operativo requeridas por Tauri (ver [Guía de Tauri](https://v2.tauri.app/start/prerequisites/))

### Instalación

1. Clona el repositorio:

    ```bash
    git clone https://github.com/SuKuShaing/Bateria-al-100.git
    cd Bateria-al-100
    ```

2. Instala las dependencias del frontend:

    ```bash
    npm install
    ```

3. Inicia el entorno de desarrollo:
    ```bash
    npm run tauri dev
    ```

### Compilar (Release)

Para generar los instaladores en tu máquina local:

```bash
npm run tauri build
```
