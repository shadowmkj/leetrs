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
leetrs auth
```

### Prompt Options

```text
🔒 LeetCode Authentication

? How would you like to authenticate?
❯ Paste tokens manually
  Extract from Firefox
  Extract from Chrome
```

1. **Paste tokens manually**: Prompts for `LEETCODE_SESSION` and `csrftoken` strings.
2. **Extract from Firefox**: Decrypts Firefox profile cookies automatically.
3. **Extract from Chrome**: Decrypts Chrome profile cookies automatically.

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
