---
title: Validate overview
domain: validate
confidence: seen-in-code
last_updated: "2026-03-30"
related_files:
  - src/wiki/validate/mod.rs
  - src/wiki/validate/memory_items.rs
  - src/wiki/validate/notes.rs
  - src/wiki/validate/domains.rs
memory_items:
  - id: validate-001
    type: business_rule
    text: "11 validation checks run in order: broken links, undocumented domains, dead refs, deprecated refs, confidence ratio, staleness, orphan notes, domain name coherence, cross-domain deps, memory items, migration status"
    confidence: seen-in-code
    related_files:
      - src/wiki/validate/mod.rs
    sources:
      - kind: file
        ref: src/wiki/validate/mod.rs
    status: active
    last_reviewed: "2026-03-30"
  - id: validate-002
    type: decision
    text: "Strict mode promotes all warnings to errors, causing non-zero exit on any issue"
    confidence: seen-in-code
    related_files:
      - src/wiki/validate/mod.rs
    sources:
      - kind: file
        ref: src/wiki/validate/mod.rs
        line: 205
    status: active
    last_reviewed: "2026-03-30"
  - id: validate-003
    type: business_rule
    text: "Staleness threshold defaults to 30 days, configurable via staleness_days in config.toml"
    confidence: seen-in-code
    related_files:
      - src/wiki/validate/notes.rs
      - src/wiki/config.rs
    sources:
      - kind: file
        ref: src/wiki/config.rs
        line: 13
      - kind: file
        ref: src/wiki/validate/notes.rs
        line: 27
    status: active
    last_reviewed: "2026-03-30"
  - id: validate-004
    type: exception
    text: "Memory items validation is check #10 in the validation pipeline; checks duplicate IDs, source integrity, confidence consistency, and future dates"
    confidence: seen-in-code
    related_files:
      - src/wiki/validate/memory_items.rs
    sources:
      - kind: file
        ref: src/wiki/validate/memory_items.rs
      - kind: file
        ref: src/wiki/validate/mod.rs
        line: 179
    status: active
    last_reviewed: "2026-03-30"
  - id: validate-005
    type: business_rule
    text: "Migration status check (#11) reports notes without memory_items as informational warning, never promoted to error in strict mode"
    confidence: seen-in-code
    related_files:
      - src/wiki/validate/migration_status.rs
      - src/wiki/validate/mod.rs
    sources:
      - kind: file
        ref: src/wiki/validate/migration_status.rs
      - kind: file
        ref: src/wiki/validate/mod.rs
    status: active
    last_reviewed: "2026-03-30"
---

# Validate

## Purpose

Runs a suite of 11 structural integrity checks against the wiki, reporting errors and warnings. Used both locally and in CI to ensure wiki health.

## Key behaviors

- Checks run sequentially and each reports its own errors/warnings independently
- Broken links, dead references, and domain name mismatches are errors (always fail)
- Undocumented domains, deprecated references, staleness, orphan notes, and cross-domain deps are warnings
- Confidence ratio warns if >40% of notes are inferred or needs-validation
- Memory items check validates: duplicate IDs (within and across notes), empty source fields, confidence inconsistency between item and note, future last_reviewed dates, missing sources, and all-deprecated states
- Summary box shows pass/warning/error counts

## Dependencies

- [check-diff](../check-diff/_overview.md) (shared note model via `src/wiki/note.rs`)
- [context](../context/_overview.md) (shared note model via `src/wiki/note.rs`)

## Referenced by

_None yet._
