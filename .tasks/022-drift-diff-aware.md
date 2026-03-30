# 022 — Drift diff-aware (evolution detect-drift)

## Statut : termine

## Horizon : 90 jours — Semaine 10

## Dependances : 009

## Contexte

`detect-drift` actuel ne fait que verifier staleness, confiance, et related_files. L'evolution vise a le rendre conscient du diff reel : quels memory_items sont potentiellement impactes par les lignes modifiees, pas juste les fichiers.

## Ce qui doit etre implemente

### 1. Lecture du diff par fichier

Au lieu de juste `--name-only`, lire `git diff` avec les hunks pour chaque fichier.

### 2. Zones modifiees -> items potentiellement impactes

Si un item a un `sources` avec `line: 42` et que le hunk touche les lignes 38-50, l'item est "potentiellement impacte".

### 3. Heuristiques complementaires

- Si une fonction nommee dans un item est modifiee
- Si un pattern (ex: nom de variable, constante) mentionne dans le texte de l'item apparait dans le diff

### 4. Sortie enrichie

```text
[drift] billing-001 potentially impacted
  [exception] Le client X utilise encore l'ancien calcul
  Reason: source file src/billing/legacy_pricing.ts modified near line 42
  Action: verify if this exception still holds
```

## Fichiers a modifier

- `src/wiki/drift.rs` — evolution de la logique

## Criteres de validation

### CV-1 : Detection par proximite de ligne
- Item source line 42, hunk touche 38-50
- Item signale comme potentiellement impacte

### CV-2 : Pas de faux positif si hunk loin
- Item source line 42, hunk touche 200-210
- Item pas signale

### CV-3 : Fallback sur comportement actuel
- Si pas de line info dans les sources
- Comportement actuel preserve

### CV-4 : Regression zero
- Tests existants de drift passent toujours

## Tests a ecrire

```
test_drift_diff_aware_line_proximity
test_drift_diff_aware_no_false_positive
test_drift_diff_aware_fallback_no_line_info
test_drift_diff_aware_function_name_match
test_regression_existing_drift_tests
```

## Risques

- Le parsing des hunks git est non trivial. Utiliser une librairie ou un parsing minimal.
- Les numeros de ligne dans les sources deviennent stale rapidement. C'est un signal faible, pas une verite.
