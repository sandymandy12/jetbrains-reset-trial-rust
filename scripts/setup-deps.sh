#!/bin/bash
# Install build dependencies for JetBrains Trial Reset

set -e

echo "📦 Installing build dependencies..."
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "❌ This script requires sudo privileges"
    echo ""
    echo "Run with:"
    echo "  sudo bash scripts/setup-deps.sh"
    echo ""
    exit 1
fi

# Detect package manager and install dependencies
if command -v apt-get &> /dev/null; then
    echo "Using apt (Debian/Ubuntu/WSL)"
    apt-get update
    apt-get install -y build-essential pkg-config libssl-dev
    echo "✅ Build dependencies installed"

elif command -v dnf &> /dev/null; then
    echo "Using dnf (Fedora/RHEL)"
    dnf install -y gcc pkg-config openssl-devel
    echo "✅ Build dependencies installed"

elif command -v pacman &> /dev/null; then
    echo "Using pacman (Arch)"
    pacman -S --noconfirm base-devel
    echo "✅ Build dependencies installed"

elif command -v zypper &> /dev/null; then
    echo "Using zypper (openSUSE)"
    zypper install -y gcc pkg-config libopenssl-devel
    echo "✅ Build dependencies installed"

else
    echo "❌ Could not detect package manager"
    echo ""
    echo "Please install build tools manually:"
    echo "  - gcc / clang"
    echo "  - pkg-config"
    echo "  - libssl-dev (or equivalent)"
    echo ""
    exit 1
fi
