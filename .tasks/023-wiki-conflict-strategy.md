# 023 — Strategie conflits .wiki/ en equipe

## Statut : a faire

## Horizon : 90 jours — Semaine 11

## Dependances : aucune

## Contexte

Le format `.wiki/` markdown est ideal pour un dev solo mais cree des conflits de merge en equipe. Cette tache definit et implemente une strategie pour limiter les collisions.

## Ce qui doit etre implemente

### 1. Separation fichiers generes vs edites

Clarifier dans la doc et dans le code :
- **Generes** (regenerables, pas de merge necessaire) : `_index.md`, `_graph.md`, `_needs-review.md`, `.file-index.json`
- **Edites** (contenu humain, merge necessaire) : `domains/*/_overview.md`, `decisions/*`

### 2. Gitattributes pour les fichiers generes

```gitattributes
.wiki/_index.md merge=ours
.wiki/_graph.md merge=ours
.wiki/_needs-review.md merge=ours
.wiki/.file-index.json merge=ours
```

Les fichiers generes prennent toujours la version locale. `rebuild` les regenere.

### 3. Structure de notes atomiques

Au lieu d'un gros `_overview.md` par domaine, permettre des notes separees par item :
```
.wiki/domains/billing/
  _overview.md         # metadata + narration
  billing-001.md       # un item par fichier (optionnel)
  billing-002.md
```

Ceci est une evolution optionnelle, pas obligatoire au MVP.

### 4. Commande rebuild

`project-wiki rebuild` doit regenerer tous les fichiers generes a partir des notes editees. C'est la resolution de conflit : en cas de doute, rebuild.

### 5. Documentation

Guide pratique pour les equipes :
- Quand merger les notes
- Quand rebuild
- Comment gerer les conflits sur les items

## Fichiers a creer/modifier

- `.wiki/.gitattributes` (template genere par init)
- Documentation dans README ou guide dedie
- `src/wiki/index.rs` — s'assurer que rebuild est robuste

## Criteres de validation

### CV-1 : Gitattributes genere
- `init` cree `.wiki/.gitattributes` avec les bonnes regles

### CV-2 : Rebuild regenere les fichiers
- Modifier `_index.md` manuellement
- `rebuild` le regenere correctement

### CV-3 : Notes editees preservees
- `rebuild` ne touche pas aux `_overview.md` edites par l'humain

### CV-4 : Documentation claire
- Un guide explique la strategie de merge pour les equipes

## Tests a ecrire

```
test_gitattributes_generated_on_init
test_rebuild_regenerates_index
test_rebuild_regenerates_graph
test_rebuild_preserves_overview_notes
test_rebuild_preserves_memory_items
```

## Risques

- Les notes atomiques par item ajoutent de la complexite au parsing (scanner N fichiers par domaine au lieu de 1). A evaluer avant d'implementer.
- Les gitattributes `merge=ours` necessitent une config git locale. Documenter la procedure.
