# Plan d'execution project-wiki

## Contexte

Ce plan traduit les specs (positionnement, execution, roadmap 30/60/90, memory candidates UX, check-diff v0, memory item format) en taches atomiques implementables.

Chaque tache est un fichier independant avec description, criteres de validation, et tests attendus.

## Horizons

### Horizon 30 jours — Prouver la boucle de valeur

| # | Tache | Dependances | Semaine |
|---|-------|-------------|---------|
| 001 | Struct MemoryItem + types | — | S1 |
| 002 | Parsing memory_items depuis front-matter | 001 | S1 |
| 003 | Serialisation memory_items vers YAML | 001 | S1 |
| 004 | Validation des memory_items | 002 | S1 |
| 005 | Context v1 : lecture et priorisation des memory_items | 002 | S2 |
| 006 | Context v1 : fallback notes sans memory_items | 005 | S2 |
| 007 | check-diff : commande CLI | — | S3 |
| 008 | check-diff : collecte des fichiers modifies | 007 | S3 |
| 009 | check-diff : resolution domaine + agregation memoire | 002, 008 | S3 |
| 010 | check-diff : calcul de sensibilite | 009 | S3 |
| 011 | check-diff : sortie texte et JSON | 009, 010 | S3 |
| 012 | Extend confirm pour items individuels | 002 | S4 |
| 013 | README repositionne | — | S1 (parallele) |

### Horizon 60 jours — Cold start et maintenance

| # | Tache | Dependances | Semaine |
|---|-------|-------------|---------|
| 014 | Heuristiques de generation de candidates | 002 | S5 |
| 015 | Generation du fichier _candidates.md | 014 | S5 |
| 016 | Commande promote | 002, 015 | S6 |
| 017 | Hardening scan TypeScript | — | S6 |
| 018 | Refactor init (moins intrusif, opt-in) | — | S7 |
| 019 | Hardening validate | 004 | S7 |

### Horizon 90 jours — Review et echelle

| # | Tache | Dependances | Semaine |
|---|-------|-------------|---------|
| 020 | Sortie JSON stable pour context + check-diff | 005, 011 | S9 |
| 021 | Integration review / CI | 011, 020 | S10 |
| 022 | Drift diff-aware (evolution detect-drift) | 009 | S10 |
| 023 | Strategie conflits .wiki/ en equipe | — | S11 |

## Chemin critique

```
001 -> 002 -> 005 -> 006
              |
              +-> 004
              |
              +-> 009 -> 010 -> 011 -> 020 -> 021
                                         |
008 -------->+                           +-> 022
007 -> 008
```

## Regles d'execution

1. Ne jamais marquer une tache comme terminee sans que tous les criteres de validation soient verts
2. Chaque tache produit des tests avant ou pendant l'implementation
3. Si une tache revele un probleme de design, STOP et re-planifier
4. Commits atomiques : un commit par tache (sauf si la tache est trop grosse, auquel cas la decouper)
5. Apres chaque tache, verifier que les tests existants passent toujours (`cargo test`)
