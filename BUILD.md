# Build Instructions

We use `just` to standardize build commands across platforms.

## Prerequisites
1.  **Node.js** (v18+)
2.  **Rust** (Stable)
3.  **Just**: `cargo install just` (or via your package manager)

## Standard Workflows

### 1. Native Build (Windows/macOS/Linux)
Builds the standard installer for your current OS (`.exe`, `.dmg`, `.deb/.rpm`):

```bash
just build
```

**Output:** `src-tauri/target/release/bundle/`

---

### 2. Flatpak (Linux Only)

#### Option A: Local Bundle (Faster)
Builds the binary on your host and bundles it. Good for local testing or GitHub Releases.

```bash
just flatpak-local
```

#### Option B: Flathub Source Build (Strict)
Compiles everything from source (Cargo + NPM) in an offline sandbox. Required for Flathub submission.

```bash
just flatpak-flathub
```
*Note: This requires `org.freedesktop.Sdk.Extension.rust-stable//25.08` and `node22//25.08`.*

#### Run Flatpak
```bash
just run-flatpak
```

---

## Troubleshooting & logs
To capture logs during a native build:

**Linux/macOS:**
```bash
npm run tauri build 2>&1 | tee build.log
```

**Windows (PowerShell):**
```powershell
npm run tauri build *>&1 | Tee-Object build.log
```
