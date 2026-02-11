# Better Shot

<img width="3600" height="2025" alt="stage-1768238789948" src="https://github.com/user-attachments/assets/3051266a-5179-440f-a747-7980abd7bac3" />

[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?style=for-the-badge&logo=discord&logoColor=white)](https://discord.gg/zThjstVs)
[![X (Twitter)](https://img.shields.io/badge/X-%231DA1F2.svg?style=for-the-badge&logo=X&logoColor=white)](https://x.com/code_kartik)
[![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-%23FFDD00.svg?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black)](https://www.buymeacoffee.com/code_kartik)

> An open-source screenshot tool for Linux. Capture, edit, and enhance your screenshots with professional quality.

Better Shot is a fast, lightweight screenshot tool built with Tauri + React. It provides a powerful yet simple workflow for capturing screenshots, editing them with backgrounds/effects/annotations, and exporting quickly.

## Table of Contents

- [Features](#features)
- [Install](#install)
  - [Build from Source](#build-from-source)
  - [System Dependencies](#system-dependencies)
- [Usage](#usage)
  - [Quick Start](#quick-start)
  - [Capture Modes](#capture-modes)
  - [Auto-apply Workflow](#auto-apply-workflow)
  - [Keyboard Shortcuts](#keyboard-shortcuts)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)
- [Star History](#star-history)

## Features

### Capture Modes

- **Region Capture** — select any area of your screen
- **Fullscreen Capture** — capture a specific monitor (multi-monitor support with a selector)
- **Window Capture** — click on any open window to capture it
- **OCR Region** — extract text from a selected region (requires `tesseract-ocr`)

### Image Editing

- **Background Library** — curated wallpapers, mesh patterns, and assets
- **Custom Backgrounds** — solid colors and transparent checkerboard
- **Effects** — blur + noise controls
- **Shadow + Roundness** — tune depth and corner radius
- **Export** — save at high quality via a native file picker

### Annotation Tools

- **Shapes** — circle, rectangle, line, arrow
- **Text** — add text with adjustable size
- **Numbered Labels** — auto-incrementing badges for step-by-step callouts
- **Editability** — select, move, and delete annotations
- **Styling** — colors, opacity, borders, alignment

### Workflow

- **Global Shortcuts** — capture from anywhere, even when the window is hidden
- **Auto-apply** — apply default background and save without opening the editor
- **Quick Overlay** — preview captures in a floating overlay that automatically fades out
- **Clipboard** — copy to clipboard after capture/export
- **Native File Picker** — choose where to save and what filename to use
- **System Tray** — accessible from the system tray
- **Preferences** — save directory, defaults, and shortcut settings persist
- **Native Performance** — built with Rust + Tauri

## Install

### System Dependencies

Better Shot requires the following system packages on Linux. Install them before building:

**Ubuntu / Debian:**

```bash
# Build dependencies
sudo apt-get update
sudo apt-get install -y \
  build-essential curl wget file clang libclang-dev \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev libwebkit2gtk-4.1-dev libpipewire-0.3-dev \
  libgbm-dev libasound2-dev

# Runtime dependencies
sudo apt-get install -y \
  xclip xdotool imagemagick \
  tesseract-ocr tesseract-ocr-eng
```

| Package                        | Purpose                                      |
| ------------------------------ | -------------------------------------------- |
| `xclip`                        | Copy images to clipboard                     |
| `xdotool`                      | Window selection for window capture mode     |
| `imagemagick`                  | Window screenshot capture (`import` command) |
| `tesseract-ocr`                | OCR text extraction                          |
| `libwebkit2gtk-4.1-dev`        | Tauri webview                                |
| `libgtk-3-dev`                 | GTK3 for native UI                           |
| `libayatana-appindicator3-dev` | System tray support                          |
| `libpipewire-0.3-dev`          | Screen capture support                       |

**Fedora:**

```bash
sudo dnf install -y \
  gcc-c++ curl wget file clang clang-devel \
  openssl-devel gtk3-devel libayatana-appindicator-gtk3-devel \
  librsvg2-devel webkit2gtk4.1-devel pipewire-devel \
  mesa-libgbm-devel alsa-lib-devel \
  xclip xdotool ImageMagick \
  tesseract tesseract-langpack-eng
```

**Arch Linux:**

```bash
sudo pacman -S --needed \
  base-devel curl wget file clang \
  openssl gtk3 libayatana-appindicator \
  librsvg webkit2gtk-4.1 pipewire \
  mesa alsa-lib \
  xclip xdotool imagemagick \
  tesseract tesseract-data-eng
```

### Build from Source

**Prerequisites:**

- **Node.js** 18+
- **pnpm** (install via `npm install -g pnpm`)
- **Rust** (latest stable — install via [rustup](https://rustup.rs/))
- System dependencies listed above

**Steps:**

```bash
# Clone the repository
git clone https://github.com/BoredYama/better-shot.git
cd better-shot

# (Optional) Run the automated setup script
# This installs system deps, Rust, pnpm, and project deps
bash scripts/setup_linux.sh

# Or manually:
pnpm install
pnpm tauri build
```

The built application will be located in `src-tauri/target/release/bundle/`. You'll find:

- **`.deb`** — Debian/Ubuntu package (install with `sudo dpkg -i bettershot_*.deb`)
- **`.AppImage`** — Portable, runs directly (make executable with `chmod +x` and run)
- **`.rpm`** — Fedora/RHEL package (install with `sudo rpm -i bettershot_*.rpm`)

### Post-Install

**Verify runtime dependencies are available:**

```bash
# These should all return a path or version
which xclip xdotool import tesseract
```

If any are missing, install them with your package manager (see [System Dependencies](#system-dependencies)).

## Usage

### Quick Start

1. Launch Better Shot from your application menu or system tray
2. Choose a capture mode: **Region**, **Screen**, **Window**, or **OCR Region**
3. Edit your screenshot — add backgrounds, effects, shadows, annotations
4. Export with **Ctrl+S** (opens a native file picker) or copy to clipboard with **Ctrl+Shift+C**

### Capture Modes

| Mode           | How It Works                                                                                 |
| -------------- | -------------------------------------------------------------------------------------------- |
| **Region**     | Click and drag to select any area of your screen                                             |
| **Screen**     | Captures a full monitor. On multi-monitor setups, a selector appears to choose which display |
| **Window**     | Your cursor changes to a crosshair — click on any window to capture it                       |
| **OCR Region** | Select a region and the text is extracted and copied to your clipboard                       |

### Auto-apply Workflow

For faster workflows, enable **Auto-apply background** on the main screen:

1. Toggle on "Auto-apply background" on the main page
2. Set your preferred default background in Preferences
3. Capture a screenshot — the background is applied and saved instantly
4. A Quick Overlay appears showing the preview, then fades out automatically
5. No editor needed — perfect for quick captures with consistent styling

### Keyboard Shortcuts

All capture shortcuts are customizable in **Preferences**.

#### Capture Shortcuts

| Action           | Default Shortcut                     |
| ---------------- | ------------------------------------ |
| Capture Region   | `Ctrl+Shift+2`                       |
| Capture Screen   | `Ctrl+Shift+F` (disabled by default) |
| Capture Window   | `Ctrl+Shift+D` (disabled by default) |
| OCR Region       | `Ctrl+Shift+O` (disabled by default) |
| Cancel Selection | `Esc`                                |

#### Editor Shortcuts

| Action            | Shortcut                |
| ----------------- | ----------------------- |
| Save/Export Image | `Ctrl+S`                |
| Copy to Clipboard | `Ctrl+Shift+C`          |
| Undo              | `Ctrl+Z`                |
| Redo              | `Ctrl+Shift+Z`          |
| Delete Annotation | `Delete` or `Backspace` |
| Close Editor      | `Esc`                   |

## Development

This repo contains:

- The **desktop app** (Tauri + Vite) at the repo root
- The **landing site** (Next.js) in `bettershot-landing/`

### Desktop App (Tauri)

```bash
# Start the dev server with hot reload
pnpm tauri dev

# Build for production
pnpm tauri build

# Run lint checks
pnpm lint:ci

# Run Rust tests
pnpm test:rust
```

### Landing Site (Next.js)

```bash
cd bettershot-landing
pnpm install
pnpm dev
```

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting a pull request.

## License

This project is licensed under the BSD 3-Clause License — see the [LICENSE](LICENSE) file for details.

## Star History

<a href="https://www.star-history.com/#KartikLabhshetwar/better-shot&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=KartikLabhshetwar/better-shot&type=date&theme=dark&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=KartikLabhshetwar/better-shot&type=date&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=KartikLabhshetwar/better-shot&type=date&legend=top-left" />
 </picture>
</a>
