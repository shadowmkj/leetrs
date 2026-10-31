---
id: daily
title: daily
sidebar_position: 5
---

# 📅 `leetrs daily`

Fetches today's LeetCode Problem of the Day and opens it the same way [`pick`](./pick.md) does.

---

## Usage

```bash
leetrs daily [OPTIONS] [LANGUAGE]
```

---

## Arguments

| Argument | Type | Required | Description |
|---|---|---|---|
| `[LANGUAGE]` | `string` | No | Language for the code template (`python`, `rust`, `pandas`, `mysql`, `postgres`). Defaults to the `config.toml` setting. |

---

## Options

| Option | Type | Required | Description |
|---|---|---|---|
| `-p`, `--preview` | `flag` | No | Print the problem description Markdown to `stdout` without opening an editor. |

---

## Behaviour

- The active challenge comes from LeetCode's `questionOfToday` GraphQL query. Its date, number, title and difficulty are printed to `stderr` before the problem is picked, so `leetrs daily --preview > problem.md` keeps that line out of the file.
- Files are written and reused exactly as with `pick`.
- If today's problem is for LeetCode Premium subscribers and your account is not premium, `daily` stops with a message instead of failing on the missing content. If premium status can't be confirmed, it warns and tries anyway.

```bash
leetrs daily
leetrs daily rust
leetrs daily --preview
```
