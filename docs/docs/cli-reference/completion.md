---
id: completion
title: completion
sidebar_position: 6
---

# 🐚 `leetrs completion`

Generates shell autocomplete scripts for `bash`, `zsh`, and `fish`.

---

## Usage

```bash
leetrs completion <SHELL>
```

---

## Arguments

| Argument | Supported Values | Description |
|---|---|---|
| `<SHELL>` | `bash`, `zsh`, `fish` | Target shell environment |

---

## Setup Instructions

### Zsh Setup

Add completion script to your `~/.zshrc`:

```zsh
# Generate and source zsh autocompletion
eval "$(leetrs completion zsh)"
```

### Bash Setup

Add completion script to your `~/.bashrc`:

```bash
eval "$(leetrs completion bash)"
```

### Fish Setup

Save completion script to fish completions directory:

```fish
leetrs completion fish > ~/.config/fish/completions/leetrs.fish
```
