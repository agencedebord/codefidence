# 027 — Remodulariser candidates

## Statut : a faire

## Horizon : immediat

## Contexte

candidates.rs fait 1002 lignes. Heuristiques, dedup, scoring, rendering markdown dans un seul fichier.

## Ce qui doit etre fait

Transformer `src/init/candidates.rs` en `src/init/candidates/` :

- `mod.rs` — types publics (Candidate, CandidateType, ProvenanceEntry) + fn generate()
- `heuristics.rs` — detect_exception/decision/business_rule + regexes + helpers (is_excluded_path, is_generic_text, count_file_lines, find_test_for_file, infer_source_for_test, truncate_text)
- `dedupe.rs` — deduplicate, prioritize_and_limit, type_priority, assign_ids
- `render.rs` — format_candidates_markdown, write_candidates_file, parse_processed_ids

## Contraintes

- Zero changement d'API publique
- Tous les tests existants passent sans modification
