---
id: overview
title: CLI Command Overview
sidebar_position: 1
---

# 💻 CLI Command Overview

`leetrs` provides a clean set of command-line subcommands built with `clap`.

---

## 📋 Command Summary Table

| Subcommand | Arguments / Flags | Purpose |
|---|---|---|
| [`auth`](./auth-and-status.md) | — | Interactively authenticate via Chrome, Firefox, or manual token input |
| [`status`](./auth-and-status.md) | — | Display active authentication status and token previews |
| [`tui`](./tui.md) | `[language]` | Launch interactive Ratatui terminal problem browser |
| [`pick`](./pick.md) | `<identifier> [language] [-p, --preview]` | Fetch problem description & stub, generate local files, and open editor |
| [`test`](./test-and-submit.md) | `<file>` | Test local solution against sample test cases without official submission |
| [`submit`](./test-and-submit.md) | `<file>` | Submit local solution to LeetCode for full judging and statistics |
| [`completion`](./completion.md) | `<shell>` | Generate shell autocomplete scripts (`bash`, `zsh`, `fish`) |

---

## 🌐 Global Flags

| Flag | Short | Description |
|---|---|---|
| `--help` | `-h` | Print help information for `leetrs` or any subcommand |
| `--version` | `-V` | Print version information (`leetrs 1.0.20`) |

---

## ⚡ Default Action

Running `leetrs` with no arguments defaults to launching the interactive TUI:

```bash
leetrs
# is equivalent to:
leetrs tui
```
