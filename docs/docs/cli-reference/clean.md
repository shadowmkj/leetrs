---
id: clean
title: clean & reset
sidebar_position: 8
---

# 🧹 `leetrs clean`

Invalidates and resets local application cache, problem metadata, and stored authentication credentials.

---

## 📖 Overview

`leetrs clean` (and its alias `leetrs reset`) completely purges locally cached state from your system. This command is designed to recover from corrupted cache files, force a clean synchronization with LeetCode's servers, or securely log out by removing stored session tokens.

### Target Files

Running `leetrs clean` safely targets and deletes the following files:

| File | Location | Content |
|---|---|---|
| `data.json` | OS data directory | Cached problem catalog, difficulty levels, acceptance rates, and topic tags |
| `user.json` | OS data directory | Cached user profile information |
| `credentials.json` | OS config / data directory | Stored `LEETCODE_SESSION` and `csrftoken` authentication cookies |

---

## ⚡ Usage

```bash
# Clean local cache and stored credentials
leetrs clean

# Or using the reset alias
leetrs reset
```

---

## 📋 Example Output

When cache or credential files are present:

```text
🧹 Cleaning local cache and credentials...

  ✓ Removed data.json (problem cache)
  ✓ Removed user.json (problem cache)
  ✓ Removed credentials.json (stored credentials)

✨ Successfully cleared 3 file(s). Local state has been reset.
```

When everything is already clean:

```text
🧹 Cleaning local cache and credentials...

✨ Everything is already clean. No cache or credential files found.
```

---

## 💡 When to Use

1. **Corrupted Problem Cache**: If problem lists fail to render properly or show partial data in the TUI, run `leetrs clean` to trigger a fresh download on the next launch.
2. **Account Logout / Switching**: To clear saved credentials before authenticating with a different LeetCode account.
3. **Stale Metadata**: To force an immediate update of problem lists, acceptance percentages, and tags without waiting for background cache refresh.
