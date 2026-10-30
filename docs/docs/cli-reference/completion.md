---
id: completion
title: completion
sidebar_position: 6
---

# 🐚 `leetrs completion`

Generates shell autocomplete scripts for `bash`, `zsh`, `fish`, `elvish`, and `powershell`.

---

## Usage

```bash
leetrs completion <SHELL>
```

---

## Arguments

| Argument | Supported Values | Description |
|---|---|---|
| `<SHELL>` | `bash`, `zsh`, `fish`, `elvish`, `powershell` | Target shell environment |

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

### Elvish Setup

Add completion script to your `~/.config/elvish/rc.elv`:

```elvish
eval (leetrs completion elvish | slurp)
```

### PowerShell Setup

Add completion script to your PowerShell profile (`$PROFILE`):

```powershell
leetrs completion powershell | Out-String | Invoke-Expression
```
