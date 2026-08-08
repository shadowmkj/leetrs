---
id: tui
title: tui
sidebar_position: 3
---

# 🖥️ `leetrs tui`

Launches the interactive Ratatui terminal browser for searching, filtering, and picking LeetCode problems.

---

## Usage

```bash
leetrs tui [LANGUAGE]
# — or simply —
leetrs [LANGUAGE]
```

---

## Arguments

| Argument | Type | Optional | Description |
|---|---|---|---|
| `LANGUAGE` | `string` | Yes | Temporary language override for the current TUI session (e.g. `rust`, `python3`, `mysql`). Overrides `config.toml` for stub generation. |

---

## Examples

### Default TUI Launch

```bash
leetrs tui
```

### Launch TUI with Rust Override

```bash
leetrs tui rust
```

---

## Features

- **Local Data Caching**: Problem metadata is cached in `~/.local/share/leetrs/data.json` for immediate loading.
- **Fuzzy Search**: Filter thousands of problems in real-time by typing `/`.
- **Difficulty & Topic Overlays**: Filter by Easy (`1`), Medium (`2`), Hard (`3`), or Topic Tags (`t`).
- **Direct Editor Launch**: Pressing `Enter` automatically triggers `leetrs pick` for the selected item.
