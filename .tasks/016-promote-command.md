# 016 — Commande promote

## Statut : a faire

## Horizon : 60 jours — Semaine 6

## Dependances : 002, 015

## Contexte

Le workflow de validation des candidates est trop manuel si l'utilisateur doit copier-coller du markdown entre fichiers. La commande `promote` automatise le passage d'une candidate vers un memory_item confirme dans la note cible.

## Ce qui doit etre implemente

### 1. Commande CLI

```bash
project-wiki promote <candidate-id> [--confidence <level>] [--text "reformulation"]
```

- `candidate-id` : id de la candidate (ex: `billing-001`)
- `--confidence` : niveau de confiance cible (default: `confirmed`)
- `--text` : reformulation optionnelle du texte

### 2. Workflow

1. Lire `_candidates.md` (ou `._candidates.json` si shadow file)
2. Trouver la candidate par id
3. Verifier que le status est `pending`
4. Creer un `MemoryItem` a partir de la candidate
5. L'ajouter dans la note cible (champ `memory_items` du front-matter)
6. Attribuer un id definitif si different de l'id candidate
7. Mettre `confidence` = confirmed (ou la valeur passee en option)
8. Mettre `last_reviewed` = date du jour
9. Marquer la candidate comme `confirmed` dans `_candidates.md`
10. Message de confirmation

### 3. Sortie

```text
✓ Promoted billing-001 to .wiki/domains/billing/_overview.md
  [business_rule] La facture est emise apres synchro [confirmed]
```

### 4. Reject

Ajouter aussi :
```bash
project-wiki reject <candidate-id>
```

- Marque la candidate comme `rejected` dans `_candidates.md`
- Ne modifie aucune note

### 5. Erreurs

- Candidate non trouvee -> erreur
- Candidate deja traitee (confirmed/rejected) -> warning
- Note cible introuvable -> erreur avec suggestion de creer le domaine d'abord
- Conflit d'id (id deja utilise dans la note cible) -> generer un nouvel id automatiquement

## Fichiers a creer/modifier

- Nouveau : `src/wiki/promote.rs`
- `src/cli.rs` — commandes promote et reject
- `src/wiki/mod.rs` — export

## Criteres de validation

### CV-1 : Promote basique
- Candidate pending billing-001
- `promote billing-001`
- L'item apparait dans la note cible avec confidence confirmed
- La candidate est marquee confirmed dans _candidates.md

### CV-2 : Promote avec reformulation
- `promote billing-001 --text "La facture est toujours emise apres synchro complete"`
- L'item a le nouveau texte

### CV-3 : Promote avec confidence custom
- `promote billing-001 --confidence seen-in-code`
- L'item a confidence = seen-in-code

### CV-4 : Reject
- `reject billing-001`
- Candidate marquee rejected
- Aucune note modifiee

### CV-5 : Candidate non trouvee
- `promote billing-999`
- Erreur claire

### CV-6 : Candidate deja traitee
- `promote billing-001` sur une candidate deja confirmed
- Warning, pas de duplication

### CV-7 : Conflit d'id
- Id billing-001 deja utilise dans la note
- Nouvel id genere (billing-003 par ex)
- Message indiquant le nouvel id

### CV-8 : Note cible mise a jour correctement
- Apres promote, la note est re-parseable
- Le markdown content est preserve
- Les autres items sont preserves

### CV-9 : last_reviewed auto-set
- Apres promote, last_reviewed = date du jour

## Tests a ecrire

```
test_promote_basic
test_promote_with_text_override
test_promote_with_confidence_override
test_promote_updates_candidate_status
test_promote_adds_item_to_note
test_promote_preserves_existing_items
test_promote_preserves_markdown_content
test_promote_sets_last_reviewed
test_reject_basic
test_reject_does_not_modify_note
test_promote_candidate_not_found
test_promote_candidate_already_processed
test_promote_id_conflict_auto_resolve
test_promote_note_target_not_found
```

## Risques

- Le parsing/ecriture de `_candidates.md` doit etre robuste. Le shadow JSON est recommande pour eviter les bugs de parsing markdown.
- L'auto-generation d'id en cas de conflit doit etre deterministe (prendre le max existant + 1).
