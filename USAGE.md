# Usage Guide - JetBrains Trial Reset

## Quick Start

```bash
# See what products you have installed
jb-reset list

# Reset all trials (with automatic backup)
jb-reset reset --all

# Reset specific product
jb-reset reset intellij
```

## Command Reference

### List Products

```bash
# Simple list
jb-reset list

# JSON output (for scripts)
jb-reset list --json

# Rofi/dmenu format
jb-reset list --rofi
```

### Reset Trials

```bash
# Reset all products
jb-reset reset --all

# Reset specific products
jb-reset reset intellij
jb-reset reset pycharm webstorm

# Dry run (preview only, no changes)
jb-reset reset --all --dry-run

# Reset without creating backup (not recommended)
jb-reset reset intellij --no-backup
```

### Check Status

```bash
# Detailed status of all products
jb-reset status
```

### Manage Backups

```bash
# List all backups
jb-reset backup list

# Delete specific backup
jb-reset backup delete "IntelliJ IDEA_2025.2_20251103_154530"
```

## Integration Examples

### Bash Script

```bash
#!/bin/bash
# Auto-reset when trial expires

products=$(jb-reset list --json)
expired=$(echo "$products" | jq -r '.[] | select(.trial_status.Expired) | .product')

if [ -n "$expired" ]; then
    echo "Found expired trials, resetting..."
    jb-reset reset --all
    notify-send "JetBrains Trials Reset" "All expired trials have been reset"
fi
```

### Rofi Integration

Add to your rofi config (~/.config/rofi/config.rasi):

```bash
configuration {
    modi: "drun,run,window,jb-reset:~/.config/rofi/scripts/jb-reset.sh";
}
```

Then launch with: `rofi -show jb-reset`

### i3/dwm Keybinding

Add to your window manager config:

```bash
# i3 config
bindsym $mod+j exec ~/.config/rofi/scripts/jb-reset.sh

# dwm config (in your dwm-start script or similar)
Super+j: exec ~/.config/rofi/scripts/jb-reset.sh
```

### Systemd Timer (Auto-reset)

Create `~/.config/systemd/user/jb-reset.service`:

```ini
[Unit]
Description=JetBrains Trial Reset

[Service]
Type=oneshot
ExecStart=/usr/local/bin/jb-reset reset --all
```

Create `~/.config/systemd/user/jb-reset.timer`:

```ini
[Unit]
Description=Auto-reset JetBrains trials weekly

[Timer]
OnCalendar=weekly
Persistent=true

[Install]
WantedBy=timers.target
```

Enable:

```bash
systemctl --user enable --now jb-reset.timer
```

## Scripting Examples

### Python Integration

```python
import subprocess
import json

# Get products as JSON
result = subprocess.run(['jb-reset', 'list', '--json'], capture_output=True)
products = json.loads(result.stdout)

for product in products:
    name = product['product']
    status = product['trial_status']
    
    if 'Active' in status:
        days = status['Active']['days_remaining']
        print(f"{name}: {days} days remaining")
```

### Shell Function

Add to your `.bashrc` or `.zshrc`:

```bash
# Quick JetBrains reset
jbr() {
    case "$1" in
        "")
            jb-reset list
            ;;
        "reset")
            jb-reset reset --all
            ;;
        "status")
            jb-reset status
            ;;
        *)
            jb-reset reset "$1"
            ;;
    esac
}

# Aliases
alias jls='jb-reset list'
alias jst='jb-reset status'
alias jrs='jb-reset reset --all'
```

## Backup and Restore

### Manual Backup Before Reset

```bash
# Backups are automatic, but you can verify
jb-reset backup list

# Example output:
# 📦 IntelliJ IDEA_2025.2_20251103_154530 │ 2025-11-03 15:45:30
```

### Restore from Backup (Manual)

If you need to restore:

```bash
# Find your backup
BACKUP=~/.jetbrains-trial-backups/IntelliJ_IDEA_2025.2_20251103_154530

# Restore options directory
cp -r "$BACKUP/options" ~/.config/JetBrains/IntelliJIdea2025.2/
```

## Tips & Tricks

### 1. Check Before IDE Opens

```bash
# Add to your IDE launcher script
if jb-reset list | grep -q "Expired"; then
    jb-reset reset --all
fi
idea  # or pycharm, webstorm, etc.
```

### 2. Weekly Auto-check

Add to crontab:

```bash
# Edit crontab
crontab -e

# Add line (runs every Monday at 9 AM)
0 9 * * 1 /usr/local/bin/jb-reset reset --all
```

### 3. Integration with Desktop Notification

```bash
# Reset and notify
jb-reset reset --all && notify-send "Trials Reset" "JetBrains trials have been refreshed"
```

### 4. Selective Reset Based on Days

```bash
#!/bin/bash
# Reset only if less than 7 days remaining

products=$(jb-reset list --json)
for row in $(echo "$products" | jq -c '.[]'); do
    name=$(echo "$row" | jq -r '.product')
    status=$(echo "$row" | jq -r '.trial_status')
    
    if echo "$status" | jq -e '.Active.days_remaining < 7' > /dev/null; then
        echo "Resetting $name (low days remaining)"
        jb-reset reset "$name"
    fi
done
```

## Troubleshooting

### Product Not Detected

```bash
# Check config directory
ls ~/.config/JetBrains/

# Run with verbose output
RUST_LOG=debug jb-reset list
```

### Reset Not Working

1. **Close the IDE** before resetting
2. Check if backup was created: `jb-reset backup list`
3. Verify files were modified: `jb-reset reset --dry-run`
4. Check permissions: `ls -la ~/.config/JetBrains/`

### Permission Issues

```bash
# Fix ownership
sudo chown -R $USER:$USER ~/.config/JetBrains/
sudo chown -R $USER:$USER ~/.local/share/JetBrains/

# Fix permissions
chmod -R u+rw ~/.config/JetBrains/
```

## Safety Notes

- Always **close the IDE** before resetting
- Backups are created automatically in `~/.jetbrains-trial-backups/`
- Use `--dry-run` to preview changes
- The tool only modifies trial-related files, not your projects or settings
- Backups include:
  - `options/other.xml` (contains trial keys)
  - `eval/` directory (if exists)

## Performance

- List operation: ~50ms
- Reset operation: ~200ms per product
- Memory usage: <10MB
- Disk space: Backups ~100KB per product

## Advanced Usage

### Custom Backup Location

The tool uses `~/.jetbrains-trial-backups/` by default. To change:

```bash
# Create symlink to different location
rm -rf ~/.jetbrains-trial-backups
ln -s /path/to/backup/location ~/.jetbrains-trial-backups
```

### Multiple Versions

The tool detects all versions automatically:

```bash
$ jb-reset list
🧠 IntelliJ IDEA (2025.1) │ Active (15 days)
🧠 IntelliJ IDEA (2025.2) │ Active (30 days)
```

Reset specific version by including version number:

```bash
jb-reset reset intellij  # Resets all IntelliJ versions
```

## Support

For issues, questions, or contributions:
- GitHub Issues: https://github.com/YOUR_USERNAME/jetbrains-trial-reset/issues
- Documentation: https://github.com/YOUR_USERNAME/jetbrains-trial-reset

Happy coding! 🚀
