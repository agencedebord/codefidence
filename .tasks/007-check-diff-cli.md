# 007 — check-diff : commande CLI

## Statut : termine

## Horizon : 30 jours — Semaine 3

## Dependances : aucune (definition CLI pure)

## Contexte

`check-diff` est la nouvelle commande differenciante. Cette tache couvre uniquement la definition de la commande CLI avec clap, les arguments, et le squelette d'execution. La logique interne est dans les taches suivantes (008-011).

## Ce qui doit etre implemente

### 1. Commande clap

Ajouter `CheckDiff` au enum `Commands` dans `cli.rs`.

```rust
/// Check modified files against wiki memory
CheckDiff {
    /// Files to check (default: git diff --name-only)
    files: Vec<String>,

    /// Use staged changes only
    #[arg(long)]
    staged: bool,

    /// Output as JSON
    #[arg(long)]
    json: bool,

    /// Maximum memory items to show per domain
    #[arg(long, default_value = "3")]
    max_items: usize,
},
```

### 2. Squelette d'execution

Le handler doit :
1. Determiner la source de fichiers (args ou git diff)
2. Appeler la logique metier (taches 008-011)
3. Formater la sortie
4. Retourner le bon code de sortie

### 3. Code de sortie

- 0 : aucun risque detecte
- 0 : risque detecte mais pas une erreur (le produit informe, ne bloque pas)
- 1 : erreur technique (wiki introuvable, git diff echoue, etc.)

Note : au v0, `check-diff` ne bloque jamais. Il informe.

### 4. Help text

Le `--help` doit clairement decrire :
- ce que fait la commande
- ses limites (resolution fichier/domaine, pas d'analyse semantique)
- un exemple d'usage

## Fichiers a modifier

- `src/cli.rs` — ajout de la commande
- Nouveau fichier `src/wiki/check_diff.rs` — module pour la logique metier (vide pour l'instant)
- `src/wiki/mod.rs` — export du nouveau module

## Criteres de validation

### CV-1 : La commande existe
- `project-wiki check-diff --help` fonctionne
- Affiche la description et les arguments

### CV-2 : Arguments parses
- `project-wiki check-diff src/a.ts src/b.ts` -> files = ["src/a.ts", "src/b.ts"]
- `project-wiki check-diff --staged` -> staged = true
- `project-wiki check-diff --json` -> json = true
- `project-wiki check-diff --max-items 5` -> max_items = 5

### CV-3 : Mode par defaut
- `project-wiki check-diff` sans arguments -> utilise git diff

### CV-4 : Le binaire compile
- `cargo build` sans erreur
- `cargo test` sans regression

### CV-5 : Code de sortie sur erreur technique
- Pas de .wiki/ -> code 1 + message d'erreur

## Tests a ecrire

```
test_check_diff_command_exists
test_check_diff_parse_files_args
test_check_diff_parse_staged_flag
test_check_diff_parse_json_flag
test_check_diff_parse_max_items
test_check_diff_default_max_items_is_3
test_check_diff_error_no_wiki (integration)
```

## Risques

- Le nom `check-diff` avec un tiret : verifier que clap le gere bien avec le naming kebab-case.
- Le flag `--staged` n'est pas utilise au v0 mais doit etre prevu dans la signature pour eviter un breaking change.
