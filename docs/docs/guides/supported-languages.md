---
id: supported-languages
title: Supported Languages
sidebar_position: 1
---

# 🌐 Supported Languages

`leetrs` supports key programming languages on LeetCode, automatically inferring file extensions, comment syntax, and stub formats.

:::warning[Language Support Status]
Currently, only **Python 3** (`python3`) and **Rust** (`rust`) are fully tested and working as expected. Other languages may contain bugs and will be fixed in future releases.
:::

---

## 📋 Language Table

| Display Name | LeetCode Slug | File Extension | Comment Style | Metadata Header Example |
|---|---|---|---|---|
| **Rust** | `rust` | `.rs` | `//` | `// id=1 slug=two-sum lang=rust` |
| **Python 3** | `python3` | `.py` | `#` | `# id=1 slug=two-sum lang=python3` |
| **Pandas** | `pythondata` | `.py` | `#` | `# id=1 slug=two-sum lang=pythondata` |
| **MySQL** | `mysql` | `.sql` | `#` / `--` | `# id=1 slug=two-sum lang=mysql` |
| **PostgreSQL** | `postgresql` | `.sql` | `--` | `-- id=1 slug=two-sum lang=postgresql` |

---

## 🔍 Language Resolution Rules

When running `leetrs test` or `leetrs submit`:
1. `leetrs` inspects the top header line of the file.
2. If `lang=...` is found, `leetrs` uses that exact language slug.
3. If no header line exists, `leetrs` falls back to inferring language from the file extension (`.rs` → `rust`, `.py` → `python3`, `.sql` → `mysql`).
