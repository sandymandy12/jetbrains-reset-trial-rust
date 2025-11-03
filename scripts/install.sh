#!/bin/bash
# Installation script for JetBrains Trial Reset

set -e

INSTALL_DIR="/usr/local/bin"
DESKTOP_DIR="/usr/share/applications"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "🚀 Installing JetBrains Trial Reset..."

# Check if running with sudo for system-wide install
if [ "$EUID" -ne 0 ]; then
    echo "ℹ️  Installing to user directory (~/.local/bin)"
    INSTALL_DIR="$HOME/.local/bin"
    DESKTOP_DIR="$HOME/.local/share/applications"
    mkdir -p "$INSTALL_DIR" "$DESKTOP_DIR"
fi

# Build release binary
echo "📦 Building release binary..."
cd "$PROJECT_ROOT"
cargo build --release

# Install binary
echo "📥 Installing binary to $INSTALL_DIR..."
cp target/release/jb-reset "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/jb-reset"

# Install rofi script
echo "🔧 Installing rofi launcher..."
ROFI_SCRIPT_DIR="$HOME/.config/rofi/scripts"
mkdir -p "$ROFI_SCRIPT_DIR"
cp scripts/rofi-launcher.sh "$ROFI_SCRIPT_DIR/jb-reset.sh"
chmod +x "$ROFI_SCRIPT_DIR/jb-reset.sh"

# Install desktop entry
echo "🖥️  Installing desktop entry..."
cat > "$DESKTOP_DIR/jb-reset.desktop" << EOF
[Desktop Entry]
Name=JetBrains Trial Reset
Comment=Reset trial period for JetBrains IDEs
Exec=$INSTALL_DIR/jb-reset status
Icon=jetbrains-toolbox
Terminal=true
Type=Application
Categories=Development;Utility;
Keywords=jetbrains;trial;reset;ide;
EOF

echo ""
echo "✅ Installation complete!"
echo ""
echo "Usage:"
echo "  jb-reset list          - List all products"
echo "  jb-reset reset --all   - Reset all products"
echo "  jb-reset status        - Show status"
echo ""
echo "Rofi launcher: ~/.config/rofi/scripts/jb-reset.sh"
echo ""
