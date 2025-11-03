#!/bin/bash
# Demo script showing jb-reset capabilities

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JB_RESET="${SCRIPT_DIR}/../target/release/jb-reset"

# Check if jb-reset exists
if [ ! -f "${JB_RESET}" ]; then
    echo "Error: jb-reset binary not found at ${JB_RESET}"
    echo "Please run: cargo build --release"
    exit 1
fi

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║     JetBrains Trial Reset - Feature Demonstration             ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

echo "🔍 1. Scanning installed products..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
"${JB_RESET}" list
echo ""

echo "📊 2. Detailed status..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
"${JB_RESET}" status
echo ""

echo "🔧 3. Dry run (preview without changes)..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
"${JB_RESET}" reset --all --dry-run
echo ""

echo "📋 4. JSON output (for scripting)..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
"${JB_RESET}" list --json | head -20
echo "..."
echo ""

echo "🚀 5. Rofi format..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
"${JB_RESET}" list --rofi
echo ""

echo "✅ Demo complete!"
echo ""
echo "To actually reset trials, run:"
echo "  ${JB_RESET} reset --all"
echo ""
echo "Or reset specific products:"
echo "  ${JB_RESET} reset intellij pycharm"
echo ""
