---
title: Promote overview
domain: promote
confidence: seen-in-code
last_updated: "2026-03-30"
related_files:
  - src/wiki/promote.rs
memory_items:
  - id: promote-001
    type: decision
    text: "ID conflict resolution auto-increments with `{prefix}-{num:03}` format (e.g. billing-002)"
    confidence: seen-in-code
    related_files:
      - src/wiki/promote.rs
    sources:
      - kind: file
        ref: src/wiki/promote.rs
        line: 308
    status: active
    last_reviewed: "2026-03-30"
  - id: promote-002
    type: decision
    text: "Default confidence on promote is Confirmed when no override is provided"
    confidence: seen-in-code
    related_files:
      - src/wiki/promote.rs
    sources:
      - kind: file
        ref: src/wiki/promote.rs
        line: 85
    status: active
    last_reviewed: "2026-03-30"
  - id: promote-003
    type: business_rule
    text: "Only candidates with status 'pending' can be promoted; already-processed candidates trigger a warning and return early"
    confidence: seen-in-code
    related_files:
      - src/wiki/promote.rs
    sources:
      - kind: file
        ref: src/wiki/promote.rs
        line: 51
    status: active
    last_reviewed: "2026-03-30"
  - id: promote-004
    type: exception
    text: "Provenance entries must be indented with '  - ' (two spaces + dash) to distinguish from top-level metadata during parsing"
    confidence: seen-in-code
    related_files:
      - src/wiki/promote.rs
    sources:
      - kind: file
        ref: src/wiki/promote.rs
        line: 228
    status: active
    last_reviewed: "2026-03-30"
---

# Promote

## Purpose

Promotes pending candidates from `_candidates.md` into memory items on target domain notes, or rejects them. This is the human-in-the-loop step where auto-generated candidate insights become confirmed wiki knowledge.

## Key behaviors

- Parses `_candidates.md` to extract candidate metadata (id, status, type, text, target, provenance)
- Resolves the target note path and adds a new `MemoryItem` to its front matter
- Transfers provenance entries from the candidate into `sources` on the new memory item
- Supports confidence override (`--confidence`) and text override (`--text`) at promote time
- On ID conflict, auto-increments the numeric suffix (e.g. `billing-001` becomes `billing-002`)
- Updates the candidate status in `_candidates.md` to `confirmed` or `rejected`
- Reject leaves the target note untouched

## Dependencies

- [note](../note/_overview.md) (WikiNote, MemoryItem, Confidence types)
- [candidates](../candidates/_overview.md) (generates the `_candidates.md` file consumed here)

## Referenced by

- [candidates](../candidates/_overview.md) (promote is the downstream consumer of generated candidates)
