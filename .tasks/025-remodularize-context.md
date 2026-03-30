# 025 — Remodulariser context

## Statut : a faire

## Horizon : immediat

## Contexte

context.rs fait 992 lignes. Resolution, priorisation, fallback, 2 renderers et hook dans un seul fichier.

## Ce qui doit etre fait

Transformer `src/wiki/context.rs` en `src/wiki/context/` :

- `mod.rs` — types publics (ContextJsonOutput, ContextJsonItem) + fn run(), run_from_stdin()
- `resolve.rs` — resolve_context, resolve_context_json
- `prioritize.rs` — prioritize_memory_items, type_priority, confidence_priority, has_related_file
- `fallback.rs` — compact_summary_fallback
- `render.rs` — compact_summary, compact_summary_v1, truncate_output, extract_sections, extract_bullet_points
- `hook.rs` — HookInput, HookOutput, run_from_stdin logic

## Contraintes

- Zero changement d'API publique
- Tous les tests existants passent sans modification
