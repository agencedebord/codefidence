---
title: Add overview
domain: add
confidence: seen-in-code
last_updated: "2026-03-30"
related_files:
  - src/wiki/add.rs
memory_items:
  - id: add-001
    type: business_rule
    text: "Domain name normalization: lowercase, replace spaces/underscores with hyphens, strip path separators and double dots for traversal protection"
    confidence: seen-in-code
    related_files:
      - src/wiki/add.rs
    sources:
      - kind: file
        ref: src/wiki/add.rs
        line: 18
    status: active
    last_reviewed: "2026-03-30"
  - id: add-002
    type: decision
    text: "Decision filenames use `{YYYY-MM-DD}-{slug}.md` format, slug is generated via slugify() and truncated to 50 chars"
    confidence: seen-in-code
    related_files:
      - src/wiki/add.rs
    sources:
      - kind: file
        ref: src/wiki/add.rs
        line: 184
    status: active
    last_reviewed: "2026-03-30"
  - id: add-003
    type: business_rule
    text: "Context text appended to 'Key behaviors' section first (priority), falls back to 'Business rules' section, or end of file if neither exists"
    confidence: seen-in-code
    related_files:
      - src/wiki/add.rs
    sources:
      - kind: file
        ref: src/wiki/add.rs
        line: 247
    status: active
    last_reviewed: "2026-03-30"
---

# Add

## Purpose

Provides commands to add new domains, context, and decisions to the wiki. Handles input normalization, template rendering, and file creation.

## Key behaviors

- `add domain <name>` creates a directory under `.wiki/domains/` with an `_overview.md` from template
- `add context <text>` appends a `[confirmed]` bullet to an existing domain note; auto-guesses domain from text if `--domain` is not specified
- `add decision <text>` creates a dated markdown file in `.wiki/decisions/` using a template
- Domain name validation rejects empty names, `.`, and names containing path separators after normalization
- Domain guessing fails explicitly if zero or multiple domains match the text (no silent default)
- All three commands regenerate the wiki index after mutation

## Dependencies

- [note](../note/_overview.md) (domain overview template structure)

## Referenced by

_None yet._
