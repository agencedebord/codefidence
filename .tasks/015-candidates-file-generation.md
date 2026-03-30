# 015 — Generation du fichier _candidates.md

## Statut : termine

## Horizon : 60 jours — Semaine 5

## Dependances : 014

## Contexte

Les heuristiques (tache 014) produisent des candidates en memoire. Cette tache les ecrit dans le fichier `.wiki/_candidates.md` au format defini dans la spec UX memory candidates.

## Ce qui doit etre implemente

### 1. Generation du fichier

Creer `.wiki/_candidates.md` avec :
- En-tete explicatif
- Section par domaine
- Section par candidate avec tous les champs

### 2. Format de sortie

```markdown
# Memory Candidates

> Propositions auto-generees a confirmer, rejeter ou reformuler.
> Ces candidates ne sont pas encore de la memoire confirmee.
> Editez ce fichier ou utilisez `project-wiki promote <id>` pour valider.

## billing

### billing-001

- **status**: pending
- **type**: business_rule
- **confidence**: inferred
- **provenance**:
  - file: src/billing/invoice.ts
  - test: tests/billing/invoice.test.ts
- **rationale**: Regle detectee a partir d'un test et d'un commentaire TODO
- **target**: .wiki/domains/billing/_overview.md

> La facture semble n'etre emise qu'apres synchronisation complete.

**Action** : confirmer | reformuler | rejeter
```

### 3. Integration dans init

Apres le scan et la generation des notes, generer `_candidates.md` si des candidates existent.

### 4. Commande standalone

Ajouter une commande `project-wiki generate-candidates` pour regenerer le fichier sans refaire tout le init.

### 5. Idempotence

- Si `_candidates.md` existe deja, ne pas ecraser les candidates deja traitees (status != pending)
- Ajouter uniquement les nouvelles candidates
- Si aucune nouvelle candidate, ne rien modifier

### 6. Instrumentation

Logger :
- Nombre de candidates generees
- Types des candidates
- Domaines concernes

## Fichiers a creer/modifier

- `src/init/candidates.rs` — ecriture du fichier
- `src/cli.rs` — commande generate-candidates
- `src/init/mod.rs` — integration dans init

## Criteres de validation

### CV-1 : Fichier genere correctement
- 3 candidates generees
- Le fichier contient 3 sections de candidates avec tous les champs

### CV-2 : Format lisible
- Le fichier est du markdown valide
- Chaque candidate est comprehensible en moins de 10 secondes

### CV-3 : Provenance visible
- Chaque candidate a au moins une source dans provenance

### CV-4 : Status initial = pending
- Toutes les nouvelles candidates ont status: pending

### CV-5 : Idempotence
- Lancer generate-candidates deux fois
- Les candidates pending ne sont pas dupliquees
- Les candidates confirmed/rejected ne sont pas ecrasees

### CV-6 : Zero candidates
- Scan sans signal fort
- Pas de fichier genere (ou fichier avec message "Aucune candidate detectee")

### CV-7 : Integration init
- `project-wiki init` sur un repo avec des signaux
- `_candidates.md` est genere automatiquement

## Tests a ecrire

```
test_generate_candidates_file_format
test_generate_candidates_all_fields_present
test_generate_candidates_provenance_visible
test_generate_candidates_status_pending
test_generate_candidates_idempotent
test_generate_candidates_preserves_confirmed
test_generate_candidates_zero_candidates
test_generate_candidates_integrated_in_init
test_generate_candidates_markdown_valid
```

## Risques

- Le parsing de `_candidates.md` pour l'idempotence est fragile. Envisager un fichier JSON shadow (`._candidates.json`) pour la logique machine, et le markdown pour l'humain.
