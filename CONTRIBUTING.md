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

## Making a contribution

1. Fork the repository
2. Create a new branch [e.g `git checkout -b feat/your-feature-name`]
3. Write code
4. Use `cargo fmt --all` to format the code
5. Run `cargo clippy --all --release` and fix any warnings
6. Commit your chages (the commit messages should follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)),
   also if your commit targets a specific issue you should reference that in the
   description
7. Push to your fork
8. Create a pull request
