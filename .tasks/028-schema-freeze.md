# 028 — Gel du schema memory_items MVP

## Statut : a faire

## Horizon : apres remodularisation

## Contexte

memory_items est la primitive centrale. Tout converge dessus : context, check-diff, promote, validate.
Il faut le geler pour permettre migrations et integrations stables.

## Ce qui doit etre fait

### 1. Documenter le schema dans .wiki/_templates/memory-item-schema.md
- Champs obligatoires vs optionnels
- Valeurs valides par champ
- Invariants de serialisation

### 2. Tests de stabilite
- test_memory_item_roundtrip_yaml (serialize -> deserialize -> equal)
- test_memory_item_backward_compat (ancien YAML sans nouveaux champs -> parse OK)
- test_memory_item_forward_compat (nouveau YAML avec champs inconnus -> parse OK, ignore)
- test_memory_item_schema_version

### 3. Convention de migration
- Nouvelles notes : memory_items obligatoires
- Anciennes notes : fallback temporaire
- validate avertit si note non migree
