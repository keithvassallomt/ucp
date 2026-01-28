# Build Instructions

## Prerequisites

Ensure you have the following installed:
- **Node.js** (v16 or later)
- **Rust** (latest stable)
- **Git**

## Windows

### 1. System Requirements
- **Microsoft Visual Studio C++ Build Tools**. You can download the "Build Tools for Visual Studio" installer. During installation, select the "Desktop development with C++" workload.

### 2. Build Release (Installer)
To generate the `.exe` installer (NSIS):

```powershell
npm run tauri build
```

This command will compile the Rust backend, build the React frontend, and bundle them into an installer.

**Output Location:**
The installer will be located at:
`src-tauri/target/release/bundle/nsis/ClusterCut_x.x.x_x64-setup.exe`

## Linux

### 1. System Requirements
**Debian/Ubuntu:**
```bash
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

**Fedora:**
```bash
sudo dnf install webkit2gtk3-devel openssl-devel curl wget file libappindicator-gtk3-devel librsvg2-devel libayatana-appindicator-gtk3-devel fuse rpm-build
```

### 2. Build Release (.deb & .AppImage)

```bash
npm run tauri build
```

**Output Location:**
- `.deb` package: `src-tauri/target/release/bundle/deb/`
- AppImage: `src-tauri/target/release/bundle/appimage/`

### 3. Build Flatpak (Optional)

**Prerequisites:**

1.  Clone `flathub/shared-modules` inside `src-tauri/flatpak`:
    ```bash
    git clone https://github.com/flathub/shared-modules.git src-tauri/flatpak/shared-modules
    ```
2.  Install the GNOME 47 Platform:
    ```bash
    flatpak install flathub org.gnome.Platform//47 org.gnome.Sdk//47
    ```

**Build Command:**

```bash
cd src-tauri/flatpak
flatpak-builder --user --install --force-clean build-dir com.keithvassallo.clustercut.yml
```

**Run:**

```bash
flatpak run com.keithvassallo.clustercut
```

## macOS

### 1. System Requirements
- Xcode Command Line Tools (`xcode-select --install`)

### 2. Build Release (.dmg)

```bash
npm run tauri build
```

**Output Location:**
- `.dmg` image: `src-tauri/target/release/bundle/dmg/`
- `.app` bundle: `src-tauri/target/release/bundle/macos/`

## Troubleshooting

### Capturing Build Logs
If the build fails and you need to share the errors, you can save the output to a file.

**Windows (PowerShell):**
```powershell
npm run tauri build *>&1 | Tee-Object build_log.txt
```
*This command runs the build, shows the output on screen, AND saves it to `build_log.txt`.*

**Linux / macOS:**
```bash
npm run tauri build 2>&1 | tee build_log.txt
```
