---
id: quickstart
title: Quickstart Guide
sidebar_position: 1
---

# ⚡ Quickstart Guide

Get up and running with `leetrs` in under 5 minutes.

---

## 📋 Prerequisites

Before starting, ensure you have:
1. A valid [LeetCode](https://leetcode.com) account.
2. Active login session in **Google Chrome** or **Mozilla Firefox** (for automatic cookie extraction).
3. **Neovim** (0.9+) installed and available as `nvim` in your `$PATH`.

---

## 🛠️ Step-by-step Setup

### Step 1: Install `leetrs`

If you have Rust installed, install via `cargo`:

```bash
cargo install leetrs
```

*(Alternatively, see the [Installation Guide](./installation.md) for Homebrew or Shell Script options).*

---

### Step 2: Authenticate with LeetCode

Run the interactive authentication wizard:

```bash
leetrs auth
```

Select your browser (e.g. `Extract from Firefox` or `Extract from Chrome`). `leetrs` will extract your active session cookies (`LEETCODE_SESSION` and `csrftoken`) and save them securely to `~/.config/leetrs/` (or Windows `%APPDATA%\leetrs`).

Verify authentication state:

```bash
leetrs status
```

---

### Step 3: Browse Problems via TUI

Launch the interactive Terminal User Interface:

```bash
leetrs tui
```

Within the TUI:
- Press `/` to start fuzzy searching for problems (e.g., `two sum`).
- Press `1`, `2`, or `3` to filter by **Easy**, **Medium**, or **Hard** difficulty.
- Press `t` to open the topic overlay modal and filter by tags (e.g. `Array`, `Hash Table`).
- Press `Enter` to select a problem.

---

### Step 4: Pick a Problem Directly

Alternatively, pick a problem directly by numeric ID or slug from your terminal:

```bash
leetrs pick 1
# — or —
leetrs pick two-sum --language rust
```

`leetrs` will:
1. Fetch the problem description and code stub from LeetCode.
2. Generate local files: `two_sum.md` (description) and `two_sum.rs` (code template).
3. Open **Neovim** in vertical split view: `two_sum.md` on the left, `two_sum.rs` on the right.

---

### Step 5: Test and Submit

Write your solution inside Neovim, save and exit (`:wq`).

#### Run sample test cases:

```bash
leetrs test two_sum.rs
```

#### Submit for official judging:

```bash
leetrs submit two_sum.rs
```

You'll see real-time judging results, execution status, runtime percentiles, and memory usage directly in your terminal! 🎉
