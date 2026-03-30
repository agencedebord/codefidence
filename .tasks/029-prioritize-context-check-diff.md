# 029 — Deduplication priorisation context / check-diff

## Statut : a faire

## Horizon : apres remodularisation

## Contexte

context.rs et check_diff.rs ont chacun leur propre logique de priorisation des memory items.
Les deux implementent type_priority, confidence_priority, has_related_file separement.
Apres remodularisation, extraire un module partage.

## Ce qui doit etre fait

- Creer `src/wiki/prioritize.rs` avec la logique partagee
- context/prioritize.rs et check_diff/prioritize.rs importent depuis le module partage
- Supprimer la duplication
