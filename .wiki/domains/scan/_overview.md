---
title: Scan overview
domain: scan
confidence: seen-in-code
last_updated: "2026-03-30"
related_files:
  - src/init/scan/mod.rs
  - src/init/scan/structure.rs
  - src/init/scan/dependencies.rs
  - src/init/scan/details.rs
  - src/init/scan/imports.rs
  - src/init/scan/generate.rs
memory_items:
  - id: scan-001
    type: decision
    text: "Three-pass architecture: Pass 1 discovers project structure and domain candidates, Pass 2 analyzes cross-domain dependencies via import extraction, Pass 3 extracts models/routes/comments details"
    confidence: seen-in-code
    related_files:
      - src/init/scan/mod.rs
    sources:
      - kind: file
        ref: src/init/scan/mod.rs
        line: 48
    status: active
    last_reviewed: "2026-03-30"
  - id: scan-002
    type: decision
    text: "Auto-generated notes from scan set confidence to 'inferred' (visible in domain_overview template used by generate.rs)"
    confidence: seen-in-code
    related_files:
      - src/init/scan/generate.rs
    sources:
      - kind: file
        ref: src/wiki/add.rs
        line: 375
    status: active
    last_reviewed: "2026-03-30"
  - id: scan-003
    type: business_rule
    text: "Domain detection uses 17+ recognized parent directory names (services, modules, features, app, lib, packages, controllers, routes, models, api, components, handlers, domains, core, plugins, apps, pages, middleware, providers)"
    confidence: seen-in-code
    related_files:
      - src/init/scan/structure.rs
    sources:
      - kind: file
        ref: src/init/scan/structure.rs
        line: 26
    status: active
    last_reviewed: "2026-03-30"
---

# Scan

## Purpose

Scans a codebase to discover domains, analyze dependencies, and extract structural details. This is the entry point of `project-wiki init` and produces the initial wiki content from an existing project.

## Key behaviors

- Pass 1 (structure): walks the filesystem respecting .gitignore, skips extra directories (.wiki, node_modules, target, etc.), assigns files to domains based on parent directory patterns
- Domain names are extracted by finding a recognized parent dir (e.g. `services/`) and taking the next path component as the domain name
- Next.js route groups like `(dashboard)` are skipped when extracting domain names
- Singular/plural domain duplicates are merged (e.g. "user" + "users" -> "users", "entity" + "entities" -> "entities")
- Loose files matching an existing domain name are merged into that domain if they sit under a recognized parent dir
- Domain name normalization strips common suffixes (.controller, .service, .model, -handler, _route, etc.)
- Pass 2 (dependencies): extracts imports from source files and builds a cross-domain dependency graph
- Pass 3 (details): extracts models, routes, TODO comments, and test files per domain
- If no domain candidates are found, the scan completes gracefully with an empty result

## Dependencies

_None (root module of the init workflow)._

## Referenced by

- [candidates](../candidates/_overview.md) (runs after scan to generate memory item candidates)
