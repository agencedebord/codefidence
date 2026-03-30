# 008 — check-diff : collecte des fichiers modifies

## Statut : termine

## Horizon : 30 jours — Semaine 3

## Dependances : 007

## Contexte

`check-diff` doit savoir quels fichiers ont ete modifies. Soit l'utilisateur les passe en arguments, soit la commande les recupere via `git diff --name-only`.

## Ce qui doit etre implemente

### 1. Mode explicite (fichiers en arguments)

Si des fichiers sont passes en arguments :
- Les utiliser directement
- Verifier que chaque fichier existe (warning si non, pas erreur bloquante)
- Normaliser les chemins relatifs par rapport a la racine du repo

### 2. Mode git diff (par defaut)

Si aucun fichier passe :
- Executer `git diff --name-only` (unstaged changes)
- Parser la sortie : une ligne = un fichier
- Filtrer les fichiers supprimes (qui n'existent plus)

### 3. Mode staged (futur)

Si `--staged` :
- Executer `git diff --cached --name-only`
- Meme logique de parsing

### 4. Cas vide

Si aucun fichier modifie :
- Message clair : "No modified files detected."
- Code de sortie 0

### 5. Filtrage

- Ignorer les fichiers dans `.wiki/` (pas de recursion)
- Ignorer les fichiers binaires courants
- Ignorer `node_modules/`, `target/`, `dist/`, etc.

## Fichiers a modifier

- `src/wiki/check_diff.rs` — fonction de collecte

## Criteres de validation

### CV-1 : Mode explicite
- Passer 3 fichiers en arguments
- La fonction retourne ces 3 fichiers

### CV-2 : Fichier inexistant en mode explicite
- Passer un fichier qui n'existe pas
- Warning emis, fichier ignore, pas de crash

### CV-3 : Mode git diff
- Avoir des fichiers modifies dans le worktree
- La fonction retourne la bonne liste

### CV-4 : Mode git diff sans changements
- Worktree propre
- Retourne un Vec vide, message "No modified files detected."

### CV-5 : Filtrage .wiki/
- Modifier un fichier dans `.wiki/`
- Il n'apparait pas dans la liste

### CV-6 : Normalisation des chemins
- Passer `./src/billing/invoice.ts`
- Le chemin est normalise en `src/billing/invoice.ts`

### CV-7 : Git non disponible
- Pas de repo git
- Erreur claire : "Not a git repository" ou equivalent

### CV-8 : Fichiers supprimes
- Un fichier dans git diff qui a ete supprime
- Il est filtre de la liste

## Tests a ecrire

```
test_collect_files_explicit_mode
test_collect_files_explicit_nonexistent_warning
test_collect_files_git_diff_mode (necessite fixture git)
test_collect_files_empty_diff
test_collect_files_filter_wiki_dir
test_collect_files_filter_node_modules
test_collect_files_normalize_paths
test_collect_files_no_git_repo
test_collect_files_deleted_files_filtered
```

## Notes d'implementation

- Utiliser `std::process::Command` pour executer `git diff`
- Attention a la gestion d'erreur de la commande git (code de sortie, stderr)
- Les tests unitaires peuvent mocker la liste de fichiers ; les tests d'integration utiliseront un vrai repo git temporaire
