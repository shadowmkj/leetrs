---
id: installation
title: Installation Guide
sidebar_position: 2
---

# 📦 Installation Guide

`leetrs` supports multiple installation methods across **macOS**, **Linux**, and **Windows**. Choose the method that best matches your development setup.

---

## 💻 System Requirements

| Requirement | Supported Version | Notes |
|---|---|---|
| **OS** | macOS, Linux, Windows | macOS 11+, Ubuntu 20.04+, Windows 10+ |
| **Neovim** | `0.9+` | Required for automatic vertical split editing |
| **Browser** | Chrome / Firefox | Used for automatic cookie extraction |
| **Rust Toolchain** | `1.70+` | Required for `cargo install` or building from source |

---

## 🚀 Installation Options

### Option 1: Cargo Install (Recommended)

If you have Rust and Cargo installed, run:

```bash
cargo install leetrs
```

:::tip PATH Verification
Ensure `~/.cargo/bin` is in your system `$PATH`:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```
:::

---

### Option 2: Homebrew (macOS & Linux)

Install via the official Homebrew tap:

```bash
brew install shadowmkj/tap/leetrs
```

To update `leetrs` in the future:

```bash
brew update && brew upgrade leetrs
```

---

### Option 3: Shell Installer Script

Download and install pre-compiled release binaries directly without needing a Rust toolchain:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/shadowmkj/leetrs/releases/download/v1.0.18/leetrs-installer.sh | sh
```

---

### Option 4: Build from Source

Build the binary from the latest source repository:

```bash
# 1. Clone the repository
git clone https://github.com/shadowmkj/leetrs.git
cd leetrs

# 2. Build optimized release binary
cargo build --release

# 3. Copy binary to system PATH
cp target/release/leetrs ~/.cargo/bin/
# — or —
sudo cp target/release/leetrs /usr/local/bin/

# 4. Verify installation
leetrs --version
```

---

## 🔍 Verifying Installation

Verify that `leetrs` is correctly installed and accessible in your shell:

```bash
leetrs --version
```

Expected output:
```text
leetrs 1.0.20
```
