---
id: authentication
title: Authentication Guide
sidebar_position: 4
---

# 🔑 Authentication Guide

To query problem details, list solved states, test code against judge servers, and submit solutions, `leetrs` requires authentication with [LeetCode.com](https://leetcode.com).

`leetrs` uses session cookies (`LEETCODE_SESSION` and `csrftoken`) to authenticate all API requests.

---

## 🔒 Interactive Auth (`leetrs auth`)

Run the interactive authentication command:

```bash
leetrs auth
```

You will see an interactive prompt powered by `dialoguer`:

```text
🔒 LeetCode Authentication

? How would you like to authenticate?
❯ Paste tokens manually
  Extract from Firefox
  Extract from Chrome
```

---

## 🌐 Option 1: Automatic Cookie Extraction

If you are already logged into LeetCode in **Chrome** or **Firefox**, select:
- `Extract from Firefox`
- `Extract from Chrome`

`leetrs` uses the [`rookie`](https://crates.io/crates/rookie) crate to decrypt and extract active browser cookies without requiring browser extensions or external helpers.

:::tip Prerequisites for Automatic Extraction
- You must be logged into [leetcode.com](https://leetcode.com) in the chosen browser.
- Close the browser if keyrings/databases are locked by exclusive file locks on Linux.
:::

---

## ✍️ Option 2: Manual Token Fallback

If you are using containerized browsers (Snap / Flatpak), custom profile paths, Brave, Arc, or a headless server:

1. Select `Paste tokens manually`.
2. Open [leetcode.com](https://leetcode.com) in your browser and open **Developer Tools** (`F12` or `Cmd+Option+I`).
3. Navigate to **Application** (Chrome) or **Storage** (Firefox) $\rightarrow$ **Cookies** $\rightarrow$ `https://leetcode.com`.
4. Copy the values of:
   - `LEETCODE_SESSION`
   - `csrftoken`
5. Paste them into the interactive prompts in `leetrs auth`.

---

## 🔍 Checking Authentication Status (`leetrs status`)

Verify your active credentials at any time:

```bash
leetrs status
```

Output:
```text
✅ Currently authenticated!
🔑 csrftoken:
d9a8...f102

🔑 LEETCODE_SESSION:
eyJhbGciOi...
```

Credentials are saved in `~/.config/leetrs/credentials.json` (or `%APPDATA%\leetrs\credentials.json` on Windows).
