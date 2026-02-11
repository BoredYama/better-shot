#!/bin/bash
set -e

echo "Setting up BetterShot for Linux..."

# Check for apt-get
if command -v apt-get &> /dev/null; then
    echo "Installing system dependencies via apt..."
    sudo apt-get update
    sudo apt-get install -y build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libwebkit2gtk-4.1-dev libpipewire-0.3-dev clang libclang-dev libgbm-dev
    
    echo "Installing runtime dependencies (OCR, Screenshot tools)..."
    sudo apt-get install -y tesseract-ocr tesseract-ocr-eng gnome-screenshot scrot xdotool libasound2-dev
else
    echo "Warning: apt-get not found. Please install the following dependencies manually:"
    echo "- build-essential, libssl-dev, libgtk-3-dev, libayatana-appindicator3-dev, librsvg2-dev, libwebkit2gtk-4.0-dev"
    echo "- tesseract-ocr, gnome-screenshot/scrot, xdotool"
fi

# Install Rust if not present
if ! command -v cargo &> /dev/null; then
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
    fi
fi

if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust is already installed."
fi

# Install pnpm if not present
if ! command -v pnpm &> /dev/null; then
    echo "Installing pnpm..."
    npm install -g pnpm
else
    echo "pnpm is already installed."
fi

echo "Installing project dependencies..."
pnpm install

echo "Setup complete! You can now run 'pnpm tauri dev' to start the app."
