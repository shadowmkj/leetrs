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

**Symptoms**: Error message `Failed to extract cookies from <browser>`.

**Solutions**:
1. Open your browser and verify that you are logged into [leetcode.com](https://leetcode.com).
2. Run with verbose diagnostic logging: `leetrs --verbose auth` (or `leetrs -v auth`) to see exactly which database paths and keyring keys were scanned.
3. If using an unsupported browser or SSH session, choose **"Paste cURL / Cookie header manually"**: in your browser's Developer Tools (`F12` → **Network** tab), right click any request → **Copy as cURL**, and paste directly into `leetrs auth`.

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
