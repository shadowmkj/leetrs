---
id: pick
title: pick
sidebar_position: 4
---

# 📝 `leetrs pick`

Fetches problem content and code templates from LeetCode, writes them to disk, and opens your configured editor.

---

## Usage

```bash
leetrs pick <IDENTIFIER> [LANGUAGE] [FLAGS]
```

---

## Arguments & Flags

| Argument / Flag | Type | Required | Description |
|---|---|---|---|
| `<IDENTIFIER>` | `string` / `number` | Yes | Problem slug (e.g. `two-sum`) or numerical ID (e.g. `1`). |
| `[LANGUAGE]` | `string` | No | Language slug override (`rust`, `python3`, `pythondata`, `mysql`, `postgresql`). Defaults to `config.toml` setting. |
| `-p`, `--preview` | `flag` | No | Print problem description Markdown directly to `stdout` without opening an editor. |

---

## Output Files

When `leetrs pick` runs, it creates two local files in the current working directory:

1. **Description file**: `<snake_slug>.md` (e.g., `two_sum.md`)
   - Contains problem title, difficulty level, and Markdown content formatted into 80-column wrapped text.
2. **Code template file**: `<snake_slug>.<ext>` (e.g., `two_sum.rs` or `two_sum.py`)
   - Pre-populated function stub with a metadata header comment containing problem ID, slug, and language.

---

## Examples

### Pick Problem #1 (Two Sum)

```bash
leetrs pick 1
```

### Pick Problem by Slug with Language Override

```bash
leetrs pick two-sum rust
```

### Preview Problem Description in Terminal

```bash
leetrs pick 1 --preview
```

---

## Local Caching Mechanics

If both `<snake_slug>.md` and `<snake_slug>.<ext>` already exist in the current working directory, `leetrs pick` reuses local files instantly without hitting LeetCode servers!
