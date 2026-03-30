# 024 — Remodulariser check_diff

## Statut : a faire

## Horizon : immediat

## Contexte

check_diff.rs fait 1692 lignes avec 9 responsabilites dans un seul fichier.
C'est le module le plus gros et le plus a risque d'empilement.

## Ce qui doit etre fait

Transformer `src/wiki/check_diff.rs` en `src/wiki/check_diff/` :

- `mod.rs` — types publics (Sensitivity, DomainHit, CheckDiffResult...) + fn run()
- `collect.rs` — collect_files, normalize_path, should_ignore
- `resolve.rs` — resolve_domains, DomainAgg
- `prioritize.rs` — prioritize_and_format_items, type_priority, confidence_priority, has_related_file
- `sensitivity.rs` — calculate_sensitivity, generate_suggestions
- `warnings.rs` — build_warnings, format_warning_detail
- `render.rs` — format_text, format_json, format_pr_comment

## Contraintes

- Zero changement d'API publique
- Tous les tests existants passent sans modification
- cargo clippy propre
