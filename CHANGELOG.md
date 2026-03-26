# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-03-26

### Added

- 14 CLI commands: `init`, `status`, `validate`, `consult`, `graph`, `search`, `add domain`, `add context`, `add decision`, `rebuild`, `index`, `confirm`, `deprecate`, `rename-domain`, `import`
- 3-pass codebase scanner (structure, relations, details) supporting JS/TS, Python, Rust, Go
- Confidence system with 5 levels: `confirmed`, `verified`, `seen-in-code`, `inferred`, `needs-validation`
- Optional Notion import via `--features notion` with batch pagination, resume support, and contradiction detection
- Claude Code integration: auto-patches `.claude/CLAUDE.md` with wiki instructions
- Machine-readable `_index.json` for LLM consumption
- Configurable staleness thresholds via `.wiki/config.toml`
- `validate` command checking broken links, dead references, staleness, orphan notes, deprecated references
- Mermaid dependency graph generation
- Full-text search with Unicode-safe highlighting
- Path traversal sanitization on domain names
- Domain rename with automatic cross-reference updates
- External markdown import with front matter handling
- Beautiful terminal UI with progress bars and color output
- Property-based tests with proptest
- Dual license: MIT OR Apache-2.0

[0.1.0]: https://github.com/agencedebord/project-wiki/releases/tag/v0.1.0
