---
id: caching-and-storage
title: Caching & Storage
sidebar_position: 3
---

# 💾 Caching & Local Storage

`leetrs` minimizes network latency by maintaining a local disk cache for problem metadata and credentials.

---

## 📁 Data Storage Layout

| Resource | Path Location | Description |
|---|---|---|
| **Config** | `~/.config/leetrs/config.toml` | User preferences (`editor`, `language`, `show_description`) |
| **Credentials** | `~/.config/leetrs/credentials.json` | Active session tokens (`csrftoken`, `session_cookie`) |
| **Problem Cache** | `~/.local/share/leetrs/data.json` | Cached problem summaries, difficulties, acceptance, and topic tags |

---

## ⚡ Cache Service Mechanics (`src/cache.rs`)

1. **First Run / Cache Miss**: When `leetrs tui` or `list_problems()` is called for the first time, `CacheService` queries LeetCode GraphQL API to fetch the full problem list and writes it to `data.json`.
2. **Cache Hit**: On subsequent launches, `data.json` is deserialized instantly into memory, reducing boot time to under 10 milliseconds.
3. **Cache Invalidation**: To force a fresh problem synchronization:
   ```bash
   rm -rf ~/.local/share/leetrs/data.json
   ```
