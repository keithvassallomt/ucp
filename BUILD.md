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
install the webkit2gtk dependencies:

```bash
sudo apt-get update
sudo apt-get install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

### 2. Build Release (.deb & .AppImage)

```bash
npm run tauri build
```

**Output Location:**
- `.deb` package: `src-tauri/target/release/bundle/deb/`
- AppImage: `src-tauri/target/release/bundle/appimage/`

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
