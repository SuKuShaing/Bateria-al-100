<p align="center">
  <img src="src-tauri/icons/128x128.png" alt="Batería al 100 Logo" width="128"/>
</p>
<h1 align="center">Batería al 100</h1>

<p align="center">
  <a href="https://github.com/SuKuShaing/Bateria-al-100/releases">
    <img src="https://img.shields.io/github/downloads/SuKuShaing/Bateria-al-100/total?logo=github&color=blue" alt="Total Downloads">
  </a>
</p>

<p align="center">
  <strong>Cuida la salud de tu batería.</strong>
  <br />
  <em>Cree una aplicación de escritorio muy ligera, rápida y silenciosa que te avisa cuando tu dispositivo está cargado, previniendo el desgaste prematuro de la batería.</em>
  <br />
  <em>La hice porque quiero cuidar mi batería y alargar al máximo Su vida útil</em>
  <br />
</p>

---

<strong>🌐 Sitio Web Oficial: <a href="https://sukushaing.github.io/Bateria-al-100/">https://sukushaing.github.io/Bateria-al-100/</a></strong>

---

## 📥 Descargas e Instalación

Descarga la última versión para tu sistema operativo desde la sección de **Releases**:

- **🪟 Windows (10/11):**<br><a href="https://apps.microsoft.com/detail/9NKTXXC7PJHR" target="_blank"><img src="https://get.microsoft.com/images/en-us%20dark.svg" width="160" alt="Get it from Microsoft"/></a><br><br>_Instaladores directos (GitHub Releases):_ [⬇️ Descargar .exe](https://github.com/SuKuShaing/Bateria-al-100/releases/latest/download/BateriaAlCien_0.1.16_x64-setup.exe) / [⬇️ Descargar .msi](https://github.com/SuKuShaing/Bateria-al-100/releases/latest/download/BateriaAlCien_0.1.16_x64_en-US.msi)

- **🍎 macOS (Apple Silicon / Intel):** [⬇️ Descargar .dmg](https://github.com/SuKuShaing/Bateria-al-100/releases/latest/download/BateriaAlCien_0.1.16_aarch64.dmg)
- **🐧 Linux (Ubuntu/Debian):** [⬇️ Descargar .deb](https://github.com/SuKuShaing/Bateria-al-100/releases/latest/download/BateriaAlCien_0.1.16_amd64.deb) / [⬇️ Descargar AppImage](https://github.com/SuKuShaing/Bateria-al-100/releases/latest/download/BateriaAlCien_0.1.16_amd64.AppImage)

> **Nota para Windows:** Al no contar con un certificado de firma digital de pago, es posible que Windows SmartScreen muestre una advertencia de "Editor Desconocido" al instalar. Haz clic en **"Más información"** y luego en **"Ejecutar de todas formas"**.
> <u>_Para evitar este mensaje de advertencia recomendamos instalar desde la Microsoft Store_</u>

---

## 🔋 Por qué monitorear la batería al 100% es útil

Las baterías de iones de litio en laptops se degradan más rápido cuando se mantienen constantemente al 100% de carga, incluso si el equipo usa corriente directa (bypass de batería). Esta aplicación te avisa para que desconectes a tiempo y evites ese estrés químico innecesario.

### 🔬 Explicación técnica breve

- **Funcionamiento del BMS:** Al enchufar el equipo y llegar al 100%, el _Battery Management System (BMS)_ desconecta la batería del circuito principal y usa la energía directa del adaptador. Aunque no hay ciclos de carga/descarga, las celdas quedan bajo alta tensión constante (4.2V por celda), lo que acelera reacciones químicas adversas como la formación de SEI (_Solid Electrolyte Interphase_).
- **Degradación:** Múltiples estudios muestran que mantener la batería al 100% reduce la capacidad máxima en ~20% durante 1 año, en comparación con mantenerla en rangos del 20-80%. El calor generado por tareas exigentes (juegos, renderizado) empeora esta degradación de 2 a 3 veces más rápido.
- **Recomendación de fabricantes:** Marcas como Lenovo, Dell y HP sugieren limitar la carga al 80% para uso de "escritorio". En la industria automotriz, empresas como Apple o Tesla usan límites del 80% por defecto en sus vehículos eléctricos (EVs).

### ✨ Beneficios de usar esta app

- ✅ **Evita la degradación prematura:** Extiende la vida útil de tu batería entre un +30% y +50%.
- ✅ **Notificaciones oportunas:** Te avisa exactamente cuando llega al 100% (o al límite configurado).
- ✅ **Totalmente compatible:** Funciona en Windows, macOS y Linux (utiliza `acpi` o WMI).
- ✅ **Configurable:** Establece tu propio umbral de carga límite y el intervalo de monitoreo.
- ✅ **Zero-Overhead:** Chequea el estado cada 100 segundos en segundo plano, sin impacto en el rendimiento.

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
