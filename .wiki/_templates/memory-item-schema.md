---
title: Memory Item Schema v1
confidence: confirmed
last_updated: "2026-03-30"
---

# Memory Item Schema v1

Canonical reference for the `memory_items` array stored in wiki note frontmatter.

## YAML Structure

```yaml
memory_items:
  - id: <domain>-<NNN>       # required, unique within domain
    type: <type>              # required
    text: <string>            # required, human-readable description
    confidence: <confidence>  # required
    related_files:            # optional, defaults to []
      - <path>
    sources:                  # optional, defaults to []
      - kind: <string>        # "file", "test", "comment"
        ref: <path>
        line: <uint>          # optional
    status: <status>          # optional, defaults to "active"
    last_reviewed: <date>     # optional, YYYY-MM-DD format
```

## Field Reference

### `id` (required)
- Format: `<domain>-<NNN>` (e.g. `billing-001`, `auth-002`)
- Must be unique within a domain
- Sequential numbering within domain scope

### `type` (required)
| Value           | Meaning                                      |
|-----------------|----------------------------------------------|
| `exception`     | Special case, legacy behavior, workaround    |
| `decision`      | Deliberate architectural or business choice  |
| `business_rule` | Domain logic not obvious from code alone     |

Serialization: `snake_case`

### `text` (required)
- Human-readable description of the memory item
- Max recommended length: 120 characters

### `confidence` (required)
| Value              | Meaning                                    |
|--------------------|--------------------------------------------|
| `confirmed`        | Validated by a human, trust as truth       |
| `verified`         | Cross-checked against code, trust as truth |
| `seen-in-code`     | Observed in source, likely correct         |
| `inferred`         | Deduced by tooling, may need validation    |
| `needs-validation` | Uncertain, must be verified before relying |

Serialization: `kebab-case`. Default: `inferred`.

### `related_files` (optional)
- List of source file paths relative to project root
- Empty list omitted from YAML output

### `sources` (optional)
Provenance entries linking to evidence:

| Field  | Required | Description                     |
|--------|----------|---------------------------------|
| `kind` | yes      | `"file"`, `"test"`, `"comment"` |
| `ref`  | yes      | Path or reference string        |
| `line` | no       | Line number (uint)              |

Empty list omitted from YAML output.

### `status` (optional)
| Value        | Meaning                    |
|--------------|----------------------------|
| `active`     | Currently relevant         |
| `deprecated` | No longer applies          |

Default: `active`. Serialization: `snake_case`.

### `last_reviewed` (optional)
- Format: `YYYY-MM-DD`
- Omitted from YAML when absent

## Compatibility Contract

### Backward compatibility
- Notes without `memory_items` parse successfully (defaults to empty array)
- Items with only required fields (`id`, `type`, `text`, `confidence`) parse successfully

### Forward compatibility
- Unknown fields in `memory_items` entries are silently ignored
- Unknown fields in frontmatter are silently ignored
- No `#[serde(deny_unknown_fields)]` on any schema type

### JSON Output Versioning
- `schema_version: "1"` is included in all JSON output structs (`ContextJsonOutput`, `CheckDiffResult`)
- Consumers should check this field before parsing

## Migration Convention

### Adding a new optional field
1. Add field with `#[serde(default, skip_serializing_if = ...)]`
2. Add roundtrip test covering the new field
3. Bump `schema_version` only if the change affects JSON output structs
4. Existing YAML files continue to work unchanged

### Adding a new required field
1. Never add required fields without a migration path
2. Add as optional first, backfill existing notes, then make required
3. `validate` should warn on notes missing the new field during transition

### Deprecating a field
1. Keep parsing the old field for at least one major version
2. Add a `validate` warning for notes still using the deprecated field
3. Remove only when all notes are migrated
