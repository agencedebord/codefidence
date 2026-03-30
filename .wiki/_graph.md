# Domain dependency graph

> Auto-generated from domain notes. Do not edit manually.
> Last regenerated: 2026-03-30

```mermaid
graph LR
    validate
    check_diff["check-diff"]
    validate -->|(shared note model via `src/wiki/note.rs`)| check_diff
    context
    validate -->|(shared note model via `src/wiki/note.rs`)| context
    context -->|(shared prioritization logic via `src/wiki/prioritize.rs`)| check_diff
    context -->|(shared note parsing via `src/wiki/note.rs`)| validate
    check_diff -->|(shared prioritization logic via `src/wiki/prioritize.rs`)| context
    check_diff -->|(shared note parsing via `src/wiki/note.rs`)| validate

    style check_diff fill:#e74c3c,color:#fff
    style context fill:#e74c3c,color:#fff
    style validate fill:#e74c3c,color:#fff
```
