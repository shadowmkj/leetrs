# 🍪 cookie-extract

A lightweight, decoupled cross-platform browser cookie extraction and parsing library written in Rust.

## Features

- 🦊 **Mozilla Firefox & Forks**: Plaintext extraction from `cookies.sqlite` across native distros (Arch, Debian, Fedora), XDG paths, Flatpak, Snap, macOS, and forks (LibreWolf, Floorp, Waterfox).
- 🌐 **Chromium Family**: Decrypts cookies for Chrome, Brave, Microsoft Edge, and Arc using OS keyrings (macOS Keychain, Linux SecretService / KWallet, Windows DPAPI) with AES-128-CBC and header stripping.
- 🔒 **Safe SQLite Staging**: Stages isolated temporary copies of SQLite databases and companion `-wal` / `-shm` transaction logs to avoid database lock contention.
- 📋 **Robust Token & Header Parser**: Parses full copied cURL commands (`-H 'Cookie: ...'`, `-b '...'`), raw `Cookie:` headers, and delimited key-value token strings.
- 🔍 **Rich Diagnostics**: Integrated with the `log` crate for full debug traceability.

## Usage

```rust
use cookie_extract::{extract, parse_raw_cookie_header, Browser};

// Auto-detect browser and extract leetcode cookies
let cookies = cookie_extract::auto_extract(&["leetcode.com", ".leetcode.com"])?;

// Extract from specific browser
let chrome_cookies = extract(Browser::Chrome, &["leetcode.com"])?;

// Parse a raw cURL command or Cookie header
let parsed = parse_raw_cookie_header("Cookie: LEETCODE_SESSION=abc; csrftoken=def")?;
```
