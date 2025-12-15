#!/bin/bash
# Setup Rust/Cargo for JetBrains Trial Reset build

set -e

echo "🦀 Checking Rust installation..."

# Check if cargo is already installed
if command -v cargo &> /dev/null; then
    echo "✅ Cargo is already installed"
    cargo --version
    rustc --version
    exit 0
fi

echo ""
echo "❌ Rust/Cargo is not installed"
echo ""
echo "Installing Rust via rustup..."
echo ""

# Download and run rustup installer
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Source cargo environment
echo ""
echo "📦 Setting up Cargo environment..."
source "$HOME/.cargo/env"

# Verify installation
echo ""
echo "✅ Rust/Cargo installed successfully!"
echo ""
cargo --version
rustc --version

echo ""
echo "💡 Note: You may need to run 'source \$HOME/.cargo/env' in new terminal windows"
echo ""
