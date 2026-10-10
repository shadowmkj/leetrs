# 🚀 leetrs

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/leetrs.svg?style=flat-square&logo=rust)](https://crates.io/crates/leetrs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Beta-orange.svg?style=flat-square)](https://github.com/shadowmkj/leetrs)
[![Neovim](https://img.shields.io/badge/Neovim-0.9%2B-green.svg?style=flat-square&logo=neovim)](https://neovim.io/)
[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey.svg?style=flat-square)](https://github.com/shadowmkj/leetrs)
[![CI](https://github.com/shadowmkj/leetrs/actions/workflows/ci.yml/badge.svg)](https://github.com/shadowmkj/leetrs/actions/workflows/ci.yml)
[![Codecov](https://img.shields.io/codecov/c/github/shadowmkj/leetrs?style=flat-square&logo=codecov)](https://codecov.io/gh/shadowmkj/leetrs)
[![Built with Ratatui](https://ratatui.rs/built-with-ratatui/badge.svg)](https://ratatui.rs)

**A blazing-fast, Rust-powered CLI engine and TUI for solving LeetCode problems natively in Neovim.**

[Features](#-features) • [Installation](#%EF%B8%8F-installation) • [Quick Start](#-quick-start) • [CLI Reference](#-cli-command-reference) • [TUI Keybindings](#-tui-keybindings)

</div>


---

> [!WARNING]
>
> This is still a **work in progress**. Breaking changes to commands and configuration
> might happen.

## 📋 Table of Contents

- [📌 Overview](#-overview)
- [✨ Features](#-features)
- [🔧 Prerequisites](#-prerequisites)
- [🛠️ Installation](#%EF%B8%8F-installation)
  - [Option 1 — Cargo (Recommended)](#option-1--cargo-install-recommended)
  - [Option 2 — Homebrew (macOS & Linux)](#option-2--homebrew-macos--linux)
  - [Option 3 — Shell Installer Script](#option-3--curl-installer-quickstart)
  - [Option 4 — Build from Source](#option-4--build-from-source)
- [⚙️ Configuration](#%EF%B8%8F-configuration)
  - [Config File Location](#config-file-location)
  - [Configuration Options](#configuration-options)
  - [Example `config.toml`](#example-configuration)
- [⚡ Quick Start](#-quick-start)
- [💻 CLI Command Reference](#-cli-command-reference)
- [⌨️ TUI Keybindings](#-tui-keybindings)
- [🌐 Supported Languages](#-supported-languages)
- [❓ Troubleshooting & FAQ](#-troubleshooting--faq)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)

---

## 📌 Overview

`leetrs` is built for developers who live in the terminal and rely on **Neovim**. It strips away browser friction by providing a complete LeetCode workflow directly within your terminal:

1. **Browse & Search:** Search problems using an interactive terminal interface (TUI) with fuzzy search, difficulty filters, and topic overlays.
2. **Fetch & Format:** Instantly generate clean Markdown problem descriptions and language-specific code templates.
3. **Edit in Neovim:** Launch Neovim automatically with a vertical split (`vsplit`) placing the problem description and code template side-by-side.
4. **Test & Submit:** Asynchronously execute sample test cases or submit your solution for full judging, with colorized terminal feedback and runtime statistics.

https://github.com/user-attachments/assets/86783e7e-afc6-449a-828b-c29e34fa9dbb

---

## ✨ Features

- 🖥️ **Interactive TUI Browser (`leetrs tui` / `leetrs`)**
  - Instant fuzzy search across thousands of LeetCode problems.
  - Difficulty filters (**Easy**, **Medium**, **Hard**) and topic tag overlays.
  - Visual status indicators for solved status (`ACCEPTANCE`), subscription locks, and premium gates.
  - Quick keys to toggle help popups or open selected problems directly in your system browser.

- 🔑 **Intelligent Cookie Authentication (`leetrs auth`)**
  - Automatically extracts `LEETCODE_SESSION` and `csrftoken` cookies across **Firefox**, **Chrome**, **Brave**, **Edge**, and **Arc** on macOS and Linux (including Snap, Flatpak, and privacy forks).
  - Supports single-paste cURL commands and raw `Cookie:` headers for instant manual authentication.

- 📝 **Frictionless Problem Fetching (`leetrs pick`)**
  - Fetch problems using URL slugs (e.g., `two-sum`) or numerical IDs (e.g., `1`).
  - Converts raw LeetCode HTML into clean, wrapped terminal Markdown text.
  - Generates idiomatic code files (e.g., `two_sum.rs`, `two_sum.py`, `two_sum.sql`) with pre-populated function stubs and metadata headers.

- ⚡ **Native Neovim Integration**
  - Seamlessly launches Neovim and opens a side-by-side vertical split (`vsplit`).
  - Problem description sits in the left pane; code solution sits in the right pane.

- 🧪 **Async Testing & Submission Engine (`leetrs test` / `leetrs submit`)**
  - Run code against sample test cases locally without polluting your official submission history.
  - Submit directly to LeetCode's judging servers and view real-time judge results, runtime/memory percentiles, and compiler error logs.

---

## 🔧 Prerequisites

| Requirement | Version | Notes |
|---|---|---|
| [Neovim](https://neovim.io/) | 0.9+ | Must be available as `nvim` in your `$PATH` |
| [LeetCode Account](https://leetcode.com/) | — | Required for authentication and submission |
| Supported Browser | Firefox, Chrome, Brave, Edge, Arc | Used for automatic browser cookie extraction |
| [Rust Toolchain](https://rustup.rs/) | 1.70+ | Required only if installing via Cargo or building from source |

---

## 🛠️ Installation

Choose the installation method that best fits your environment.

### Option 1 — Cargo Install (Recommended)

If you already have Rust and Cargo installed:

```bash
cargo install leetrs
```

> [!TIP]
> Ensure `~/.cargo/bin` is present in your system `$PATH`.

---

### Option 2 — Homebrew (macOS & Linux)

Install via Homebrew tap:

```bash
brew install shadowmkj/tap/leetrs
```

---

### Option 3 — `curl` Installer (Quickstart)

Download and install pre-compiled binaries for your platform directly without requiring a Rust installation:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/shadowmkj/leetrs/releases/download/v1.1.0/leetrs-installer.sh | sh
```

---

### Option 4 — Build from Source

Build the latest release binary from source code:

```bash
# 1. Clone the repository
git clone https://github.com/shadowmkj/leetrs.git
cd leetrs

# 2. Build optimized release binary
cargo build --release

# 3. Copy binary to executable PATH
cp target/release/leetrs ~/.cargo/bin/
# — or —
sudo cp target/release/leetrs /usr/local/bin/

# 4. Verify installation
leetrs --version
```

---

## ⚙️ Configuration

`leetrs` uses a **TOML** configuration file to customize editor launcher commands, default languages, and display settings.

### Config File Location

The configuration file (`config.toml`) is stored in standard system directories:

| OS | Default Path |
|---|---|
| **Linux / macOS** | `~/.config/leetrs/config.toml` |
| **Windows** | `%APPDATA%\leetrs\config.toml` |

> [!NOTE]
> The configuration file and parent directories are automatically created on first execution if they do not exist.

### Configuration Options

| Option | Type | Default | Description |
|---|---|---|---|
| `editor` | `string` | `"nvim"` | Command executed when picking a problem. Use `"nvim"` or `"vim"` for terminal split, or `"code"` for VS Code. |
| `language` | `string` | `"python3"` | Default language for code stub generation (`"rust"`, `"python3"`, `"pythondata"`, `"mysql"`, `"postgresql"`). |
| `show_description` | `boolean` | `true` | When `true`, opens problem description alongside the code stub in split view. |

### Example Configuration

```toml
# ~/.config/leetrs/config.toml
editor = "nvim"
language = "rust"
show_description = true
```

---

## ⚡ Quick Start

Follow these steps to get up and running:

```bash
# 1. Authenticate with LeetCode (extracts browser cookies automatically)
leetrs auth

# 2. Check authentication status
leetrs status

# 3. Launch the interactive TUI problem browser
leetrs tui

# 4. Alternatively, pick a problem directly by slug or numeric ID
leetrs pick two-sum
leetrs pick 1

# 5. Solve the problem in Neovim, then test against example testcases
leetrs test two_sum.rs

# 6. Submit solution for full judging
leetrs submit two_sum.rs
```

---

## 💻 CLI Command Reference

`leetrs` provides a suite of CLI subcommands:

| Subcommand | Arguments / Flags | Description |
|---|---|---|
| `auth` | `[-v, --verbose]` | Interactively extracts cookies across browsers or accepts single-paste cURL / Cookie headers. |
| `tui` | `[language]` | Launches the interactive TUI problem browser. Optionally set temporary language override. |
| `pick` | `<identifier> [language] [-p, --preview]` | Fetches problem, creates files, and opens editor. Identifier can be a slug (`two-sum`) or ID (`1`). `--preview` prints markdown to stdout. |
| `test` | `<file>` | Tests local solution file against sample test cases without official submission. |
| `submit` | `<file>` | Submits solution file to LeetCode for full judging and statistics. |
| `status` | — | Displays active authentication state and token information. |
| `completion` | `<shell>` | Generates shell autocomplete scripts (`bash`, `zsh`, `fish`). |

---

## ⌨️ TUI Keybindings

When navigating the interactive TUI (`leetrs tui`), the following keybindings are available:

### Navigation & Problem Selection

| Keybinding | Action |
|---|---|
| `j` / `Down` | Move selection down one item |
| `k` / `Up` | Move selection up one item |
| `g g` | Jump to top of problem list |
| `G` | Jump to bottom of problem list |
| `Ctrl+d` | Page down (problem list & topic filter) |
| `Ctrl+u` | Page up (problem list & topic filter) |
| `Enter` | Pick selected problem and open in configured editor |
| `o` | Open selected problem in default web browser |

### Filtering & Modes

| Keybinding | Action |
|---|---|
| `/` | Focus search bar (fuzzy search by title / ID) |
| `1`| Easy Problems|
| `2` | Medium Problems|
| `3` | Hard Problems|
| `4` | All difficulties|
| `t` | Open topic overlay filter modal (real-time topic search, `Ctrl+j`/`Ctrl+k` navigation, `Enter`/`Space` toggle) |
| `Tab` | Cycle focus between UI components |
| `?` | Toggle interactive Help overlay |
| `Esc` | Clear search filter / exit edit mode / dismiss popup |
| `q` | Quit TUI |

---

## 🌐 Supported Languages

`leetrs` supports major LeetCode programming languages, automatically inferring extensions and comment formats:

| Language Name | LeetCode Slug | File Extension | Meta Header Comment Style |
|---|---|---|---|
| **Rust** | `rust` | `.rs` | `// id=1 slug=two-sum lang=rust` |
| **Python 3** | `python3` | `.py` | `# id=1 slug=two-sum lang=python3` |
| **Pandas** | `pythondata` | `.py` | `# id=1 slug=two-sum lang=pythondata` |
| **MySQL** | `mysql` | `.sql` | `# id=1 slug=two-sum lang=mysql` |
| **PostgreSQL** | `postgresql` | `.sql` | `-- id=1 slug=two-sum lang=postgresql` |

---

## ❓ Troubleshooting & FAQ

<details>
<summary><b>Automatic cookie extraction fails during <code>leetrs auth</code></b></summary>

- Ensure you are logged into [leetcode.com](https://leetcode.com) in your selected browser (Firefox, Chrome, Brave, Edge, Arc).
- Run `leetrs --verbose auth` (or `leetrs -v auth`) to view detailed diagnostic logging and database discovery paths.
- Alternatively, select **"Paste cURL / Cookie header manually"**: in your browser's Developer Tools (`F12` → **Network** tab), right click any request → **Copy as cURL**, and paste into `leetrs auth`.
</details>

<details>
<summary><b>Neovim fails to launch after <code>leetrs pick</code></b></summary>

- Verify that Neovim is installed and accessible as `nvim` in your system `$PATH`:
  ```bash
  which nvim
  ```
- If using another editor, set `editor = "code"` or your preferred editor binary in `~/.config/leetrs/config.toml`.
</details>

<details>
<summary><b>How do I refresh cached problem lists or user profile data?</b></summary>

- `leetrs` caches problem metadata locally in your user data directory (`~/.local/share/leetrs/` or OS equivalent).
- If problem cache becomes corrupted, delete the cache file to trigger a fresh sync on next run:
  ```bash
  rm -rf ~/.local/share/leetrs/data.json
  ```
</details>

---

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, development setup, and pull request workflow.

1. Fork the Repository
2. Create your Feature Branch (`git checkout -b feature/amazing-feature`)
3. Commit your Changes (`git commit -m "feat: add amazing feature"`)
4. Push to the Branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📄 License

Distributed under the MIT License. See [LICENSE](LICENSE) for more details.

Developed with ❤️ by [shadowmkj](https://github.com/shadowmkj).
