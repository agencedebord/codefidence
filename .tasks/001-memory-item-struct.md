# 001 — Struct MemoryItem + types

## Statut : termine

## Horizon : 30 jours — Semaine 1

## Dependances : aucune

## Contexte

Le format des memory items est defini dans la spec `project-wiki-memory-item-format.md`. Les items de memoire deviennent des objets structures dans le front-matter des notes de domaine. C'est le socle de tout : `context`, `check-diff`, `validate`, `promote` s'appuient dessus.

## Ce qui doit etre implemente

### 1. Enum `MemoryItemType`

Types MVP :
- `Decision`
- `BusinessRule`
- `Exception`

Serialisation kebab-case : `decision`, `business_rule`, `exception`

### 2. Enum `MemoryItemStatus`

- `Active`
- `Deprecated`

Serialisation kebab-case : `active`, `deprecated`

### 3. Struct `MemoryItemSource`

```rust
pub struct MemoryItemSource {
    pub kind: String,    // "file", "test", "comment", "ticket", "note"
    pub ref_: String,    // chemin ou reference
    pub line: Option<u32>, // optionnel, numero de ligne
}
```

Le champ `ref` est un mot reserve en Rust, utiliser `ref_` avec `#[serde(rename = "ref")]`.

### 4. Struct `MemoryItem`

```rust
pub struct MemoryItem {
    pub id: String,
    pub type_: MemoryItemType,   // serde rename "type"
    pub text: String,
    pub confidence: Confidence,
    pub related_files: Vec<String>,
    pub sources: Vec<MemoryItemSource>,
    pub status: MemoryItemStatus,
    pub last_reviewed: Option<NaiveDate>,
}
```

### 5. Extension de WikiNote

Ajouter un champ :
```rust
pub memory_items: Vec<MemoryItem>,
```

Default : vec vide (compatibilite avec notes existantes sans memory_items).

## Fichiers a modifier

- `src/wiki/note.rs` — ajout des structs et enums
- Eventuellement un nouveau fichier `src/wiki/memory_item.rs` si `note.rs` devient trop gros

## Criteres de validation

### CV-1 : Les types se serialisent correctement
- `MemoryItemType::Decision` -> `"decision"`
- `MemoryItemType::BusinessRule` -> `"business_rule"`
- `MemoryItemType::Exception` -> `"exception"`

### CV-2 : Les status se serialisent correctement
- `MemoryItemStatus::Active` -> `"active"`
- `MemoryItemStatus::Deprecated` -> `"deprecated"`

### CV-3 : MemoryItemSource gere le rename de `ref`
- La serialisation YAML produit `ref:` (pas `ref_:`)
- La deserialisation lit `ref:` et le mappe sur `ref_`

### CV-4 : MemoryItem roundtrip
- Creer un MemoryItem avec tous les champs remplis
- Le serialiser en YAML
- Le deserialiser
- Verifier que tous les champs sont identiques

### CV-5 : MemoryItem avec champs optionnels
- Un MemoryItem sans `last_reviewed` se serialise/deserialise sans erreur
- Un MemoryItemSource sans `line` se serialise/deserialise sans erreur

### CV-6 : WikiNote avec memory_items vide
- Une WikiNote sans `memory_items` dans le front-matter se deserialise avec un Vec vide
- Compatibilite arriere avec toutes les notes existantes

### CV-7 : WikiNote avec memory_items rempli
- Une WikiNote avec 2+ memory_items se parse correctement
- Chaque item a ses champs correctement mappes

### CV-8 : Confidence reutilise l'enum existante
- Le champ `confidence` de MemoryItem utilise la meme enum `Confidence` que WikiNote
- Pas de duplication de type

## Tests a ecrire

```
test_memory_item_type_serialization
test_memory_item_type_deserialization
test_memory_item_status_serialization
test_memory_item_status_deserialization
test_memory_item_source_ref_rename
test_memory_item_source_with_line
test_memory_item_source_without_line
test_memory_item_full_roundtrip
test_memory_item_without_last_reviewed
test_wiki_note_without_memory_items_compat
test_wiki_note_with_memory_items
test_wiki_note_existing_notes_still_parse (regression)
```

## Risques

- Le crate `gray_matter` + `serde_yml` doit supporter des structures imbriquees dans le front-matter. Verifier tot.
- Le champ `type` est reserve en Rust. Le rename serde doit fonctionner avec serde_yml.
