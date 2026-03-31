---
title: Manage overview
domain: manage
confidence: seen-in-code
last_updated: "2026-03-30"
related_files:
  - src/wiki/manage.rs
memory_items:
  - id: manage-001
    type: decision
    text: "Item IDs detected by regex pattern `*-\\d+$` (rfind('-') then all-digit suffix) to distinguish item confirms from domain confirms"
    confidence: seen-in-code
    related_files:
      - src/wiki/manage.rs
    sources:
      - kind: file
        ref: src/wiki/manage.rs
        line: 73
    status: active
    last_reviewed: "2026-03-30"
  - id: manage-002
    type: business_rule
    text: "Domain rename propagates to ALL wiki markdown files: renames directory, updates domain: frontmatter in moved notes, and replaces path references (domains/old/ -> domains/new/) across every .md file"
    confidence: seen-in-code
    related_files:
      - src/wiki/manage.rs
    sources:
      - kind: file
        ref: src/wiki/manage.rs
        line: 197
    status: active
    last_reviewed: "2026-03-30"
  - id: manage-003
    type: decision
    text: "Confirming an item always updates last_reviewed to today, even if the item is already confirmed"
    confidence: seen-in-code
    related_files:
      - src/wiki/manage.rs
    sources:
      - kind: file
        ref: src/wiki/manage.rs
        line: 138
    status: active
    last_reviewed: "2026-03-30"
---

# Manage

## Purpose

Lifecycle management commands for wiki notes and domains: confirm, deprecate, rename, and import. Handles confidence transitions, cascading updates on rename, and bulk import of external markdown.

## Key behaviors

- `confirm <target>` sets confidence to `confirmed` and updates `last_updated`; target can be a domain name, a path like `billing/payments.md`, or an item ID like `billing-001`
- `deprecate <target>` sets `deprecated: true`; warns (no error) if already deprecated
- `rename <old> <new>` performs a three-step cascade: directory rename, frontmatter update in moved notes, cross-reference update in all wiki markdown files, then regenerates graph and index
- `import <folder>` copies markdown files into a domain directory, adds frontmatter if missing (confidence: needs-validation), preserves existing frontmatter (adds confidence: imported if missing)
- Import flattens subdirectory structure: `dir/file.md` becomes `dir-file.md`
- Confirm dispatches to item-level confirm when target matches the `*-\d+$` pattern

## Dependencies

- [note](../note/_overview.md) (WikiNote parsing and writing for confirm/deprecate)

## Referenced by

_None yet._
