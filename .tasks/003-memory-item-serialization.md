# 003 — Serialisation memory_items vers YAML

## Statut : termine (implemente avec 001)

## Horizon : 30 jours — Semaine 1

## Dependances : 001

## Contexte

Quand le produit ecrit ou met a jour une note (via `add`, `promote`, `confirm`, ou le scan), il doit pouvoir serialiser les memory_items dans le front-matter YAML. Le format doit etre lisible par un humain et re-parseable par le produit.

## Ce qui doit etre implemente

### 1. Serialisation WikiNote -> front-matter YAML

La serialisation doit produire un front-matter valide avec les memory_items integres.

Sortie attendue :
```yaml
---
domain: billing
confidence: verified
last_updated: "2026-03-29"
related_files:
  - src/billing/invoice.ts
deprecated: false
memory_items:
  - id: billing-001
    type: exception
    text: Le client X utilise encore l'ancien calcul
    confidence: confirmed
    related_files:
      - src/billing/legacy_pricing.ts
    sources:
      - kind: file
        ref: src/billing/legacy_pricing.ts
    status: active
    last_reviewed: "2026-03-29"
---
```

### 2. Serialisation sans memory_items

Si `memory_items` est vide, le champ ne doit pas apparaitre dans le YAML (utiliser `#[serde(skip_serializing_if = "Vec::is_empty")]`).

### 3. Ecriture sur disque

Adapter la fonction qui ecrit les notes pour inclure les memory_items dans le front-matter avant le contenu markdown.

### 4. Roundtrip garanti

parse(serialize(note)) == note pour toutes les variantes.

## Fichiers a modifier

- `src/wiki/note.rs` — serialisation
- Toute fonction qui ecrit des notes sur disque (scaffold, add, etc.)

## Criteres de validation

### CV-1 : Serialisation complete
- Un WikiNote avec 2 memory_items se serialise en YAML valide
- Le YAML contient tous les champs de chaque item

### CV-2 : Champs renommes correctement
- `type_` -> `type` dans le YAML
- `ref_` -> `ref` dans le YAML

### CV-3 : Champs optionnels omis
- `last_reviewed: None` -> pas de champ `last_reviewed` dans le YAML de l'item
- `line: None` dans source -> pas de champ `line`

### CV-4 : memory_items vide -> champ absent
- Un WikiNote avec `memory_items: vec![]` ne produit pas de clef `memory_items` dans le YAML

### CV-5 : Roundtrip
- Creer un WikiNote complet avec memory_items
- Serialiser -> deserialiser
- Verifier egalite sur tous les champs

### CV-6 : Lisibilite humaine
- Le YAML produit est indente proprement
- Les listes sont formatees avec `-` sur des lignes separees
- Pas de notation inline `{}`

### CV-7 : Contenu markdown preserve apres ecriture
- Ecrire un WikiNote avec du contenu markdown
- Re-lire le fichier
- Le contenu markdown est intact apres le front-matter

## Tests a ecrire

```
test_serialize_note_with_memory_items
test_serialize_note_without_memory_items_omits_field
test_serialize_type_rename
test_serialize_ref_rename
test_serialize_optional_fields_omitted
test_serialize_roundtrip_full
test_serialize_roundtrip_no_items
test_serialize_preserves_markdown_content
test_serialize_yaml_formatting_readable
```

## Risques

- `serde_yml` peut produire un formatage YAML different de ce qu'on attend (inline vs block). Tester le format de sortie tot.
- Les `skip_serializing_if` doivent etre coherents entre serialisation et deserialisation.
