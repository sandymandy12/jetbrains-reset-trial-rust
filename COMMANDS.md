# Quick Reference Commands

## Build & Install

```bash
# Build release binary
cargo build --release

# Install system-wide (requires sudo)
sudo ./scripts/install.sh

# Install to user directory (~/.local/bin)
./scripts/install.sh

# Build with optimization
./scripts/build.sh

# Run demo
./scripts/demo.sh
```

## Basic Usage

```bash
# List all installed products
jb-reset list

# Show detailed status
jb-reset status

# Reset all products
jb-reset reset --all

# Reset specific product
jb-reset reset intellij
jb-reset reset pycharm
jb-reset reset webstorm

# Dry run (preview only)
jb-reset reset --all --dry-run

# Reset without backup (not recommended)
jb-reset reset --all --no-backup
```

## Output Formats

```bash
# Human-readable table (default)
jb-reset list

# JSON output
jb-reset list --json

# Rofi/dmenu format
jb-reset list --rofi

# Pretty JSON (for debugging)
jb-reset list --json | jq .
```

## Backup Management

```bash
# List all backups
jb-reset backup list

# Delete specific backup
jb-reset backup delete "IntelliJ IDEA_2025.2_20251103_154530"

# Manual backup location
ls ~/.jetbrains-trial-backups/
```

## Integration Commands

```bash
# Rofi launcher
~/.config/rofi/scripts/jb-reset.sh

# Add to rofi modi (edit ~/.config/rofi/config.rasi)
modi: "drun,jb-reset:~/.config/rofi/scripts/jb-reset.sh";

# Test notification
notify-send "Test" "JetBrains Trial Reset" -i jetbrains-toolbox
```

## Advanced Usage

```bash
# Run with debug logging
RUST_LOG=debug jb-reset list

# Check binary size
du -h target/release/jb-reset

# Strip symbols (reduce size)
strip target/release/jb-reset

# Verify it's working
./target/release/jb-reset --version

# Check help
jb-reset --help
jb-reset reset --help
```

## Testing Commands

```bash
# Dry run to see what would change
jb-reset reset --all --dry-run

# Check products before reset
jb-reset status

# Reset and verify
jb-reset reset intellij
jb-reset status

# List backups created
jb-reset backup list
```

## Scripting Examples

```bash
# Check if any product expired
jb-reset list --json | jq -r '.[] | select(.trial_status == "Expired") | .product'

# Get products with <7 days
jb-reset list --json | jq -r '.[] | select(.trial_status.Active.days_remaining < 7) | .product'

# Count installed products
jb-reset list --json | jq 'length'

# Reset only expired
for product in $(jb-reset list --json | jq -r '.[] | select(.trial_status == "Expired") | .product'); do
    jb-reset reset "$product"
done
```

## Aliases (add to ~/.bashrc or ~/.zshrc)

```bash
# Quick aliases
alias jls='jb-reset list'
alias jst='jb-reset status'
alias jrs='jb-reset reset --all'
alias jdr='jb-reset reset --all --dry-run'
alias jbk='jb-reset backup list'

# Function wrapper
jbr() {
    case "$1" in
        "") jb-reset list ;;
        "reset") jb-reset reset --all ;;
        "status") jb-reset status ;;
        *) jb-reset reset "$1" ;;
    esac
}
```

## Keybindings

### i3wm
```bash
# Add to ~/.config/i3/config
bindsym $mod+j exec ~/.config/rofi/scripts/jb-reset.sh
```

### dwm
```bash
# Add to your dwm startup script or use sxhkd
# ~/.config/sxhkd/sxhkdrc
super + j
    ~/.config/rofi/scripts/jb-reset.sh
```

### sway
```bash
# Add to ~/.config/sway/config
bindsym $mod+j exec ~/.config/rofi/scripts/jb-reset.sh
```

## Systemd Integration

```bash
# Create service
cat > ~/.config/systemd/user/jb-reset.service << EOF
[Unit]
Description=JetBrains Trial Reset

[Service]
Type=oneshot
ExecStart=/usr/local/bin/jb-reset reset --all
EOF

# Create timer (weekly)
cat > ~/.config/systemd/user/jb-reset.timer << EOF
[Unit]
Description=Auto-reset JetBrains trials weekly

[Timer]
OnCalendar=weekly
Persistent=true

[Install]
WantedBy=timers.target
EOF

# Enable and start
systemctl --user daemon-reload
systemctl --user enable --now jb-reset.timer

# Check status
systemctl --user status jb-reset.timer
systemctl --user list-timers
```

## Troubleshooting Commands

```bash
# Check if products are detected
ls ~/.config/JetBrains/

# Verify config files exist
ls ~/.config/JetBrains/IntelliJIdea2025.2/options/other.xml

# Check trial keys in config
grep -i "evlsprt\|trial" ~/.config/JetBrains/IntelliJIdea2025.2/options/other.xml

# Check data directory
ls ~/.local/share/JetBrains/

# Manual backup
cp -r ~/.config/JetBrains/ ~/jetbrains-backup-$(date +%Y%m%d)

# Manual reset (emergency)
rm -rf ~/.config/JetBrains/*/eval/
sed -i '/evlsprt\|trial\.state/d' ~/.config/JetBrains/*/options/other.xml

# Check permissions
ls -la ~/.config/JetBrains/
ls -la ~/.local/share/JetBrains/

# Fix permissions
chmod -R u+rw ~/.config/JetBrains/
```

## Development Commands

```bash
# Run in debug mode
cargo run -- list

# Run tests
cargo test

# Check for issues
cargo clippy

# Format code
cargo fmt

# Update dependencies
cargo update

# Clean build artifacts
cargo clean

# Build documentation
cargo doc --open

# Run specific test
cargo test scanner

# Benchmark (if implemented)
cargo bench
```

## Distribution Commands

```bash
# Create release binary
cargo build --release

# Strip and compress
strip target/release/jb-reset
upx target/release/jb-reset  # Optional compression

# Create tarball
tar czf jb-reset-linux-x64.tar.gz -C target/release jb-reset

# Generate checksums
sha256sum target/release/jb-reset > jb-reset.sha256

# Package for distribution
mkdir -p dist
cp target/release/jb-reset dist/
cp README.md LICENSE dist/
tar czf jb-reset-v1.0.0-linux-x64.tar.gz dist/
```

## Git Commands (if creating repository)

```bash
# Initialize repo
git init
git add .
git commit -m "Initial commit: JetBrains Trial Reset tool"

# Create repository on GitHub, then:
git remote add origin git@github.com:YOUR_USERNAME/jetbrains-trial-reset.git
git branch -M main
git push -u origin main

# Create release
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

## Performance Testing

```bash
# Time execution
time jb-reset list
time jb-reset reset --all --dry-run

# Memory profiling
/usr/bin/time -v jb-reset list

# Binary size analysis
size target/release/jb-reset
ls -lh target/release/jb-reset

# Dependencies
cargo tree
```

## Safety Checks

```bash
# Always check before real reset
jb-reset list
jb-reset status
jb-reset reset --all --dry-run

# Verify backup exists
jb-reset backup list

# Test on one product first
jb-reset reset intellij --dry-run
jb-reset reset intellij

# Verify it worked
jb-reset status
```

---

**Tip:** Always close JetBrains IDEs before running reset!
