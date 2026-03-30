---
title: Note overview
domain: note
confidence: seen-in-code
last_updated: "2026-03-30"
related_files:
  - src/wiki/note.rs
memory_items:
  - id: note-001
    type: decision
    text: "Default confidence is Inferred (conservative default via #[default] on the enum)"
    confidence: seen-in-code
    related_files:
      - src/wiki/note.rs
    sources:
      - kind: file
        ref: src/wiki/note.rs
        line: 18
    status: active
    last_reviewed: "2026-03-30"
  - id: note-002
    type: decision
    text: "Empty vectors (related_files, sources) and None optionals (last_reviewed, line) are omitted in YAML serialization via skip_serializing_if"
    confidence: seen-in-code
    related_files:
      - src/wiki/note.rs
    sources:
      - kind: file
        ref: src/wiki/note.rs
        line: 94
      - kind: file
        ref: src/wiki/note.rs
        line: 98
      - kind: file
        ref: src/wiki/note.rs
        line: 104
    status: active
    last_reviewed: "2026-03-30"
  - id: note-003
    type: business_rule
    text: "Forward compatibility: unknown YAML fields are silently ignored during deserialization (serde default behavior, verified by tests)"
    confidence: seen-in-code
    related_files:
      - src/wiki/note.rs
    sources:
      - kind: file
        ref: src/wiki/note.rs
        line: 808
    status: active
    last_reviewed: "2026-03-30"
  - id: note-004
    type: business_rule
    text: "MemoryItemStatus has only 2 states: Active (default) and Deprecated"
    confidence: seen-in-code
    related_files:
      - src/wiki/note.rs
    sources:
      - kind: file
        ref: src/wiki/note.rs
        line: 55
    status: active
    last_reviewed: "2026-03-30"
---

# Note

## Purpose

Defines the core data model for wiki notes: `WikiNote`, `FrontMatter`, `MemoryItem`, `Confidence`, `MemoryItemType`, `MemoryItemStatus`, and `MemoryItemSource`. This is the foundational module that all other wiki commands depend on for parsing and writing domain notes.

## Key behaviors

- Parses YAML front matter from markdown files using `gray_matter`
- Supports round-trip write: serializes front matter back to YAML and preserves markdown content
- Domain name is inferred from the parent directory of the note file
- `Confidence` enum uses kebab-case serialization (e.g. `seen-in-code`, `needs-validation`)
- `MemoryItemType` uses snake_case serialization (e.g. `business_rule`)
- `MemoryItemSource.ref_` is serialized as `ref` in YAML (serde rename)
- `MemoryItem.type_` is serialized as `type` in YAML (serde rename)
- Notes without front matter parse successfully with all defaults

## Dependencies

_None (leaf module)._

## Referenced by

- [check-diff](../check-diff/_overview.md) (imports WikiNote, MemoryItem types)
- [context](../context/_overview.md) (imports WikiNote, MemoryItem types)
- [validate](../validate/_overview.md) (imports WikiNote, MemoryItem types)
- [promote](../promote/_overview.md) (imports WikiNote, MemoryItem, Confidence types)
- [drift](../drift/_overview.md) (imports WikiNote, Confidence, MemoryItemStatus types)
- [candidates](../candidates/_overview.md) (candidate types mirror MemoryItemType)
