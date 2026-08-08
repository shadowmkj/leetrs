---
id: neovim-workflow
title: Neovim Side-by-Side Integration
sidebar_position: 2
---

# ⚡ Neovim Side-by-Side Integration

One of the defining features of `leetrs` is its seamless terminal integration with **Neovim**.

---

## 📐 How Vertical Splitting Works

When you pick a problem via `leetrs pick` or by hitting `Enter` in the TUI:

1. `leetrs` creates two files in your current working directory:
   - `<snake_slug>.md` (Problem description)
   - `<snake_slug>.<ext>` (Code stub)
2. `leetrs` executes Neovim using the following command:

```bash
nvim <snake_slug>.md -c "vsplit <snake_slug>.<ext>"
```

3. Neovim launches in your active terminal, opening the Markdown description in the left vertical pane and your code template in the right vertical pane.

---

## 📑 Metadata Headers

`leetrs` injects a standard metadata header into the top of every generated code template:

### Rust (`two_sum.rs`)

```rust
// id=1 slug=two-sum lang=rust
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
    }
}
```

### Python 3 (`two_sum.py`)

```python
# id=1 slug=two-sum lang=python3
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        pass
```

### SQL (`two_sum.sql`)

```sql
-- id=1 slug=two-sum lang=postgresql
SELECT * FROM Users;
```

:::important Preserving Metadata Headers
Do not remove or alter the header line (`// id=... slug=... lang=...`). `leetrs test` and `leetrs submit` rely on this comment header to identify which LeetCode problem and language to submit to!
:::
