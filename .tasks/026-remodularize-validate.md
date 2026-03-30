# 026 — Remodulariser validate

## Statut : a faire

## Horizon : immediat

## Contexte

validate.rs fait 1327 lignes. 10 fonctions de check, orchestration, strict mode, et 27 tests dans un seul fichier.

## Ce qui doit etre fait

Transformer `src/wiki/validate.rs` en `src/wiki/validate/` :

- `mod.rs` — fn run(strict), orchestration, output formatting
- `notes.rs` — check_confidence_ratio, check_staleness
- `memory_items.rs` — check_memory_items
- `links.rs` — collect_all_md_files, check_broken_links, check_dead_references, check_deprecated_references, check_orphan_notes
- `domains.rs` — check_undocumented_domains, check_domain_name_coherence, check_missing_dependencies

## Contraintes

- Zero changement d'API publique
- Tous les tests existants passent sans modification
