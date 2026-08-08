---
id: overview
title: Architecture Overview
sidebar_position: 1
---

# 🏗️ Architecture Overview

`leetrs` is designed as a modular, asynchronous Rust crate focused on performance, clean separation of concerns, and robust error handling.

---

## 🧱 Module Structure

The Rust codebase (`src/`) is organized into dedicated modules:

| Module | Source File | Description |
|---|---|---|
| **`auth`** | `src/auth.rs` | Encapsulates browser cookie extraction (`rookie`) and credential persistence |
| **`client`** | `src/client.rs` | Authenticated HTTP client managing LeetCode REST and GraphQL API queries |
| **`models`** | `src/models/` | Serde-compatible data models (`ProblemSummary`, `Question`, `Submission`, `Language`) |
| **`picker`** | `src/picker.rs` | Main workflow orchestrator connecting client, caching, disk I/O, and editor launching |
| **`tui`** | `src/tui/` | Ratatui rendering engine, screen state management, and custom widget components |
| **`services`** | `src/services/` | Polling and submission handling logic |
| **`cache`** | `src/cache.rs` | Disk caching service for problem list metadata (`data.json`) |
| **`config`** | `src/config.rs` | Global configuration manager backed by `OnceLock<Config>` |
| **`format`** | `src/format.rs` | Terminal colorizing and result formatting utilities |
| **`error`** | `src/error.rs` | Custom error types implemented with `thiserror` (`EngineError`) |

---

## 🔄 Core Execution Pipeline

```mermaid
sequenceDiagram
    autonumber
    actor User
    participant CLI as main.rs / commands.rs
    participant Auth as auth.rs
    participant Config as config.rs
    participant Picker as picker.rs
    participant Client as client.rs
    participant Editor as Neovim Process

    User->>CLI: leetrs pick 1
    CLI->>Auth: LeetCodeCredentials::load()
    Auth-->>CLI: Credentials
    CLI->>Config: CONFIG.get()
    Config-->>CLI: Config Options
    CLI->>Picker: Picker::new(client).pick(1)
    Picker->>Client: get_question_by_id(1)
    Client-->>Picker: Question Details & Code Snippets
    Picker->>Picker: Write two_sum.md & two_sum.rs to disk
    Picker->>Editor: Launch nvim two_sum.md -c "vsplit two_sum.rs"
    Editor-->>User: Side-by-side terminal split
```
