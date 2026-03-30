# 004 — Validation des memory_items

## Statut : a faire

## Horizon : 30 jours — Semaine 1

## Dependances : 002

## Contexte

La commande `validate` existe deja et verifie la coherence des notes. Elle doit etre etendue pour valider les memory_items : unicite des ids, types autorises, confidences valides, structure des sources, coherence des dates.

## Ce qui doit etre implemente

### 1. Validation de l'unicite des ids

- Au sein d'une note : pas de doublon d'id
- Au sein du wiki entier : pas de doublon d'id entre notes differentes
- Format d'id attendu : `{domain}-{number}` (warning si non respecte, pas erreur bloquante au MVP)

### 2. Validation des types

- Seuls `decision`, `business_rule`, `exception` sont autorises au MVP
- Tout autre type -> erreur avec message clair (fichier + id de l'item)

### 3. Validation des confidences

- Seuls les 5 niveaux existants sont autorises
- Coherence : un item `confirmed` dans une note `needs-validation` -> warning

### 4. Validation des sources

- Chaque source doit avoir un `kind` et un `ref`
- `kind` doit etre parmi : `file`, `test`, `comment`, `ticket`, `note`
- `ref` ne doit pas etre vide

### 5. Validation des dates

- `last_reviewed` si present doit etre un format date valide
- `last_reviewed` ne doit pas etre dans le futur (warning)

### 6. Validation de coherence

- Un item `deprecated` ne devrait pas etre le seul item d'un domaine (warning)
- Un item sans sources -> warning (pas erreur au MVP)
- Un item avec `related_files` vide -> OK (pas bloquant)

### 7. Integration dans la commande validate existante

- Les erreurs memory_items s'ajoutent aux erreurs existantes
- Le format de sortie reste coherent
- Les nouvelles validations ne cassent pas le comportement actuel

## Fichiers a modifier

- `src/wiki/validate.rs` — ajout des regles de validation
- Eventuellement `src/wiki/note.rs` si des helpers sont necessaires

## Criteres de validation

### CV-1 : Doublon d'id dans une note
- Deux items avec le meme id dans une note -> erreur

### CV-2 : Doublon d'id entre notes
- Meme id dans deux notes differentes -> erreur

### CV-3 : Type inconnu
- Item avec `type: custom_type` -> erreur avec message "[fichier] item billing-001: type inconnu 'custom_type'"

### CV-4 : Confidence inconnue sur item
- Item avec `confidence: maybe` -> erreur

### CV-5 : Source sans kind
- Source avec `ref` mais sans `kind` -> erreur

### CV-6 : Source sans ref
- Source avec `kind` mais `ref` vide -> erreur

### CV-7 : Kind inconnu
- Source avec `kind: slack` -> warning (pas erreur, extensible)

### CV-8 : Item confirmed dans note needs-validation
- -> warning "item billing-001 est confirmed mais la note est needs-validation"

### CV-9 : Date dans le futur
- `last_reviewed: "2030-01-01"` -> warning

### CV-10 : Notes existantes sans memory_items
- La validation passe toujours sans erreur sur les notes actuelles
- Aucune regression

### CV-11 : Sortie coherente
- Les erreurs memory_items apparaissent dans le meme format que les erreurs existantes
- Le code de retour est non-zero si erreurs

## Tests a ecrire

```
test_validate_duplicate_id_same_note
test_validate_duplicate_id_across_notes
test_validate_unknown_type
test_validate_unknown_confidence
test_validate_source_missing_kind
test_validate_source_empty_ref
test_validate_unknown_source_kind_warning
test_validate_confidence_inconsistency_warning
test_validate_future_date_warning
test_validate_deprecated_only_item_warning
test_validate_item_without_sources_warning
test_validate_notes_without_items_pass
test_validate_existing_notes_regression
test_validate_mixed_notes_with_and_without_items
test_validate_output_format
```

## Risques

- La validation cross-note (doublon d'id) necessite de charger toutes les notes. Verifier que c'est compatible avec le flow existant de `validate`.
- Distinguer erreurs (bloquantes) et warnings (informatifs) dans la sortie.
