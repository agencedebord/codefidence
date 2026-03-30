# 020 — Sortie JSON stable pour context + check-diff

## Statut : a faire

## Horizon : 90 jours — Semaine 9

## Dependances : 005, 011

## Contexte

Les sorties JSON de `context` et `check-diff` doivent etre stables et documentees pour permettre l'integration CI, les scripts, et les outils tiers. Cette tache formalise le contrat JSON.

## Ce qui doit etre implemente

### 1. Schema JSON documente

- Definir un schema JSON pour `context --json`
- Definir un schema JSON pour `check-diff --json`
- Les documenter dans `.wiki/_templates/` ou un fichier dedie

### 2. Versionning du schema

- Ajouter un champ `"schema_version": "1"` dans chaque sortie JSON
- Permettre les evolutions futures sans casser les consommateurs

### 3. Garanties de stabilite

- L'ordre des champs est deterministe
- Les champs ne sont jamais retires dans une meme version majeure
- Les nouveaux champs sont additifs

### 4. context --json

```json
{
  "schema_version": "1",
  "domain": "billing",
  "confidence": "verified",
  "last_updated": "2026-03-29",
  "memory_items": [...],
  "secondary_domains": [...],
  "warnings": [...],
  "fallback_mode": false
}
```

### 5. check-diff --json

Schema deja defini dans la tache 011. Ajouter `schema_version`.

## Fichiers a modifier

- `src/wiki/context.rs` — flag --json et sortie
- `src/wiki/check_diff.rs` — ajout schema_version
- `src/cli.rs` — flag --json sur context

## Criteres de validation

### CV-1 : context --json parseable
- Sortie JSON valide et conforme au schema

### CV-2 : check-diff --json avec version
- Champ schema_version present

### CV-3 : Determinisme
- Deux appels identiques -> meme JSON (meme ordre de champs)

### CV-4 : Compatibilite ascendante
- Ajouter un champ -> les anciens parsers ne cassent pas

## Tests a ecrire

```
test_context_json_schema_version
test_context_json_valid
test_check_diff_json_schema_version
test_json_deterministic_output
test_json_backward_compatible
```
