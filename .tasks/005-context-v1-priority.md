# 005 — Context v1 : lecture et priorisation des memory_items

## Statut : a faire

## Horizon : 30 jours — Semaine 2

## Dependances : 002

## Contexte

`context` est le coeur produit existant. Il injecte le contexte wiki dans la fenetre de Claude avant edition. Aujourd'hui il renvoie un resume compact (max 2000 chars) avec key behaviors, business rules, dependencies. Il doit evoluer pour lire les `memory_items` structures et les prioriser par type.

## Etat actuel

Le `context` actuel :
1. Resout fichier -> domaine via file_index
2. Lit la note `_overview.md` du domaine
3. Extrait les sections markdown : Key behaviors, Business rules, Dependencies
4. Formate un resume compact avec limite 2000 chars
5. Inclut un warning si confidence faible

## Ce qui doit etre implemente

### 1. Lecture des memory_items

Quand une note a des `memory_items`, les utiliser comme source primaire au lieu des sections markdown.

### 2. Priorisation par type

Ordre de priorite (spec check-diff v0 et plan d'execution) :
1. `exception` — les derogations sont les plus dangereuses a ignorer
2. `decision` — les choix explicites sont critiques
3. `business_rule` — les regles metier completes le contexte

### 3. Priorisation secondaire

A type egal :
1. Confiance forte (`confirmed`, `verified`) d'abord
2. Relations explicites fichier -> item (via `related_files` de l'item)
3. Recence (`last_reviewed` plus recent d'abord)

### 4. Limite d'items

- Maximum 3 items de memoire dans la sortie
- Si plus de 3, les 3 les plus prioritaires
- Mentionner "+N autres items" si tronque

### 5. Format de sortie enrichi

```
[project-wiki] Domain: billing (confidence: verified, updated: 2026-03-29)

Memory:
  [exception] Le client X utilise encore l'ancien calcul [confirmed]
  [decision] Pas de deduplication des lignes importees [verified]
  [business_rule] La facture n'est emise qu'apres synchro complete [seen-in-code]

Dependencies: payments, taxes
Related files: src/billing/invoice.ts, src/billing/service.ts

⚠ 1 item has low confidence
```

### 6. Mode hook

La sortie hook (JSON `additionalContext`) doit inclure les memory_items priorises dans le meme format texte.

### 7. Domaines secondaires

Si le fichier est lie a un domaine secondaire (via dependencies), mentionner les items critiques (exceptions uniquement) des domaines secondaires. Maximum 1 domaine secondaire, maximum 1 item.

## Fichiers a modifier

- `src/wiki/context.rs` — logique de priorisation et formatage

## Criteres de validation

### CV-1 : Priorisation par type
- Note avec 1 business_rule, 1 decision, 1 exception
- Sortie : exception en premier, decision en second, business_rule en troisieme

### CV-2 : Priorisation secondaire par confiance
- 2 decisions : une confirmed, une inferred
- La confirmed apparait en premier

### CV-3 : Priorisation secondaire par relation fichier
- 2 decisions confirmed : une avec related_file matchant le fichier requete, une sans
- Celle avec related_file apparait en premier

### CV-4 : Limite a 3 items
- Note avec 5 items
- Sortie contient exactement 3 items + mention "+2 autres items"

### CV-5 : Format de sortie
- La sortie contient le type entre crochets
- La sortie contient la confiance entre crochets
- Le texte est lisible en moins de 20 secondes

### CV-6 : Warning low confidence
- Au moins un item `inferred` ou `needs-validation`
- La sortie inclut un warning

### CV-7 : Mode hook JSON
- La sortie JSON contient `additionalContext` avec le texte formate
- Le JSON est parseable

### CV-8 : Domaine secondaire
- Fichier lie a billing, billing depend de payments
- payments a une exception
- La sortie mentionne l'exception de payments

### CV-9 : Note sans memory_items
- Doit etre geree par le fallback (tache 006), pas par cette logique
- Si memory_items est vide, cette branche ne s'active pas

### CV-10 : Sortie courte
- La sortie totale ne depasse pas 2000 chars
- Si les 3 items + metadata depassent, tronquer les textes d'items

## Tests a ecrire

```
test_context_prioritize_exception_first
test_context_prioritize_decision_over_business_rule
test_context_secondary_sort_by_confidence
test_context_secondary_sort_by_related_file
test_context_secondary_sort_by_recency
test_context_limit_3_items
test_context_limit_shows_remaining_count
test_context_format_type_brackets
test_context_format_confidence_brackets
test_context_warning_low_confidence
test_context_no_warning_all_confirmed
test_context_hook_json_output
test_context_secondary_domain_exception
test_context_no_secondary_domain_if_no_exception
test_context_output_under_2000_chars
test_context_truncates_long_item_text
```

## Risques

- Le changement de format de sortie peut affecter les hooks Claude existants. Verifier que les hooks tolerent un format enrichi.
- La resolution de domaine secondaire peut etre couteuse si beaucoup de domaines. Limiter a 1 lookup.
