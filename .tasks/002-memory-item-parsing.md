# 002 — Parsing memory_items depuis front-matter

## Statut : termine (implemente avec 001)

## Horizon : 30 jours — Semaine 1

## Dependances : 001

## Contexte

Le codebase utilise `gray_matter` pour extraire le front-matter YAML des notes, puis `serde_yml` pour le deserialiser en `WikiNote`. Il faut que le parsing supporte le nouveau champ `memory_items` sans casser les notes existantes.

## Ce qui doit etre implemente

### 1. Deserialisation du front-matter enrichi

Le front-matter cible :

```yaml
---
domain: billing
confidence: verified
last_updated: "2026-03-29"
related_files:
  - src/billing/invoice.ts
deprecated: false
memory_items:
  - id: billing-001
    type: exception
    text: Le client X utilise encore l'ancien calcul
    confidence: confirmed
    related_files:
      - src/billing/legacy_pricing.ts
    sources:
      - kind: file
        ref: src/billing/legacy_pricing.ts
      - kind: test
        ref: tests/billing/legacy_pricing.test.ts
    status: active
    last_reviewed: "2026-03-29"
---
```

### 2. Compatibilite arriere

- Les notes sans `memory_items` doivent continuer a se parser normalement
- Le champ `memory_items` doit avoir `#[serde(default)]` pour defaulter a `Vec::new()`
- Aucune note existante ne doit casser

### 3. Gestion des erreurs

- Si un `memory_item` a un type inconnu -> erreur explicite
- Si un `memory_item` a une confidence inconnue -> erreur explicite
- Si un `memory_item` n'a pas d'id -> erreur explicite
- Si un `memory_item` n'a pas de text -> erreur explicite

### 4. Fonction de chargement

Adapter la fonction existante qui parse les notes (probablement `WikiNote::from_file` ou equivalent) pour supporter les memory_items.

## Fichiers a modifier

- `src/wiki/note.rs` — adaptation du parsing existant
- Tout fichier qui construit un `WikiNote` a partir de front-matter

## Criteres de validation

### CV-1 : Note existante sans memory_items
- Parser une note au format actuel (sans champ memory_items)
- Resultat : `memory_items` est un Vec vide
- Aucune erreur

### CV-2 : Note avec memory_items valides
- Parser une note avec 2 memory_items complets
- Chaque item a son id, type, text, confidence, related_files, sources, status
- Tous les champs sont correctement mappes

### CV-3 : Note avec memory_items partiels
- Un item sans `last_reviewed` -> parse OK, champ None
- Un item sans `related_files` -> parse OK, Vec vide
- Un item sans `sources` -> parse OK, Vec vide

### CV-4 : Erreur sur type inconnu
- Un item avec `type: unknown_type` -> erreur claire, pas panic

### CV-5 : Erreur sur confidence inconnue
- Un item avec `confidence: maybe` -> erreur claire

### CV-6 : Erreur sur item sans id
- Un item sans champ `id` -> erreur claire

### CV-7 : Regression sur le parsing existant
- Toutes les notes generees par le scan actuel doivent continuer a se parser
- Lancer les tests existants de `note.rs` -> tous verts

### CV-8 : Contenu markdown preserve
- Le contenu apres le front-matter n'est pas affecte par les memory_items
- Le parsing split front-matter / content fonctionne toujours

## Tests a ecrire

```
test_parse_existing_note_without_memory_items
test_parse_note_with_full_memory_items
test_parse_note_with_partial_memory_items
test_parse_note_memory_item_without_last_reviewed
test_parse_note_memory_item_without_related_files
test_parse_note_memory_item_without_sources
test_parse_error_unknown_type
test_parse_error_unknown_confidence
test_parse_error_missing_id
test_parse_error_missing_text
test_parse_preserves_markdown_content
test_regression_all_existing_note_formats
```

## Risques

- `gray_matter` pourrait avoir des limites avec des structures YAML imbriquees complexes. A tester tot.
- `serde_yml` doit gerer correctement les nested structs avec des renames (`type` -> `type_`, `ref` -> `ref_`).
- Si le parsing echoue, l'erreur doit etre claire pour l'utilisateur (quel fichier, quel champ).
