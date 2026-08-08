---
id: configuration
title: Configuration Guide
sidebar_position: 3
---

# ⚙️ Configuration Guide

`leetrs` uses a simple **TOML** configuration file (`config.toml`) to manage default programming languages, editor launch commands, and display options.

---

## 📁 Configuration File Location

The configuration file is stored in standard system directories according to XDG / OS conventions:

| Platform | Configuration Path |
|---|---|
| **Linux / macOS** | `~/.config/leetrs/config.toml` |
| **Windows** | `%APPDATA%\leetrs\config.toml` |

:::note Automatic File Creation
If `config.toml` does not exist when `leetrs` is invoked, `leetrs` creates the directory and populates `config.toml` with default values automatically.
:::

---

## 🛠️ Configuration Options

| Parameter | Type | Default | Description |
|---|---|---|---|
| `editor` | `string` | `"nvim"` | Command executed when picking a problem. Supports `"nvim"`, `"vim"`, `"code"`, or any custom terminal editor binary. |
| `language` | `string` | `"python3"` | Default language slug used when fetching problem code templates (`"rust"`, `"python3"`, `"pythondata"`, `"mysql"`, `"postgresql"`). |
| `show_description` | `boolean` | `true` | When `true`, opens the problem description alongside the code template in split view. |

---

## 📄 Example `config.toml`

### Default Configuration

```toml
# ~/.config/leetrs/config.toml
editor = "nvim"
language = "python3"
show_description = true
```

### Rust Developer Setup

```toml
# ~/.config/leetrs/config.toml
editor = "nvim"
language = "rust"
show_description = true
```

### VS Code Integration Setup

```toml
# ~/.config/leetrs/config.toml
editor = "code"
language = "python3"
show_description = true
```

---

## 🔧 Behavior Notes

- **Neovim & Vim Splitting**: When `editor` contains `"nvim"` or `"vim"` and `show_description` is `true`, `leetrs` executes:
  ```bash
  nvim <desc_file.md> -c "vsplit <code_file.ext>"
  ```
  This creates a vertical side-by-side split automatically.
- **Other Editors**: For editors like `code` (VS Code), `leetrs` passes both file paths as positional arguments:
  ```bash
  code <desc_file.md> <code_file.ext>
  ```
