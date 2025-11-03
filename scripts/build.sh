#!/bin/bash
# Build script for JetBrains Trial Reset

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "🔨 Building JetBrains Trial Reset..."

# Clean previous build
cargo clean

# Build release
cargo build --release

# Strip binary for smaller size
strip target/release/jb-reset 2>/dev/null || true

# Show size
SIZE=$(du -h target/release/jb-reset | cut -f1)
echo ""
echo "✅ Build complete!"
echo "📦 Binary size: $SIZE"
echo "📍 Location: target/release/jb-reset"
echo ""
echo "To install: sudo ./scripts/install.sh"
echo "Or run directly: ./target/release/jb-reset --help"
