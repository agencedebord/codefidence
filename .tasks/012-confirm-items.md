# 012 — Extend confirm pour items individuels

## Statut : a faire

## Horizon : 30 jours — Semaine 4

## Dependances : 002

## Contexte

La commande `confirm` existe et opere sur des notes/domaines entiers. Avec les memory_items structures, il faut pouvoir confirmer un item individuel par son id. Cela permet aussi de mettre a jour `last_reviewed` automatiquement.

## Etat actuel

`confirm` prend un `target` (nom de domaine ou chemin de note) et met la confiance de la note a `confirmed`.

## Ce qui doit etre implemente

### 1. Detection du type de target

- Si le target matche le format `{domain}-{number}` (ex: `billing-001`) -> mode item
- Sinon -> mode note (comportement actuel preserve)

### 2. Mode item

1. Chercher l'item dans toutes les notes du wiki
2. Si trouve : mettre sa `confidence` a `confirmed`
3. Mettre a jour `last_reviewed` a la date du jour
4. Sauvegarder la note modifiee
5. Message de confirmation : "Confirmed billing-001: Le client X utilise encore l'ancien calcul"

### 3. Mode note (preserve)

Comportement identique a l'actuel. Pas de changement.

### 4. Erreurs

- Item id non trouve -> erreur "No memory item found with id 'billing-001'"
- Item deja confirmed -> warning "billing-001 is already confirmed", pas d'erreur

### 5. Auto-update last_reviewed

A chaque `confirm` d'item, `last_reviewed` est mis a la date du jour. Pas besoin de le passer en argument.

## Fichiers a modifier

- `src/wiki/manage.rs` ou le fichier qui gere `confirm` actuellement
- `src/wiki/note.rs` si un helper de recherche d'item par id est necessaire

## Criteres de validation

### CV-1 : Confirm un item existant
- Item billing-001 avec confidence inferred
- `confirm billing-001`
- L'item a maintenant confidence = confirmed
- last_reviewed = date du jour

### CV-2 : Confirm preserve le reste
- Note avec 3 items, on confirme le 2e
- Les items 1 et 3 sont inchanges
- Les metadonnees de la note sont inchangees

### CV-3 : Confirm une note (comportement preserve)
- `confirm billing`
- La note billing a confidence = confirmed
- Les memory_items ne sont PAS modifies individuellement

### CV-4 : Item non trouve
- `confirm billing-999`
- Erreur "No memory item found with id 'billing-999'"
- Code de sortie 1

### CV-5 : Item deja confirmed
- Item deja confirmed
- `confirm billing-001`
- Warning, pas d'erreur, code de sortie 0
- last_reviewed quand meme mis a jour

### CV-6 : Sauvegarde correcte
- Apres confirm, re-parser la note
- L'item a bien sa nouvelle confiance et date

### CV-7 : Detection du mode
- `billing-001` -> mode item
- `billing` -> mode note
- `billing/_overview.md` -> mode note

## Tests a ecrire

```
test_confirm_item_changes_confidence
test_confirm_item_updates_last_reviewed
test_confirm_item_preserves_other_items
test_confirm_item_preserves_note_metadata
test_confirm_note_still_works
test_confirm_item_not_found
test_confirm_item_already_confirmed
test_confirm_item_roundtrip_save_load
test_confirm_target_detection_item_format
test_confirm_target_detection_domain_format
```

## Risques

- Le format d'id `{domain}-{number}` peut confluer avec des noms de domaine qui contiennent des tirets (ex: `user-auth`). Il faut une heuristique claire : si le target se termine par `-\d+`, c'est un item. Sinon, c'est un domaine.
- Attention a ne pas ecraser le contenu markdown de la note lors de la sauvegarde. Le roundtrip front-matter + contenu doit etre propre.
