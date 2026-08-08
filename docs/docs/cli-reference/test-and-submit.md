---
id: test-and-submit
title: test & submit
sidebar_position: 5
---

# 🧪 `leetrs test` & `leetrs submit`

Commands for executing code against sample test cases and submitting solutions to LeetCode judging servers.

---

## 🧪 `leetrs test`

Executes your local solution file against sample test cases without recording an official submission on LeetCode.

### Usage

```bash
leetrs test <FILE>
```

### Arguments

| Argument | Type | Description |
|---|---|---|
| `<FILE>` | `path` | Path to the local solution file (e.g. `two_sum.rs`, `two_sum.py`) |

### Workflow

1. Parses the metadata header comment inside `<FILE>` to extract problem ID, slug, and language.
2. Sends a `test-run` payload to LeetCode's judging API.
3. Polls judging status asynchronously until complete.
4. Renders colored test results: passed test cases, runtime, output vs expected output, and compiler errors.

### Example

```bash
leetrs test two_sum.rs
```

---

## 🚀 `leetrs submit`

Submits your local solution file to LeetCode for official judging and profile score updating.

### Usage

```bash
leetrs submit <FILE>
```

### Arguments

| Argument | Type | Description |
|---|---|---|
| `<FILE>` | `path` | Path to the local solution file (e.g. `two_sum.rs`, `two_sum.py`) |

### Output Metrics

Upon completion, `leetrs submit` displays:
- **Judge Status**: `Accepted`, `Wrong Answer`, `Time Limit Exceeded`, `Compile Error`, or `Runtime Error`.
- **Test cases passed**: e.g., `57 / 57 testcases passed`.
- **Runtime Performance**: e.g., `0 ms` (Beats `100.00%` of Rust submissions).
- **Memory Usage**: e.g., `2.1 MB` (Beats `98.50%` of Rust submissions).

### Example

```bash
leetrs submit two_sum.rs
```
