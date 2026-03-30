# 014 — Heuristiques de generation de candidates

## Statut : termine

## Horizon : 60 jours — Semaine 5

## Dependances : 002

## Contexte

Le cold start a besoin de proposer 3 a 5 memory candidates utiles sans que l'utilisateur ecrive quoi que ce soit. Le scan actuel extrait des structures (modeles, routes, imports, TODOs), mais ne produit pas de propositions de memoire typees. Cette tache definit les heuristiques simples qui transforment les signaux du scan en candidates.

## Principes

- Regles binaires, pas de scoring opaque
- Tracables et debuggables
- Peu de candidates (3-5 max au total)
- Provenance toujours explicite
- Preferer s'abstenir que bruiter

## Ce qui doit etre implemente

### 1. Heuristiques par type

#### business_rule candidates

Signal : un fichier a des tests ET contient un pattern significatif :
- Fonction avec un nom explicite (ex: `calculateDiscount`, `validateImport`, `processRefund`)
- Condition metier non triviale detectee dans le code + couverte par test
- Section TODO/HACK/NOTE mentionnant un comportement voulu

Regle : si un fichier non trivial (>50 lignes) a un test associe ET un commentaire TODO/HACK/NOTE -> proposer une candidate business_rule.

#### exception candidates

Signal : nommage suggestif dans le code ou les chemins :
- Fichiers ou fonctions contenant : `legacy`, `compat`, `override`, `workaround`, `migration`, `deprecated`, `old`, `v1`
- Commentaire mentionnant un client specifique ou un cas particulier

Regle : si un fichier contient un pattern de nommage d'exception ET est non trivial -> proposer une candidate exception.

#### decision candidates

Signal : commentaires deliberes :
- Commentaires contenant : `decision`, `chosen`, `we decided`, `deliberately`, `intentionally`, `on purpose`, `trade-off`, `tradeoff`
- ADR references dans le code

Regle : si un commentaire contient un pattern de decision -> proposer une candidate decision.

### 2. Scoring de priorisation

Si plus de 5 candidates generees, garder les 5 meilleures selon :
1. Exception > decision > business_rule
2. Nombre de signaux combines (test + code + commentaire > code seul)
3. Taille du fichier source (fichier plus gros = potentiellement plus critique)

### 3. Deduplication

- Pas de doublon sur le meme fichier + meme type
- Si deux candidates sont sur le meme fichier avec des types differents, garder les deux

### 4. Exclusion

Ne pas generer si :
- Le texte propose est trop generique ("This module handles billing")
- La provenance est un seul fichier sans test ni commentaire
- Le fichier est un utilitaire pur (helpers, utils, constants)
- Le fichier est dans un repertoire de tests uniquement

## Fichiers a creer/modifier

- Nouveau : `src/init/candidates.rs` — logique de generation
- Modifier : `src/init/mod.rs` — integration dans le flow init

## Criteres de validation

### CV-1 : business_rule detectee
- Fichier avec test + TODO mentionnant un comportement
- Candidate business_rule generee avec provenance correcte

### CV-2 : exception detectee
- Fichier nomme `legacy_pricing.ts`
- Candidate exception generee

### CV-3 : decision detectee
- Commentaire "We decided to not deduplicate"
- Candidate decision generee

### CV-4 : Maximum 5 candidates
- Scan avec 10 signaux potentiels
- Exactement 5 candidates en sortie

### CV-5 : Exclusion generique
- Fichier `utils/helpers.ts` avec un TODO generique
- Pas de candidate generee

### CV-6 : Deduplication
- Meme fichier, deux patterns business_rule
- Une seule candidate

### CV-7 : Provenance toujours presente
- Chaque candidate a au moins une source dans `provenance`

### CV-8 : Texte propose non generique
- Aucune candidate avec un texte qui resume juste la structure ("This is the billing module")

## Tests a ecrire

```
test_heuristic_business_rule_test_plus_todo
test_heuristic_exception_legacy_naming
test_heuristic_exception_compat_naming
test_heuristic_decision_comment_pattern
test_heuristic_max_5_candidates
test_heuristic_exclusion_utils
test_heuristic_exclusion_test_only
test_heuristic_dedup_same_file_same_type
test_heuristic_keep_same_file_different_type
test_heuristic_priority_exception_first
test_heuristic_provenance_always_present
test_heuristic_empty_scan_no_candidates
```

## Risques

- Les heuristiques seront imparfaites. C'est attendu. Le but est de les iterer apres observation du taux d'acceptation.
- Les patterns de nommage sont biaises vers l'anglais. Acceptable au MVP.
