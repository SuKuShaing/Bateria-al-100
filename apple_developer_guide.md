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
    - El **Bundle ID** es crucial. Debe coincidir exactamente con el campo `identifier` en tu `tauri.conf.json` (ejemplo: `com.sukushaing.bateriaalcien`). Ojo: No puedes usar `com.tauri.build`.
2.  **App Store Connect:**
    - Ve a **My Apps** en App Store Connect y crea una nueva aplicación.
    - Selecciona macOS y asóciala al Bundle ID que creaste en el paso anterior. (Este paso es necesario para la automatización, incluso si solo distribuyes fuera de la Mac App Store, el backend los asocia).

---

## Parte 5: Configuración de Tauri (`tauri.conf.json`)

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

Es muy común necesitar un archivo de "entitlements" genérico (permisos). Crea un archivo llamado `entitlements.mac.plist` en la raíz de tu proyecto (o carpeta src-tauri):

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

### Paso 6.1: Preparar los Secrets

1. Ve a tu repositorio en GitHub -> **Settings** -> **Secrets and variables** -> **Actions**.
2. Convierte el archivo `.p12` a texto Base64 (GitHub Secrets no acepta archivos binarios):
    ```bash
    base64 -i certificado_final.p12 > cert_base64.txt
    ```
3. Haz lo mismo con el archivo `.p8` del Paso 3:
    ```bash
    base64 -i AuthKey_XXXXXXXXXX.p8 > api_key_base64.txt
    ```
4. Añade estos **5 Repository Secrets** (son los únicos necesarios):

    | Secret Name | Valor |
    |---|---|
    | `APPLE_CERTIFICATE_BASE64` | Contenido de `cert_base64.txt` |
    | `APPLE_CERTIFICATE_PASSWORD` | Contraseña que pusiste al crear el `.p12` |
    | `APPLE_API_ISSUER` | Issuer ID del Paso 3 |
    | `APPLE_API_KEY` | Key ID del Paso 3 |
    | `APPLE_API_KEY_BASE64` | Contenido de `api_key_base64.txt` |

### Paso 6.2: Configurar el Workflow de GitHub Actions

> [!IMPORTANT]
> El Action de Tauri (`tauri-apps/tauri-action`) se encarga automáticamente de importar el certificado `.p12` al keychain de macOS. **NO** necesitas escribir comandos `security import` manuales.
> Sin embargo, para la **notarización**, Tauri necesita que la llave API `.p8` exista como un **archivo real** en disco, no como texto Base64. Por eso necesitamos un paso previo que decodifique el Secret y lo escriba en un archivo temporal.

Aquí está el ejemplo completo y funcional del workflow:

```yaml
steps:
    - uses: actions/checkout@v4

    # [Tus otros pasos de setup de Rust y Node]...

    # PASO CLAVE: Decodificar la API Key .p8 a un archivo real
    - name: Setup Apple API Key for Notarization (macOS only)
      if: matrix.platform == 'macos-latest'
      env:
          API_KEY_BASE64: ${{ secrets.APPLE_API_KEY_BASE64 }}
      run: |
          echo "$API_KEY_BASE64" | base64 --decode > ${{ runner.temp }}/api_key.p8

    - name: Build and upload
      uses: tauri-apps/tauri-action@v0
      env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          # --- Firma del código (certificado .p12) ---
          APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE_BASE64 }}
          APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
          # --- Notarización (API Key de App Store Connect) ---
          APPLE_API_ISSUER: ${{ secrets.APPLE_API_ISSUER }}
          APPLE_API_KEY: ${{ secrets.APPLE_API_KEY }}
          APPLE_API_KEY_PATH: ${{ runner.temp }}/api_key.p8
```

> [!CAUTION]
> **Errores comunes que nos encontramos y sus soluciones:**
> 1. **`MAC verification failed during PKCS12 import (wrong password?)`**: Esto ocurre porque OpenSSL v3 (incluido en Git Bash de Windows) usa algoritmos de encriptación modernos que macOS no reconoce. **Solución:** Al crear el `.p12`, usar la flag `-legacy` (ver Paso 2.4).
> 2. **`Apple no pudo verificar que la app no contenga software malicioso`**: Significa que la app está **firmada** pero **no notarizada**. **Solución:** Asegurarse de que la API Key `.p8` se decodifica como archivo real y se pasa como `APPLE_API_KEY_PATH` (no como `APPLE_API_KEY_BASE64`).

## Resumen del Flujo de Trabajo

1. Pagas tu membresía ($99).
2. Generas un CSR con Git Bash (Windows).
3. Solicitas un _Developer ID Certificate_ en el portal de Apple, descargas `.cer` y creas un `.p12` **usando la flag `-legacy`**.
4. Subes el `.p12` y la API Key `.p8` codificados en Base64 a GitHub como Secrets, junto a la contraseña, Issuer ID y Key ID.
5. Actualizas `tauri.conf.json` con tu Bundle ID y el archivo de Entitlements (`com.apple.security.app-sandbox` en `false`).
6. En el workflow de GitHub Actions, decodificas la API Key `.p8` a un archivo real y le pasas la ruta como `APPLE_API_KEY_PATH`.
7. Tauri construye, firma el `.dmg` con tu certificado e invoca la notarización en los servidores de Apple.
8. Al completarse, el `.dmg` descargable estará firmado y notarizado. macOS lo abrirá sin advertencias.

_(Para subir directo a la Mac App Store es un procedimiento similar donde cambias el tipo de certificado a 'Mac App Distribution' al principio, y subes el `.pkg` en lugar del `.dmg` usando Transporter Desktop CLI)._
