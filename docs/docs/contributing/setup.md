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

Run the full Rust unit and integration test suite before submitting changes:

```bash
cargo test
```

Check code formatting and linting:

```bash
cargo fmt --check
cargo clippy -- -D warnings
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
