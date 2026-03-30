# 006 — Context v1 : fallback notes sans memory_items

## Statut : a faire

## Horizon : 30 jours — Semaine 2

## Dependances : 005

## Contexte

Pendant la migration, de nombreuses notes n'auront pas de `memory_items`. Le `context` doit rester fonctionnel sur ces notes en utilisant l'ancien comportement (extraction des sections markdown). Ce fallback assure la compatibilite arriere.

## Ce qui doit etre implemente

### 1. Detection du mode

- Si `note.memory_items` est non vide -> utiliser la logique v1 (tache 005)
- Si `note.memory_items` est vide -> utiliser le fallback (logique actuelle)

### 2. Fallback : logique actuelle preservee

Reprendre le comportement existant :
- Extraire sections : Key behaviors, Business rules, Dependencies
- Filtrer les placeholders ("_None detected._")
- Limiter a 5 key behaviors, 5 business rules, 10 dependencies
- Formater en resume compact

### 3. Marqueur de mode

La sortie du fallback doit indiquer qu'elle utilise l'ancien format :
```
[project-wiki] Domain: billing (confidence: inferred, updated: 2026-03-29)
ℹ No structured memory items — showing section summary

Key behaviors: Generates invoices — Handles refunds
Business rules: No dedup on import
Dependencies: payments, taxes
```

### 4. Suggestion de migration

Si le fallback est utilise sur un domaine avec du contenu riche (>3 business rules), ajouter un hint :
```
💡 Consider adding memory_items to this note for better context injection
```

## Fichiers a modifier

- `src/wiki/context.rs` — branchement fallback

## Criteres de validation

### CV-1 : Note avec memory_items -> pas de fallback
- Note avec 2 memory_items
- La sortie utilise le format v1, pas le fallback

### CV-2 : Note sans memory_items -> fallback
- Note au format actuel
- La sortie est identique au comportement actuel + marqueur

### CV-3 : Regression zero
- Les tests existants de context.rs passent tous
- La sortie sur notes existantes est fonctionnellement equivalente

### CV-4 : Suggestion de migration
- Note sans memory_items avec 4 business rules
- La sortie contient le hint de migration

### CV-5 : Pas de suggestion sur note pauvre
- Note sans memory_items avec 1 seul business rule
- Pas de hint de migration

### CV-6 : Mode hook compatible
- Le fallback fonctionne aussi en mode hook (JSON)

## Tests a ecrire

```
test_fallback_triggered_when_no_memory_items
test_no_fallback_when_memory_items_present
test_fallback_output_matches_existing_behavior
test_fallback_includes_mode_marker
test_fallback_migration_hint_rich_note
test_fallback_no_hint_sparse_note
test_fallback_hook_mode_json
test_regression_existing_context_tests
```

## Risques

- Le branchement fallback ne doit pas dupliquer le code. Extraire la logique existante dans une fonction dediee si ce n'est pas deja le cas.
