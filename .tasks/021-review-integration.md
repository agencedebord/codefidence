# 021 — Integration review / CI

## Statut : termine

## Horizon : 90 jours — Semaine 10

## Dependances : 011, 020

## Contexte

Le 3e moment d'usage prioritaire est la review de PR. L'objectif est de faire remonter la memoire pertinente en commentaire de PR ou en check CI. La premiere integration doit etre la plus legere possible.

## Ce qui doit etre implemente

### 1. Script CI minimal

Un script ou une GitHub Action qui :
1. Execute `project-wiki check-diff --json` sur le diff de la PR
2. Parse le JSON
3. Si sensibilite >= medium : poste un commentaire de PR

### 2. Format du commentaire PR

```markdown
## 🧠 project-wiki — Memory Check

**Sensitivity: high**

### Domains touched
- **billing** (2 files, 3 memory items)

### Priority memory
| Type | Item | Confidence |
|------|------|------------|
| exception | Le client X utilise encore l'ancien calcul | confirmed |
| decision | Pas de deduplication des lignes importees | verified |

### Warnings
- ⚠ billing/_overview.md is stale (42 days)

### Suggested actions
- Review `.wiki/domains/billing/_overview.md` before merging
```

### 3. GitHub Action

```yaml
name: Wiki Memory Check
on: [pull_request]
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo install project-wiki
      - run: project-wiki check-diff --json > /tmp/wiki-check.json
      - uses: actions/github-script@v7
        with:
          script: |
            // Parse JSON and post comment if needed
```

### 4. Mode silencieux

Si sensibilite = low, ne pas poster de commentaire (eviter le bruit).

## Fichiers a creer

- `.github/workflows/wiki-check.yml` (template)
- Script de formatting du commentaire
- Documentation d'integration

## Criteres de validation

### CV-1 : Le script fonctionne en CI
- Sur une PR avec des fichiers modifies dans un domaine documente
- Commentaire poste avec la memoire pertinente

### CV-2 : Pas de commentaire si low
- PR touchant des fichiers sans memoire
- Aucun commentaire poste

### CV-3 : Commentaire lisible
- Le commentaire est compris en moins de 30 secondes

### CV-4 : Idempotence
- Deux runs sur la meme PR ne creent pas de doublons
- Le commentaire precedent est mis a jour ou un seul est maintenu

## Tests a ecrire

```
test_ci_script_parses_json
test_ci_script_formats_comment
test_ci_script_skips_low_sensitivity
test_ci_comment_idempotent
```
