---
title: Drift overview
domain: drift
confidence: seen-in-code
last_updated: "2026-03-30"
related_files:
  - src/wiki/drift.rs
memory_items:
  - id: drift-001
    type: business_rule
    text: "Hunk proximity tolerance is +/-5 lines (HUNK_PROXIMITY_TOLERANCE = 5)"
    confidence: seen-in-code
    related_files:
      - src/wiki/drift.rs
    sources:
      - kind: file
        ref: src/wiki/drift.rs
        line: 37
    status: active
    last_reviewed: "2026-03-30"
  - id: drift-002
    type: business_rule
    text: "Identifier minimum length is 4 chars to avoid false positives from short tokens"
    confidence: seen-in-code
    related_files:
      - src/wiki/drift.rs
    sources:
      - kind: file
        ref: src/wiki/drift.rs
        line: 299
    status: active
    last_reviewed: "2026-03-30"
  - id: drift-003
    type: exception
    text: "52 stopwords excluded from identifier matching (language keywords like let/var/const and generic terms like data/item/list)"
    confidence: seen-in-code
    related_files:
      - src/wiki/drift.rs
    sources:
      - kind: file
        ref: src/wiki/drift.rs
        line: 277
    status: active
    last_reviewed: "2026-03-30"
  - id: drift-004
    type: decision
    text: "Low-confidence notes (Inferred or NeedsValidation) trigger extra drift warnings advising verification"
    confidence: seen-in-code
    related_files:
      - src/wiki/drift.rs
    sources:
      - kind: file
        ref: src/wiki/drift.rs
        line: 160
    status: active
    last_reviewed: "2026-03-30"
---

# Drift

## Purpose

Detects wiki drift by analyzing file modifications against wiki domain notes. Runs as a CLI command or as a hook (reading JSON from stdin) to warn developers when their changes may invalidate documented knowledge.

## Key behaviors

- Resolves a modified file to its wiki domain via the file index
- Checks four drift conditions: stale note (exceeds `staleness_days`), low confidence, related file modified, and memory item impacted by diff hunks
- Diff-aware analysis: parses unified diff hunk headers, checks source line proximity within tolerance, and performs identifier heuristic matching
- Identifier heuristic extracts tokens from added/removed diff lines, filters stopwords and short tokens, then matches against memory item text
- Deprecated memory items are skipped during diff-aware checks
- Hook mode reads `tool_input.file_path` from stdin JSON and outputs `additionalContext` JSON to stdout

## Dependencies

- [note](../note/_overview.md) (WikiNote, Confidence, MemoryItemStatus types)
- [context](../context/_overview.md) (shared file_index resolution)
- [check-diff](../check-diff/_overview.md) (complementary: check-diff analyzes multiple files, drift analyzes one)

## Referenced by

- [check-diff](../check-diff/_overview.md) (uses similar drift detection concepts at a higher level)
