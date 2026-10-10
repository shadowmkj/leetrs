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
❯ Auto-detect browser (Recommended)
  Paste cURL / Cookie header manually
  Extract from Firefox
  Extract from Chrome
  Extract from Brave
  Extract from Edge
  Extract from Arc
```

---

## 🌐 Option 1: Automatic Cookie Extraction

If you are already logged into LeetCode in any supported browser (**Firefox**, **Chrome**, **Brave**, **Edge**, **Arc**, or privacy forks like **LibreWolf**, **Floorp**, **Waterfox**), select:
- `Auto-detect browser (Recommended)`
- Or specify your browser directly from the menu.

`leetrs` uses the internal [`cookie-extract`](https://github.com/shadowmkj/leetrs/tree/main/crates/cookie-extract) crate to safely copy and decrypt active browser cookies across macOS, Arch Linux, Debian/Ubuntu, Flatpak, and Snap without requiring browser extensions or external helpers.

:::tip[Prerequisites for Automatic Extraction]
- You must be logged into [leetcode.com](https://leetcode.com) in the chosen browser.
- Multi-profile and live sessions are supported safely via isolated temporary database staging.
:::

---

## ✍️ Option 2: Single-Paste cURL / Header Fallback

If you are on a headless server, remote SSH session, or unsupported environment:

1. Select `Paste cURL / Cookie header manually`.
2. Open [leetcode.com](https://leetcode.com) in your browser and open **Developer Tools** (`F12` or `Cmd+Option+I`) → **Network** tab.
3. Right click any `graphql` or `api` request → **Copy** → **Copy as cURL** (or copy the raw `Cookie:` header).
4. Paste the entire string into the single prompt in `leetrs auth`. `leetrs` will automatically parse `LEETCODE_SESSION` and `csrftoken`.

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
