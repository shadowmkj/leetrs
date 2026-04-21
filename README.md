
# 🚀 leetrs

[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/leetrs.svg)](https://crates.io/crates/leetrs)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-beta-orange.svg)]()
[![Neovim](https://img.shields.io/badge/Neovim-0.9%2B-green.svg)](https://neovim.io/)

**leetrs** is a blazing-fast, Rust-powered CLI engine that makes solving LeetCode problems from the terminal a first-class developer experience.

Built specifically for developers who live in the terminal and rely on **Neovim**, `leetrs` strips away the distraction of the browser. It handles intelligent authentication, Markdown problem generation, native editor window splitting, and asynchronous code submission without ever leaving your workflow.

https://github.com/user-attachments/assets/86783e7e-afc6-449a-828b-c29e34fa9dbb

---

## ✨ Features

* **Intelligent Authentication (`leetrs auth`)**
  * Automatically extracts `LEETCODE_SESSION` and `csrftoken` cookies from your active Chrome or Firefox sessions.
  * Secure, hidden manual fallback for containerized browser profiles.
* **Frictionless Problem Fetching (`leetrs pick`)**
  * Fetch problems using their URL slug (e.g., `two-sum`) or standard numerical ID (e.g., `1`).
  * Automatically parses LeetCode's raw HTML into clean, readable **Markdown**.
  * Generates idiomatic `snake_case.rs` files containing the exact boilerplate required.
* **Native Neovim Integration**
  * Instantly hijacks the terminal process to launch Neovim.
  * Forces a clean vertical split (`vsplit`) to place your problem description and code side-by-side, bypassing layout quirks from custom dashboards.
* **Async Submission Engine (`leetrs submit`)**
  * Submit your local file directly to LeetCode's execution servers.
  * Automatic ID resolution and CSRF token bypass.
  * Color-coded terminal output for execution results, including Runtime/Memory statistics and detailed compiler error logs.

---

## 🛠️ Installation

Choose the method that best fits your setup.

### Option 1 — `cargo install` (Recommended for Rust users)

Requires [Rust & Cargo](https://rustup.rs/) to be installed.

```bash
cargo install leetrs
```

The binary will be placed in `~/.cargo/bin/`. Make sure that directory is in your `$PATH`.

---

### Option 2 — Homebrew (macOS & Linux)

```bash
brew install shadowmkj/tap/leetrs
```

> **Note:** If the tap isn't published yet, use one of the other methods below while it is being set up.

---

### Option 3 — `curl` Installer (Quickstart, no Rust required)

The installer script downloads the appropriate pre-built binary for your platform and places it in `/usr/local/bin`.

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/shadowmkj/leetrs/releases/download/v1.0.18/leetrs-installer.sh | sh
```

---

### Option 4 — Build from Source

Use this if you want to contribute, run unreleased features, or modify the code yourself.

#### Prerequisites

* [Rust & Cargo](https://rustup.rs/) (stable toolchain)
* [Neovim](https://neovim.io/) accessible as `nvim` in your `$PATH`
* A valid [LeetCode](https://leetcode.com) account

#### Steps

```bash
# 1. Clone the repository
git clone https://github.com/shadowmkj/leetrs.git
cd leetrs

# 2. Build the optimised release binary
cargo build --release

# 3. Move the binary somewhere on your PATH
cp target/release/leetrs ~/.cargo/bin/
# — or —
sudo cp target/release/leetrs /usr/local/bin/
```

Verify the installation:

```bash
leetrs --version
```

---

## ⚡ Quick Start

```bash
# 1. Authenticate with your LeetCode session
leetrs auth

# 2. Pick a problem (by slug or numeric ID)
leetrs pick two-sum
leetrs pick 1

# 3. Open it in Neovim — the problem description opens in a vertical split automatically

# 4. Solve it, then submit
leetrs submit two_sum.rs
```

---

## 🔧 Prerequisites (all methods)

| Requirement | Version | Notes |
|---|---|---|
| [Neovim](https://neovim.io/) | 0.9+ | Must be available as `nvim` in `$PATH` |
| LeetCode account | — | Required for auth & submission |
| Chrome or Firefox | Any | Used for automatic cookie extraction |

---

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on reporting bugs, suggesting features, and submitting pull requests.

---

## 📄 License

MIT © [shadowmkj](https://github.com/shadowmkj)
