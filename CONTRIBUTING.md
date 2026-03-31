# Contributing to codefidence

Thank you for considering contributing to codefidence!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/codefidence.git`
3. Create a branch: `git checkout -b my-feature`
4. Make your changes
5. Run the checks: `cargo fmt --check && cargo clippy && cargo test`
6. Commit and push
7. Open a pull request

## Development Setup

Requirements:
- Rust 1.85.0+ (edition 2024)
- Cargo

```bash
# Build
cargo build

# Run tests (default features)
cargo test

# Run tests (with Notion support)
cargo test --features notion

# Run clippy
cargo clippy -- -D warnings
cargo clippy --features notion -- -D warnings

# Format
cargo fmt
```

## Code Style

- Follow standard Rust conventions (`cargo fmt` enforces formatting)
- Use `anyhow::Result` for error handling
- Add tests for new functionality (unit tests in the module, integration tests in `tests/`)
- UI output goes to stderr, data output goes to stdout
- Use `LazyLock<Regex>` for compiled regexes (never compile in loops)

## Commit Messages

- Use conventional commit style: `feat:`, `fix:`, `refactor:`, `test:`, `docs:`, `chore:`
- Wiki-related changes use the `wiki:` prefix
- One commit per logical change

## Pull Requests

- Keep PRs focused on a single concern
- Include tests for new features or bug fixes
- Update CHANGELOG.md under an `[Unreleased]` section
- Make sure CI passes (build, test, clippy, fmt)

## Adding a New Command

1. Add the variant to `Commands` in `src/cli.rs`
2. Create the implementation in the appropriate module (`src/wiki/` or `src/init/`)
3. Wire it up in the `match` block in `cli::run()`
4. Add tests
5. Update README.md

## Adding a New Feature Flag

Feature-gated integrations (like `notion`) follow this pattern:

1. Add the feature in `Cargo.toml` under `[features]` with optional deps
2. Gate modules with `#[cfg(feature = "your_feature")]`
3. Provide a clear error message when the feature is not enabled
4. Test both with and without the feature flag

## Reporting Issues

- Use the [bug report template](.github/ISSUE_TEMPLATE/bug_report.md) for bugs
- Use the [feature request template](.github/ISSUE_TEMPLATE/feature_request.md) for ideas
- Include your OS, Rust version, and codefidence version

## License

By contributing, you agree that your contributions will be licensed under the same dual license as the project: MIT OR Apache-2.0.
