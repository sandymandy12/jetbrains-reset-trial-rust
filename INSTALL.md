# Installation Guide

## Prerequisites

- Linux system (tested on Arch/CachyOS)
- Rust 1.70+ (for building from source)
- JetBrains products installed

## Quick Install

### Option 1: Automated Install (Recommended)

```bash
cd "~/jetbrains-trial-reset"
sudo ./scripts/install.sh
```

This will:
- Build the release binary
- Install to `/usr/local/bin/jb-reset`
- Install rofi launcher to `~/.config/rofi/scripts/`
- Create desktop entry

### Option 2: User Install (No sudo)

```bash
cd "~/jetbrains-trial-reset"
./scripts/install.sh
```

This will:
- Build the release binary
- Install to `~/.local/bin/jb-reset`
- Install rofi launcher to `~/.config/rofi/scripts/`
- Create desktop entry in `~/.local/share/applications/`

### Option 3: Manual Install

```bash
# Build
cargo build --release

# Copy binary
sudo cp target/release/jb-reset /usr/local/bin/
# Or for user install:
cp target/release/jb-reset ~/.local/bin/

# Make executable
chmod +x /usr/local/bin/jb-reset
# Or:
chmod +x ~/.local/bin/jb-reset

# Verify installation
jb-reset --version
```

## Post-Installation

### 1. Verify Installation

```bash
# Check if binary is in PATH
which jb-reset

# Test it works
jb-reset list
```

### 2. Add to PATH (if needed)

If you installed to `~/.local/bin/` and it's not in PATH:

```bash
# Add to ~/.bashrc or ~/.zshrc
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### 3. Configure Rofi (Optional)

The rofi launcher is installed automatically, but you can add it to your config:

```bash
# Edit ~/.config/rofi/config.rasi
modi: "drun,run,window,jb-reset:~/.config/rofi/scripts/jb-reset.sh";
```

### 4. Add Keybinding (Optional)

#### For i3wm
```bash
# Add to ~/.config/i3/config
bindsym $mod+j exec ~/.config/rofi/scripts/jb-reset.sh
```

#### For dwm (using sxhkd)
```bash
# Add to ~/.config/sxhkd/sxhkdrc
super + j
    ~/.config/rofi/scripts/jb-reset.sh
```

#### For sway
```bash
# Add to ~/.config/sway/config
bindsym $mod+j exec ~/.config/rofi/scripts/jb-reset.sh
```

## Verification

Test all features:

```bash
# List products
jb-reset list

# Show status
jb-reset status

# Dry run
jb-reset reset --all --dry-run

# Rofi launcher
~/.config/rofi/scripts/jb-reset.sh
```

## Troubleshooting

### Binary not found

```bash
# Check installation location
ls -la /usr/local/bin/jb-reset
ls -la ~/.local/bin/jb-reset

# Verify PATH
echo $PATH

# Add to PATH if needed
export PATH="$HOME/.local/bin:$PATH"
```

### Permission denied

```bash
# Make executable
chmod +x /usr/local/bin/jb-reset
# Or
chmod +x ~/.local/bin/jb-reset
```

### Rofi script not working

```bash
# Check if script exists
ls -la ~/.config/rofi/scripts/jb-reset.sh

# Make executable
chmod +x ~/.config/rofi/scripts/jb-reset.sh

# Test manually
~/.config/rofi/scripts/jb-reset.sh
```

### Build fails

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release

# Check Rust version
rustc --version  # Should be 1.70+
```

## Updating

To update to a newer version:

```bash
cd "~/jetbrains-trial-reset"

# Pull updates (if using git)
git pull

# Rebuild and reinstall
cargo build --release
sudo ./scripts/install.sh
```

## Uninstallation

```bash
# Remove binary
sudo rm /usr/local/bin/jb-reset
# Or user install:
rm ~/.local/bin/jb-reset

# Remove rofi script
rm ~/.config/rofi/scripts/jb-reset.sh

# Remove desktop entry
sudo rm /usr/share/applications/jb-reset.desktop
# Or user install:
rm ~/.local/share/applications/jb-reset.desktop

# Remove backups (optional)
rm -rf ~/.jetbrains-trial-backups/
```

## System Requirements

### Minimum
- Linux kernel 4.0+
- 10 MB free disk space
- 10 MB RAM

### Recommended
- Linux kernel 5.0+
- 50 MB free disk space (for backups)
- Modern terminal with UTF-8 and color support
- rofi/dmenu (for launcher)
- notify-send (for notifications)

## Compatibility

### Tested On
- ✅ Arch Linux / CachyOS
- ✅ dwm window manager
- ✅ rofi 1.7+

### Should Work On
- Ubuntu/Debian (20.04+)
- Fedora (35+)
- openSUSE
- Any modern Linux distribution

### Known Issues
- None currently

## Next Steps

After installation:

1. **Read the docs**
   - `README.md` - Overview and features
   - `USAGE.md` - Detailed usage guide
   - `COMMANDS.md` - Quick reference

2. **Try it out**
   ```bash
   jb-reset list
   jb-reset status
   ```

3. **Configure integrations**
   - Add rofi keybinding
   - Set up aliases
   - Configure systemd timer (optional)

4. **Backup important settings**
   - The tool backs up automatically
   - But you can create manual backups too

## Support

If you encounter issues:

1. Check `USAGE.md` for troubleshooting
2. Run with debug: `RUST_LOG=debug jb-reset list`
3. Check permissions: `ls -la ~/.config/JetBrains/`
4. Verify products are installed: `ls ~/.config/JetBrains/`

## License

MIT License - See LICENSE file

---

**Enjoy your JetBrains IDEs with extended trials!** 🚀
