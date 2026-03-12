# Guía Paso a Paso: Apple Developer Program desde Windows

Afortunadamente, **SÍ es posible realizar todo el proceso desde un PC con Windows**, con algunas salvedades importantes respecto a la generación de certificados que podemos solucionar.

Esta guía detalla el proceso completo para registrarte, obtener las llaves y configurar GitHub Actions para compilar y firmar tu aplicación Tauri y subirla a la Mac App Store.

> [!WARNING]
> La compilación en sí misma para macOS debe realizarse en un entorno macOS. GitHub Actions provee máquinas macOS (`macos-latest`), por lo que no necesitas tener una Mac física para compilar, solo para probar localmente.

---

## Parte 1: Inscribirse en el Apple Developer Program

1.  **Crear un Apple ID:** Si no tienes uno, ve a [appleid.apple.com](https://appleid.apple.com/) y créalo. Necesitarás activar la Autenticación de Dos Factores (2FA).
2.  **Descargar la App de Apple Developer (Opcional pero Recomendado):** Si tienes un iPhone o iPad, el proceso de inscripción es mucho más rápido y sencillo verificando tu identidad desde la aplicación _Apple Developer_ en el dispositivo.
3.  **Inscripción vía Web (Si no tienes dispositivo Apple):**
    - Ve a [developer.apple.com/programs/](https://developer.apple.com/programs/).
    - Haz clic en "Enroll" (Inscribirse).
    - Inicia sesión con tu Apple ID.
    - Completa el formulario (si eres individuo o empresa). Si eres individuo, solo necesitas tu información personal y tarjeta de crédito. Si eres empresa, necesitarás un número D-U-N-S.
    - Paga los $99 USD anuales. La activación de la cuenta puede tardar de unas horas a un par de días.

---

## Parte 2: Generar y Obtener los Certificados desde Windows

Normalmente, Apple asume que usas la app `Keychain Access` en una Mac para generar los certificados. Al estar en Windows, usaremos **OpenSSL** y la página web de Apple Developer.

### Paso 2.1: Instalar OpenSSL en Windows

Si tienes Git instalado (que seguramente sí para tu proyecto), ya tienes OpenSSL en la terminal "Git Bash". Si no, puedes instalarlo descargando [Git for Windows](https://gitforwindows.org/) o un binario de OpenSSL. Abre **Git Bash** para los siguientes comandos.

### Paso 2.2: Generar un Certificate Signing Request (CSR) y Llave Privada

En tu Git Bash, ejecuta:

```bash
openssl genrsa -out mi_llave_privada.key 2048
openssl req -new -key mi_llave_privada.key -out peticion_certificado.csr
```

- Te pedirá algunos datos (País, Estado, etc.). Puedes dejarlos en blanco u omitirlos pulsando Enter, no son relevantes para este tipo de certificado, excepto el 'Common Name' (pon tu nombre o empresa).

### Paso 2.3: Solicitar los Certificados en Apple Developer

1.  Ve al portal [Certificates, Identifiers & Profiles](https://developer.apple.com/account/resources/certificates/list).
2.  Haz clic en el botón `+` para crear un nuevo certificado.
3.  **Para distribuir a la Mac App Store:** Necesitas generar DOS certificados:
    - `Mac App Distribution`: Para firmar la aplicación.
    - `Mac Installer Distribution`: Para firmar el instalador `.pkg` que se sube a la tienda.
4.  **Para distribución externa (fuera de la tienda, ej. archivo .dmg normal para descargar de tu web):**
    - Selecciona `Developer ID Application`.
5.  Sigue los pasos y cuando te pida el `CSR`, sube el archivo `peticion_certificado.csr` que generaste en el Paso 2.2.
6.  Descarga el certificado generado (será un archivo `.cer`, por ejemplo, `mac_app.cer`).

### Paso 2.4: Convertir el Certificado para GitHub Actions

GitHub Actions (y Tauri) necesitan un archivo `.p12` que contenga tanto el certificado como tu llave privada.
Ejecuta esto en Git Bash, combinando el `.cer` descargado y el `.key` original:

```bash
# Convierte el .cer a formato .pem
openssl x509 -inform der -in mac_app.cer -out mac_app.pem

# Crea el .p12 combinando el .pem y tu llave privada (Importante usar -legacy para compatibilidad con macOS)
openssl pkcs12 -export -legacy -in mac_app.pem -inkey mi_llave_privada.key -out certificado_final.p12 -name "Mac App Certificate"
```

- **IMPORTANTE:** Te pedirá crear una contraseña de exportación para el `.p12`. Anótala, la necesitarás para GitHub Actions y para Tauri.
- Repite el proceso si generaste múltiples certificados (App y Installer).

---

## Parte 3: Generar Llaves de API (App Store Connect API)

Para que GitHub Actions pueda firmar la app en los servidores de Apple (`notarization`) o subir la app a la App Store Connect de manera automatizada, necesitas una API Key.

1.  Ve a [App Store Connect](https://appstoreconnect.apple.com/) e inicia sesión.
2.  Ve a **Users and Access** -> Pestaña **Keys**.
3.  Haz clic en el botón `+` para generar una nueva API Key.
4.  Dale un nombre (ej. "GitHub Actions API Key") y dale permisos de `App Manager` o `Admin`.
5.  Una vez creada, **descarga el archivo de la llave `.p8`**. Solo puedes descargarlo una vez de forma segura. Guárdalo bien.
6.  Anota el **Issuer ID** (aparece arriba en la página) y el **Key ID** (aparece junto a la llave que recién creaste).

---

## Parte 4: Configurar los Detalles de la Aplicación

1.  **Register an App ID (Identificador):**
    - En [Certificates, Identifiers & Profiles](https://developer.apple.com/account/resources/identifiers/list), crea un nuevo ID de App (App ID).
    - El **Bundle ID** es crucial. Debe coincidir exactamente con el campo `identifier` en tu [tauri.conf.json](file:///d:/Progra/Proyectos_personales/Bateria-al-100/src-tauri/tauri.conf.json) (ejemplo: `com.sukushaing.bateriaalcien`). Ojo: No puedes usar `com.tauri.build`.
2.  **App Store Connect:**
    - Ve a **My Apps** en App Store Connect y crea una nueva aplicación.
    - Selecciona macOS y asóciala al Bundle ID que creaste en el paso anterior. (Este paso es necesario para la automatización, incluso si solo distribuyes fuera de la Mac App Store, el backend los asocia).

---

## Parte 5: Configuración de Tauri ([tauri.conf.json](file:///d:/Progra/Proyectos_personales/Bateria-al-100/src-tauri/tauri.conf.json))

Debemos preparar a Tauri para la compilación en Mac. Abre el archivo de tu proyecto y asegúrate de estos valores:

```json
{
    "tauri": {
        "bundle": {
            "identifier": "com.sukushaing.bateriaalcien", // <-- DEBE COINCIDIR CON EL APP ID DE APPLE
            "macOS": {
                "entitlements": "./entitlements.mac.plist",
                "exceptionDomain": "",
                "frameworks": [],
                "providerShortName": null,
                "signingIdentity": null // Opcional, pero se maneja mejor en variables de entorno local
            }
        }
    }
}
```

Es muy común necesitar un archivo de "entitlements" genérico (permisos). Crea un archivo llamado [entitlements.mac.plist](file:///d:/Progra/Proyectos_personales/Bateria-al-100/src-tauri/entitlements.mac.plist) en la raíz de tu proyecto (o carpeta src-tauri):

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.app-sandbox</key>
    <false/>
</dict>
</plist>
```

---

## Parte 6: Integrar con GitHub Actions

1. Ve a tu repositorio en GitHub -> **Settings** -> **Secrets and variables** -> **Actions**.
2. Necesitamos convertir el archivo `.p12` que creaste en el Paso 2 en texto Base64 para guardarlo en un Secret porque GitHub Secrets no acepta archivos binarios:
   Abre Git Bash y ejecuta:
    ```bash
    base64 -i certificado_final.p12 > cert_base64.txt
    ```
3. Añade los siguientes **Repository Secrets**:
    - `APPLE_CERTIFICATE_BASE64` (Pega el contenido del `cert_base64.txt`).
    - `APPLE_CERTIFICATE_PASSWORD` (La contraseña que pusiste al crear el `.p12`).
    - `APPLE_API_ISSUER` (El Issuer ID del Paso 3).
    - `APPLE_API_KEY` (El Key ID del Paso 3).
    - `APPLE_API_KEY_BASE64` (Abre el archivo `.p8` descargado en el Paso 3, cópialo, y hazle `base64` igual que con el certificado, o pega el contenido tal cual dependiendo del Action. Generalmente Tauri recomienda base64 de la llave raw y la ruta).

    _Variables específicas para notarización/firma directa de Tauri:_
    - `APPLE_ID` (Tu usuario o correo del Apple ID - **Alternativa a la API Key, a veces requerido por ALTOOL**).
    - `APPLE_PASSWORD` (Tu "App-Specific password" generada en [appleid.apple.com](https://appleid.apple.com), en la sección seguridad. **OJO: NO tu password de iCloud**).
    - `APPLE_TEAM_ID` (Lo encuentras en tu Apple Developer Membership d154. **Modifica tu archivo Workflow de Release de Tauri (Ej. [.github/workflows/release.yml](file:///d:/Progra/Proyectos_personales/Bateria-al-100/.github/workflows/release.yml))**:

El "Action" oficial de Tauri (`tauri-apps/tauri-action`) es inteligente y automáticamente tomará las llaves del entorno para firmar los binarios de macOS.
Deberás añadir estas variables en el bloque `env` de tu paso de construcción. Aquí hay un ejemplo final:

```yaml
steps:
    - uses: actions/checkout@v4

    # [Tus otros pasos de setup de Rust y Node]...

    - name: Usar Tauri Action
      uses: tauri-apps/tauri-action@v0
      env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          # Variables de Entorno para Firma y Notarización de macOS
          APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE_BASE64 }}
          APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
          APPLE_API_ISSUER: ${{ secrets.APPLE_API_ISSUER }}
          APPLE_API_KEY: ${{ secrets.APPLE_API_KEY }}
          APPLE_API_KEY_BASE64: ${{ secrets.APPLE_API_KEY_BASE64 }}
```

## Resumen del Flujo de Trabajo

1.  Pagas tu membresía ($99).
2.  Generas un CSR con Git Bash (Windows).
3.  Solicitas un _Developer ID Certificate_ en portal de Apple, descargas `.cer` y creas un `.p12` usando la flag `-legacy`.
4.  Subes el `.p12` codificado en base64 a GitHub como Secret, junto a la contraseña, y tu App Store Connect API Key.
5.  Actualizas [tauri.conf.json](file:///d:/Progra/Proyectos_personales/Bateria-al-100/src-tauri/tauri.conf.json) con tu Bundle ID y el archivo de Entitlements de Mac (`com.apple.security.app-sandbox` en false).
6.  Github Actions lanza un job sobre sistema Mac, le pasas las variables de entorno de Apple al Action oficial de Tauri.
7.  Tauri construye, invoca en secreto las herramientas de seguridad de macOS pasándoles tus variables, firma el `.dmg` y lo sube a notarizar a los servidores de Apple.
8.  Al completarse, el `.dmg` final descargable ya no mostrará el mensaje _"App está Dañada"_.` en los "Releases" vendrá firmado. Ya no dirá _"App está Dañada"_.

_(Para subir directo a la Mac App Store es un procedimiento similar donde cambias el tipo de certificado a 'Mac App Distribution' al principio, y subes el `.pkg` en lugar del `.dmg` usando Transporter Desktop CLI)._
