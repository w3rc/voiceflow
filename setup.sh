#!/bin/bash
# VoiceFlow — System dependency installation script
# Run with: sudo bash setup.sh

set -e

echo "Installing system dependencies for VoiceFlow..."

# Detect distro
if [ -f /etc/os-release ]; then
    . /etc/os-release
    DISTRO=$ID
else
    echo "Could not detect Linux distribution."
    exit 1
fi

case "$DISTRO" in
    ubuntu|debian|linuxmint|pop)
        echo "Detected Debian-based distro ($DISTRO)"
        apt-get update
        apt-get install -y \
            build-essential \
            libwebkit2gtk-4.1-dev \
            libjavascriptcoregtk-4.1-dev \
            libssl-dev \
            libayatana-appindicator3-dev \
            librsvg2-dev \
            libasound2-dev \
            pkg-config \
            libxdo-dev
        ;;
    fedora)
        echo "Detected Fedora"
        dnf install -y \
            gcc \
            webkit2gtk4.1-devel \
            openssl-devel \
            libayatana-appindicator-gtk3-devel \
            librsvg2-devel \
            alsa-lib-devel \
            pkg-config \
            libxdo-devel \
            curl \
            wget
        ;;
    rhel|centos|rocky|almalinux)
        echo "Detected RHEL-based distro ($DISTRO)"
        dnf install -y epel-release
        dnf install -y \
            gcc \
            webkit2gtk4.1-devel \
            openssl-devel \
            librsvg2-devel \
            alsa-lib-devel \
            pkg-config \
            curl \
            wget
        ;;
    arch|manjaro|endeavouros)
        echo "Detected Arch-based distro ($DISTRO)"
        pacman -Syu --noconfirm \
            base-devel \
            webkit2gtk-4.1 \
            openssl \
            libayatana-appindicator \
            librsvg \
            alsa-lib \
            pkgconf \
            xdotool
        ;;
    *)
        echo "Unsupported distro: $DISTRO"
        echo "Please install Tauri v2 dependencies manually:"
        echo "  https://v2.tauri.app/start/prerequisites/"
        exit 1
        ;;
esac

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
