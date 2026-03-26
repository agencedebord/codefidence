# project-wiki

**Auto-managed project knowledge wiki for AI-assisted development.**

[![CI](https://github.com/agencedebord/project-wiki/actions/workflows/ci.yml/badge.svg)](https://github.com/agencedebord/project-wiki/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-blue)](https://www.rust-lang.org)

## The Problem

Business knowledge is scattered across Notion pages, code comments, Slack threads, and people's heads. When AI assistants work on your codebase, they lack the context that makes the difference between a correct fix and a subtly wrong one. `project-wiki` creates a `.wiki/` folder that lives in your repo, auto-populated from codebase scanning, with confidence levels on every piece of information so both humans and LLMs know what to trust.

## Quick Start

```bash
cargo install project-wiki
cd your-project
project-wiki init
project-wiki status
project-wiki consult --all
```

## Features

- **14 CLI commands** for managing project knowledge
- **3-pass codebase scanner** -- structure discovery, dependency analysis, detail extraction (JS/TS, Python, Rust, Go)
- **Confidence levels** on every note, from `confirmed` down to `needs-validation`
- **Optional Notion import** via `--features notion`
- **Claude Code integration** -- auto-patches `.claude/CLAUDE.md` with wiki instructions
- **Machine-readable `_index.json`** for LLM consumption
- **Beautiful terminal UI** with progress bars and color output
- **npm distribution** support (coming soon)
- **164 tests** including property-based tests with proptest

## Commands

| Command | Description |
|---------|-------------|
| `init` | Initialize a `.wiki/` folder with automatic codebase scan |
| `status` | Show wiki health summary -- note count, staleness, coverage |
| `validate` | Check wiki notes for consistency and broken references |
| `consult` | Display wiki notes for a domain or all domains |
| `graph` | Display the inter-domain dependency graph |
| `search` | Full-text search across all wiki notes |
| `add domain` | Create a new domain |
| `add context` | Add context to an existing domain |
| `add decision` | Record a business decision |
| `rebuild` | Regenerate the dependency graph and index from scratch |
| `index` | Regenerate `_index.json` and `_index.md` |
| `confirm` | Set a note's confidence level to `confirmed` |
| `deprecate` | Mark a domain or note as deprecated |
| `rename-domain` | Rename a domain and update all cross-references |
| `import` | Import external markdown files into the wiki |
| `vectors` | Manage vector embeddings for semantic search (future) |

## Confidence System

Every wiki note carries a confidence level in its YAML front matter:

| Level | Meaning |
|-------|---------|
| `confirmed` | Validated by a human -- this is ground truth |
| `verified` | Cross-referenced with multiple sources |
| `seen-in-code` | Extracted directly from source code patterns |
| `inferred` | Deduced from project structure (default for new notes) |
| `needs-validation` | Uncertain or potentially outdated -- review needed |

Use `project-wiki confirm <target>` to promote a note to `confirmed` after review.

## Configuration

Wiki settings live in `.wiki/config.toml`:

```toml
# Number of days before a note is considered stale
staleness_days = 30

# Automatically regenerate _index.json after mutations
auto_index = true
```

## Claude Code Integration

Running `project-wiki init` automatically patches your `.claude/CLAUDE.md` with instructions that tell Claude Code to:

1. Read `.wiki/_index.md` before each non-trivial task
2. Consult domain notes related to the work
3. Update wiki notes after modifying behavior
4. Use a `wiki:` commit prefix for wiki changes

This ensures AI assistants always have access to your project's business context.

## Installation

### From crates.io

```bash
cargo install project-wiki
```

### With Notion import support

```bash
cargo install project-wiki --features notion
```

### From source

```bash
git clone https://github.com/agencedebord/project-wiki.git
cd project-wiki
cargo build --release
# Binary is at target/release/project-wiki
```

### npm (coming soon)

```bash
npx project-wiki
```

## Building from Source

Requirements:
- Rust 1.85.0 or later
- Cargo (included with Rust)

```bash
git clone https://github.com/agencedebord/project-wiki.git
cd project-wiki
cargo build --release
cargo test
```

The release profile is optimized for binary size (`opt-level = "z"`, LTO, single codegen unit, symbol stripping).

## Contributing

Contributions are welcome. Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.
