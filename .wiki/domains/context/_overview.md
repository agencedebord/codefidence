---
title: Context overview
domain: context
confidence: seen-in-code
last_updated: "2026-03-30"
related_files:
  - src/wiki/context/mod.rs
  - src/wiki/context/resolve.rs
  - src/wiki/context/prioritize.rs
  - src/wiki/context/render.rs
memory_items:
  - id: context-001
    type: decision
    text: "MAX_MEMORY_ITEMS = 3: only the top 3 prioritized items are included in context output"
    confidence: seen-in-code
    related_files:
      - src/wiki/context/mod.rs
    sources:
      - kind: file
        ref: src/wiki/context/mod.rs
        line: 22
    status: active
    last_reviewed: "2026-03-30"
  - id: context-002
    type: decision
    text: "MAX_CONTEXT_LEN = 2000 chars: compact summary is truncated with '[... truncated]' suffix"
    confidence: seen-in-code
    related_files:
      - src/wiki/context/mod.rs
      - src/wiki/context/render.rs
    sources:
      - kind: file
        ref: src/wiki/context/mod.rs
        line: 19
      - kind: file
        ref: src/wiki/context/render.rs
        line: 164
    status: active
    last_reviewed: "2026-03-30"
  - id: context-003
    type: business_rule
    text: "Fallback mode activates when note has no memory_items: extracts Key behaviors, Business rules, and Dependencies from markdown sections"
    confidence: seen-in-code
    related_files:
      - src/wiki/context/resolve.rs
      - src/wiki/context/render.rs
    sources:
      - kind: file
        ref: src/wiki/context/resolve.rs
        line: 78
      - kind: file
        ref: src/wiki/context/render.rs
        line: 14
    status: active
    last_reviewed: "2026-03-30"
  - id: context-004
    type: business_rule
    text: "Prioritization order shared with check-diff: exception > decision > business_rule, then confidence, then related file match"
    confidence: seen-in-code
    related_files:
      - src/wiki/context/prioritize.rs
      - src/wiki/prioritize.rs
    sources:
      - kind: file
        ref: src/wiki/context/prioritize.rs
      - kind: file
        ref: src/wiki/prioritize.rs
    status: active
    last_reviewed: "2026-03-30"
  - id: context-005
    type: exception
    text: "Low-confidence items (inferred/needs-validation) generate a WARNING line in both text and JSON output"
    confidence: seen-in-code
    related_files:
      - src/wiki/context/render.rs
      - src/wiki/context/resolve.rs
    sources:
      - kind: file
        ref: src/wiki/context/render.rs
        line: 79
      - kind: file
        ref: src/wiki/context/resolve.rs
        line: 92
    status: active
    last_reviewed: "2026-03-30"
---

# Context

## Purpose

Resolves a source file path to its wiki domain and produces a compact context summary for LLM injection. Supports both structured (memory_items v1) and fallback (markdown extraction) output modes.

## Key behaviors

- File is resolved to a domain via the file index
- Domain's `_overview.md` is parsed for frontmatter and memory items
- In v1 mode: top 3 memory items are selected by priority, with a "(+N more items)" indicator if truncated
- In fallback mode: Key behaviors, Business rules, and Dependencies sections are extracted from markdown body
- JSON output includes `fallback_mode: true/false` to signal which path was taken
- Output is truncated to 2000 characters to fit LLM context windows
- A hook mode (`run_from_stdin`) reads file paths from stdin for git hook integration

## Dependencies

- [check-diff](../check-diff/_overview.md) (shared prioritization logic via `src/wiki/prioritize.rs`)
- [validate](../validate/_overview.md) (shared note parsing via `src/wiki/note.rs`)

## Referenced by

_None yet._
