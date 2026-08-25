---
id: setup
title: Contributor Guide & Setup
sidebar_position: 1
---

# 🤝 Contributor Guide & Setup

Thank you for contributing to `leetrs`! This guide walks through setting up your local development environment, running tests, and submitting pull requests.

---

## 🛠️ Local Development Setup

### 1. Prerequisites

- **Rust Toolchain**: `1.70+` (`rustup`)
- **Neovim**: `0.9+`
- **Node.js & pnpm**: Required only for editing Docusaurus documentation (`docs/`)
- **cargo-llvm-cov** *(optional)*: Required for generating coverage reports (`cargo install cargo-llvm-cov`)

### 2. Clone & Build

```bash
# Clone the repository
git clone https://github.com/shadowmkj/leetrs.git
cd leetrs

# Build debug binary
cargo build

# Run leetrs locally
cargo run -- tui
```

---

## 🧪 Testing & Verification

`leetrs` includes a built-in `xtask` runner to verify quality gates locally before submitting PRs:

### Run Full CI Quality Gate
```bash
cargo xtask ci
```
This runs:
1. **Format check**: `cargo fmt --all --check`
2. **Clippy analysis**: `cargo clippy --all-targets -- -D warnings`
3. **Test suite**: `cargo test`

### Individual Quality Tasks
```bash
# Auto-format all code (or check only with `cargo xtask fmt --check`)
cargo xtask fmt

# Run Clippy linter with warnings denied
cargo xtask clippy

# Run the unit test suite
cargo xtask test

# Generate code coverage HTML report (requires cargo-llvm-cov)
cargo xtask coverage --html
```

---

## 📝 Commit Conventions

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat(tui): add custom theme option`
- `fix(picker): fix slug fallback when offline`
- `docs(docusaurus): update quickstart guide`

---

## 📚 Editing Docusaurus Documentation

Documentation is built with Docusaurus located in `docs/`:

```bash
cd docs
pnpm install
pnpm start
```

Build static production docs:

```bash
pnpm build
```
