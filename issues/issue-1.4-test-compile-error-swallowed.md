# [BUG] Compilation Errors Silently Discarded in `leetrs test`

**Labels:** `bug`, `critical`, `ux`, `diagnostics`  
**Components:** [`src/format.rs`](file:///Users/milan/Documents/Desk/Apps/leetrs/src/format.rs), [`src/services/submission.rs`](file:///Users/milan/Documents/Desk/Apps/leetrs/src/services/submission.rs), [`src/models/submission.rs`](file:///Users/milan/Documents/Desk/Apps/leetrs/src/models/submission.rs)

---

## 1. Summary

When testing a solution file using `leetrs test`, compilation errors (syntax errors, type mismatches, missing imports, etc.) are completely discarded. The CLI prints `❌ Test Failed` with zero details about what caused the compilation failure. Developers are left with no compiler messages, line numbers, or hints.

---

## 2. Root Cause Analysis

The failure information is dropped across three distinct layers:

### Layer 1: Deserialization Model omits compile error fields
In [`src/models/submission.rs:36-54`](file:///Users/milan/Documents/Desk/Apps/leetrs/src/models/submission.rs#L36-L54), `TestSubmissionCheckResult` defines fields for runtime errors (`full_runtime_error`), but completely omits `compile_error` and `full_compile_error`:

```rust
// src/models/submission.rs:36-54
pub struct TestSubmissionCheckResult {
    pub code_answer: Option<Vec<String>>,
    pub code_output: Option<Vec<String>>,
    pub correct_answer: Option<bool>,
    pub expected_code_answer: Option<Vec<String>>,
    pub full_runtime_error: Option<String>,
    // Missing: compile_error, full_compile_error!
    ...
}
```

### Layer 2: Result conversion hardcodes `compile_error: None`
In [`src/services/submission.rs:73-98`](file:///Users/milan/Documents/Desk/Apps/leetrs/src/services/submission.rs#L73-L98):
```rust
pub fn from_test_check(r: TestSubmissionCheckResult) -> Self {
    ...
    Self {
        ...
        compile_error: None, // Hardcoded to None!
        full_runtime_error: r.full_runtime_error,
        ...
    }
}
```

### Layer 3: Output formatter ignores `SubmissionStatus::CompileError` on test runs
In [`src/format.rs:26-57`](file:///Users/milan/Documents/Desk/Apps/leetrs/src/format.rs#L26-L57):
```rust
if result.is_test {
    ...
    match &result.status {
        SubmissionStatus::Accepted if passed => { ... }
        SubmissionStatus::Accepted => { ... }
        SubmissionStatus::RuntimeError => {
            if let Some(err) = &result.full_runtime_error {
                println!("❌ Error\n{}", err);
            }
        }
        _ => {} // CompileError falls through here and prints NOTHING!
    }
}
```

---

## 3. Steps to Reproduce

1. Create a syntax error in any problem solution:
   ```rust
   // two_sum.rs
   impl Solution {
       pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
           invalid syntax error here !@#$
       }
   }
   ```
2. Run test:
   ```bash
   leetrs test two_sum.rs
   ```
3. Output:
   ```
   ==================================================
     ❌ Test Failed
   ==================================================
   ```
   Nothing else is displayed.

---

## 4. Expected Behavior

When a compilation error occurs during testing, `leetrs test` should display:
```
==================================================
  ❌ Compile Error
==================================================

💥 Compiler Output:
<compiler error diagnostic, line numbers, and messages>
```

---

## 5. Proposed Solution

1. In [`src/models/submission.rs`](file:///Users/milan/Documents/Desk/Apps/leetrs/src/models/submission.rs):
   Add `compile_error: Option<String>` and `full_compile_error: Option<String>` to `TestSubmissionCheckResult`.
2. In [`src/services/submission.rs`](file:///Users/milan/Documents/Desk/Apps/leetrs/src/services/submission.rs):
   Map `compile_error: r.compile_error.or(r.full_compile_error)` in `from_test_check`.
3. In [`src/format.rs`](file:///Users/milan/Documents/Desk/Apps/leetrs/src/format.rs):
   Add a match branch for `SubmissionStatus::CompileError` under `if result.is_test`:
   ```rust
   SubmissionStatus::CompileError => {
       if let Some(err) = &result.compile_error {
           println!("💥 Compiler Output:\n{}", err);
       }
   }
   ```

---

## 6. Verification & Test Plan

- **Unit Test:** Test `SubmissionResult::from_test_check` deserializing a check result with `compile_error` / `full_compile_error`.
- **Unit Test:** Test `format_result` with `is_test: true` and `status: SubmissionStatus::CompileError`, confirming compiler error is printed.
