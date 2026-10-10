---
id: auth-and-status
title: auth & status
sidebar_position: 2
---

# 🔑 `leetrs auth` & `leetrs status`

Commands for managing LeetCode session credentials.

---

## 🔒 `leetrs auth`

Launches the interactive authentication prompt to obtain and store LeetCode session cookies.

### Usage

```bash
# Standard interactive authentication
leetrs auth

# With verbose diagnostic logging
leetrs --verbose auth
# or
leetrs -v auth
```

### Prompt Options

```text
🔒 LeetCode Authentication

? How would you like to authenticate?
❯ Auto-detect browser (Recommended)
  Paste cURL / Cookie header manually
  Extract from Firefox
  Extract from Chrome
  Extract from Brave
  Extract from Edge
  Extract from Arc
```

1. **Auto-detect browser**: Scans available browser profile databases and automatically extracts cookies from the active session.
2. **Paste cURL / Cookie header manually**: Allows pasting a full copied cURL command or raw `Cookie:` header from browser Developer Tools.
3. **Extract from specific browser**: Direct extraction targeting Firefox, Chrome, Brave, Edge, or Arc.

---

## 🔍 `leetrs status`

Displays the active authentication state and token information saved in `credentials.json`.

### Usage

```bash
leetrs status
```

### Output Example (Authenticated)

```text
✅ Currently authenticated!
🔑 csrftoken:
d9a8b7c6d5e4f3a2109876543210abcd

🔑 LEETCODE_SESSION:
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### Output Example (Unauthenticated)

```text
❌ Not authenticated. No valid credentials found.
Run `leetrs auth` to set up your account.
```
