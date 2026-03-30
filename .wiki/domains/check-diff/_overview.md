---
title: Check-diff overview
domain: check-diff
confidence: seen-in-code
last_updated: "2026-03-30"
related_files:
  - src/wiki/check_diff/mod.rs
  - src/wiki/check_diff/resolve.rs
  - src/wiki/check_diff/sensitivity.rs
  - src/wiki/check_diff/prioritize.rs
memory_items:
  - id: check-diff-001
    type: decision
    text: "Maximum 3 domains shown in output, sorted by file count then memory_items count"
    confidence: seen-in-code
    related_files:
      - src/wiki/check_diff/resolve.rs
    sources:
      - kind: file
        ref: src/wiki/check_diff/resolve.rs
        line: 93
    status: active
    last_reviewed: "2026-03-30"
  - id: check-diff-002
    type: business_rule
    text: "Sensitivity is high if any exception/decision item or stale/low_confidence warning exists, medium if items exist, low otherwise"
    confidence: seen-in-code
    related_files:
      - src/wiki/check_diff/sensitivity.rs
    sources:
      - kind: file
        ref: src/wiki/check_diff/sensitivity.rs
    status: active
    last_reviewed: "2026-03-30"
  - id: check-diff-003
    type: decision
    text: "Item prioritization order: exception > decision > business_rule, then confidence rank, then related files match"
    confidence: seen-in-code
    related_files:
      - src/wiki/check_diff/prioritize.rs
      - src/wiki/prioritize.rs
    sources:
      - kind: file
        ref: src/wiki/check_diff/prioritize.rs
      - kind: file
        ref: src/wiki/prioritize.rs
    status: active
    last_reviewed: "2026-03-30"
  - id: check-diff-004
    type: exception
    text: "Deprecated memory items are filtered out before prioritization and never appear in output"
    confidence: seen-in-code
    related_files:
      - src/wiki/check_diff/prioritize.rs
    sources:
      - kind: file
        ref: src/wiki/check_diff/prioritize.rs
        line: 13
    status: active
    last_reviewed: "2026-03-30"
---

# Check-diff

## Purpose

Analyzes a set of modified files (from git diff or explicit list) and resolves them to wiki domains. Produces a sensitivity assessment with relevant memory items and suggested actions for the developer.

## Key behaviors

- Files are resolved to domains via the file index (`file_index::load_or_rebuild`)
- Domains are ranked by number of matching files, then by memory_items count on tie
- Only the top 3 domains are shown; extra domains are reported as "+N other domain(s) not shown"
- The first domain is tagged as `primary`, the rest as `secondary`
- Sensitivity calculation drives whether suggestions and PR comments are generated
- Maximum 3 suggested actions are generated, driven by stale notes and exception/decision items

## Dependencies

- [context](../context/_overview.md) (shared prioritization logic via `src/wiki/prioritize.rs`)
- [validate](../validate/_overview.md) (shared note parsing via `src/wiki/note.rs`)

## Referenced by

_None yet._
