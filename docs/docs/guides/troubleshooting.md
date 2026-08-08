---
id: troubleshooting
title: FAQ & Troubleshooting
sidebar_position: 2
---

# ❓ FAQ & Troubleshooting

Common questions and resolution steps for issues encountered while using `leetrs`.

---

## 🔒 Authentication Issues

### Cookie extraction fails during `leetrs auth`

**Symptoms**: Error message `Failed to extract cookies from Firefox/Chrome`.

**Solutions**:
1. Open Chrome/Firefox and verify that you are logged into [leetcode.com](https://leetcode.com).
2. If using containerized browsers (Snap or Flatpak on Ubuntu/Debian), browser profiles are stored in non-standard sandboxed locations. Choose **"Paste tokens manually"** in `leetrs auth`.
3. If using Firefox on Linux, ensure Firefox is closed so the SQLite cookie database file lock is released.

---

## ⚡ Neovim & Editor Issues

### Neovim fails to launch after `leetrs pick`

**Symptoms**: Error `failed to launch nvim. is it installed and in your path?`

**Solutions**:
1. Check if `nvim` is installed and in your `$PATH`:
   ```bash
   which nvim
   ```
2. If using another editor (e.g. VS Code), set `editor = "code"` inside `~/.config/leetrs/config.toml`.

---

## 💾 Cache & Data Issues

### How do I refresh problem cache?

**Solutions**:
If problem metadata or solved statuses appear outdated:

```bash
# Delete local data cache
rm -rf ~/.local/share/leetrs/data.json

# Re-run TUI to trigger sync
leetrs tui
```
