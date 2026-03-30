# 009 — check-diff : resolution domaine + agregation memoire

## Statut : termine

## Horizon : 30 jours — Semaine 3

## Dependances : 002, 008

## Contexte

C'est le coeur de `check-diff` : resoudre les fichiers modifies vers des domaines, puis agreger les memory_items associes. Cette tache reutilise au maximum l'infrastructure existante (file_index, note parsing).

## Ce qui doit etre implemente

### 1. Resolution fichier -> domaine

Pour chaque fichier modifie :
1. Chercher dans `.file-index.json` (match exact)
2. Fallback structurel (resolution par chemin, existant dans le codebase)
3. Si non resolu : marquer comme "unresolved"

### 2. Agregation par domaine

Regrouper les fichiers par domaine resolu :
```rust
struct DomainHit {
    name: String,
    role: DomainRole,          // Primary, Secondary
    modified_files: Vec<String>,
    memory_items: Vec<MemoryItem>,
    warnings: Vec<DriftWarning>,
}
```

### 3. Classification primary / secondary

- Le domaine **principal** est celui avec le plus de fichiers modifies
- En cas d'egalite : celui avec le plus de memory_items
- Jusqu'a **2 domaines secondaires** en plus du principal
- Les domaines au-dela de 3 sont ignores (mentionner "+N autres domaines")

### 4. Chargement des memory_items par domaine

Pour chaque domaine touche :
1. Charger la note `_overview.md`
2. Lire les `memory_items`
3. Filtrer les items `deprecated`
4. Prioriser : exception > decision > business_rule
5. A type egal : confiance forte > relation fichier > recence
6. Limiter a `max_items` par domaine (default 3)

### 5. Enrichissement par relation fichier

Si un memory_item a un `related_files` qui matche un des fichiers modifies :
- Le marquer comme "directly related"
- Le prioriser dans l'affichage

### 6. Generation des warnings

Pour chaque domaine touche :
- Note stale (last_updated > staleness_days) -> warning stale
- Note low-confidence (inferred, needs-validation) -> warning
- Item low-confidence parmi les items remontes -> warning
- Aucun memory_item -> warning "no structured memory"

### 7. Fichiers non resolus

Si des fichiers ne matchent aucun domaine :
- Les lister dans une section "unresolved files"
- Ne pas les ignorer silencieusement

## Fichiers a modifier

- `src/wiki/check_diff.rs` — logique de resolution et agregation
- Reutiliser `src/wiki/file_index.rs` pour la resolution
- Reutiliser `src/wiki/note.rs` pour le chargement

## Criteres de validation

### CV-1 : Resolution basique
- 2 fichiers dans le domaine billing
- Resolution vers le domaine billing
- 1 DomainHit avec 2 modified_files

### CV-2 : Multi-domaine
- 1 fichier billing, 1 fichier auth
- 2 DomainHits distincts

### CV-3 : Primary selection par fichiers
- 2 fichiers billing, 1 fichier auth
- billing est primary, auth est secondary

### CV-4 : Primary selection par memory_items (egalite)
- 1 fichier billing (3 items), 1 fichier auth (1 item)
- billing est primary

### CV-5 : Limite 3 domaines
- 5 domaines touches
- 1 primary + 2 secondary + mention "+2 autres domaines"

### CV-6 : Memory items charges et priorises
- Domaine avec 1 exception, 1 decision, 1 business_rule, 1 item deprecated
- Sortie : exception, decision, business_rule (deprecated filtre)

### CV-7 : Enrichissement related_file
- Item avec related_file = "src/billing/invoice.ts"
- Fichier modifie = "src/billing/invoice.ts"
- L'item est marque "directly related" et priorise

### CV-8 : Warning stale
- Note last_updated il y a 45 jours, staleness_days = 30
- Warning stale present

### CV-9 : Warning low confidence
- Note confidence = inferred
- Warning present

### CV-10 : Warning no memory
- Domaine touche sans memory_items
- Warning "no structured memory for domain X"

### CV-11 : Fichiers non resolus
- Fichier hors de tout domaine
- Liste dans "unresolved files"

### CV-12 : Fallback si pas de file_index
- `.file-index.json` absent
- Resolution structurelle fonctionne quand meme

## Tests a ecrire

```
test_resolve_single_domain
test_resolve_multi_domain
test_primary_domain_by_file_count
test_primary_domain_by_item_count_on_tie
test_limit_3_domains
test_load_memory_items_prioritized
test_filter_deprecated_items
test_max_items_limit
test_related_file_enrichment
test_warning_stale_note
test_warning_low_confidence_note
test_warning_low_confidence_item
test_warning_no_memory_items
test_unresolved_files_listed
test_fallback_without_file_index
test_empty_file_list
```

## Risques

- La resolution structurelle peut etre imprecise. C'est acceptable au v0, documenter les limites.
- Le chargement de toutes les notes pour les domaines touches peut etre lent si beaucoup de domaines. Au v0, acceptable (max 3 domaines charges).
