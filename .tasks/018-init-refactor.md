# 018 — Refactor init (moins intrusif, opt-in)

## Statut : termine

## Horizon : 60 jours — Semaine 7

## Dependances : aucune

## Contexte

`init` est aujourd'hui trop invasif : il scanne, patche CLAUDE.md, installe des hooks, tout en une seule commande. L'objectif est de le rendre progressif et opt-in, pour que l'utilisateur garde le controle.

## Ce qui doit etre implemente

### 1. Init minimal par defaut

`project-wiki init` doit seulement :
1. Creer la structure `.wiki/`
2. Creer les templates
3. Creer le `config.toml`
4. Message de bienvenue avec les prochaines etapes

### 2. Etapes opt-in

Chaque etape supplementaire devient une option explicite :

```bash
project-wiki init              # Structure seulement
project-wiki init --scan       # + scan du codebase
project-wiki init --hooks      # + installation hooks Claude
project-wiki init --full       # Tout (equivalent ancien comportement)
```

### 3. Mode interactif optionnel

Si lance sans options et avec un terminal interactif :
- Proposer les etapes une par une
- "Voulez-vous scanner le codebase ? [Y/n]"
- "Voulez-vous installer les hooks Claude ? [Y/n]"
- "Voulez-vous importer depuis Notion ? [y/N]"

### 4. Idempotence

- `init` sur un wiki existant ne casse rien
- Message "Wiki already initialized" + proposition de re-scan ou update

### 5. Suppression du patch automatique de CLAUDE.md

Le patch de CLAUDE.md ne doit plus etre automatique. Proposer la commande separee :
```bash
project-wiki install-claude-md
```

## Fichiers a modifier

- `src/init/mod.rs` — refactor du flow principal
- `src/cli.rs` — nouveaux flags
- `src/init/hooks.rs` — extraction en commande separee
- `src/init/patch_claude.rs` — extraction en commande separee

## Criteres de validation

### CV-1 : Init minimal
- `project-wiki init` sur un repo vide
- `.wiki/` cree avec structure minimale
- Pas de scan, pas de hooks, pas de patch CLAUDE.md

### CV-2 : Init --scan
- `project-wiki init --scan`
- Structure + scan execute
- Notes de domaine generees

### CV-3 : Init --hooks
- `project-wiki init --hooks`
- Hooks installes

### CV-4 : Init --full
- Equivalent a l'ancien comportement complet

### CV-5 : Idempotence
- Double init -> pas de crash, pas de perte de donnees

### CV-6 : CLAUDE.md pas touche par defaut
- Init sans `--full`
- CLAUDE.md inchange

### CV-7 : Regression zero
- Les tests existants d'init passent toujours
- Le flow `--full` est fonctionnellement identique a l'ancien

## Tests a ecrire

```
test_init_minimal_creates_structure
test_init_minimal_no_scan
test_init_minimal_no_hooks
test_init_with_scan
test_init_with_hooks
test_init_full_equivalent
test_init_idempotent
test_init_no_claude_md_patch_by_default
test_init_existing_wiki_message
```

## Risques

- Les utilisateurs existants qui ont des scripts dependant du comportement actuel d'init seront casses. Documenter le changement dans CHANGELOG.
