#!/bin/bash
# VoiceFlow — System dependency installation script
# Run with: sudo bash setup.sh

set -e

echo "Installing system dependencies for VoiceFlow..."

# Tauri v2 build dependencies
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    libwebkit2gtk-4.1-dev \
    libjavascriptcoregtk-4.1-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libasound2-dev \
    pkg-config \
    libxdo-dev

# Install Rust if not present
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Install npm dependencies
npm install

echo ""
echo "Setup complete! To run the app:"
echo "  npm run tauri dev"
echo ""
echo "To build for distribution:"
echo "  npm run tauri build"
