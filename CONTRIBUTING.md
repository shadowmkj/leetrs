# Contributing

Contributions of all kinds are welcome and appreciated!

## Ways to Contribute

- **Report bugs** — open an issue describing what went wrong and how to reproduce it
- **Suggest features** — open an issue describing the use case and expected behavior
- **Improve documentation** — fix typos, clarify wording, or add examples
- **Submit code** — bug fixes, new features, or performance improvements

## Opening an Issue

- Search existing issues first to avoid duplicates
- Use the appropriate issue template (bug report or feature request)
- Be as specific as possible — include versions, OS, and reproduction steps for bugs

## Making a Contribution

1. **Fork and clone** the repository:
   ```bash
   git clone https://github.com/<your-username>/leetrs.git
   cd leetrs
   ```
2. **Create a new branch**:
   ```bash
   git checkout -b feat/your-feature-name
   ```
3. **Make your changes** and add appropriate unit / integration tests.
4. **Format the code**:
   ```bash
   cargo xtask fmt
   # or: cargo fmt --all
   ```
5. **Run the CI quality gate**:
   `leetrs` provides a unified task runner (`xtask`) to run all checks locally before pushing:
   ```bash
   cargo xtask ci
   ```
   This executes:
   - 🔍 **Format check**: `cargo fmt --all --check`
   - 🔍 **Clippy linter**: `cargo clippy --all-targets -- -D warnings`
   - 🧪 **Unit tests**: `cargo test`

   You can also run individual tasks as needed:
   - `cargo xtask fmt` (or `cargo xtask fmt --check`)
   - `cargo xtask clippy` — Run Clippy with `-D warnings`
   - `cargo xtask test` — Run the unit test suite
   - `cargo xtask coverage --html` — Generate and view code coverage report (requires `cargo install cargo-llvm-cov`)
6. **Commit your changes**:
   Commit messages must follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) (e.g., `feat(tui): add topic search`, `fix(client): handle rate limits`).
   If your PR resolves an issue, reference it in the commit body / PR description (e.g., `Closes #14`).
7. **Push to your fork**:
   ```bash
   git push origin feat/your-feature-name
   ```
8. **Open a Pull Request** against the `main` branch. Ensure the checklist is completed and all automated CI checks pass.

