# Domain dependency graph

> Auto-generated from domain notes. Do not edit manually.
> Last regenerated: 2026-03-30

```mermaid
graph LR
    context
    check_diff["check-diff"]
    context -->|(shared prioritization logic via `src/wiki/prioritize.rs`)| check_diff
    validate
    context -->|(shared note parsing via `src/wiki/note.rs`)| validate
    check_diff -->|(shared prioritization logic via `src/wiki/prioritize.rs`)| context
    check_diff -->|(shared note parsing via `src/wiki/note.rs`)| validate
    validate -->|(shared note model via `src/wiki/note.rs`)| check_diff
    validate -->|(shared note model via `src/wiki/note.rs`)| context

    style context fill:#e74c3c,color:#fff
    style check_diff fill:#e74c3c,color:#fff
    style validate fill:#e74c3c,color:#fff
```
