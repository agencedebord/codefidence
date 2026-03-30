# 019 — Hardening validate

## Statut : a faire

## Horizon : 60 jours — Semaine 7

## Dependances : 004

## Contexte

La tache 004 ajoute la validation des memory_items. Cette tache durcit la validation globale : meilleure detection des incoherences, refus explicite des champs invalides, reporting plus clair.

## Ce qui doit etre implemente

### 1. Validation des metadonnees de note

- `domain` obligatoire et non vide
- `confidence` valide
- `last_updated` format date valide
- `related_files` : warning si fichier reference n'existe pas sur le disque

### 2. Validation croisee

- Un domaine reference dans les dependencies d'une note doit exister dans `.wiki/domains/`
- Un `related_file` reference dans un item doit etre un sous-ensemble des fichiers du domaine (warning, pas erreur)
- Coherence entre le nom du dossier domaine et le champ `domain` du front-matter

### 3. Reporting structure

Sortie organisee :
```text
[validate] 3 errors, 5 warnings

Errors:
  ✗ .wiki/domains/billing/_overview.md: duplicate item id 'billing-001'
  ✗ .wiki/domains/auth/_overview.md: unknown type 'custom'
  ✗ .wiki/domains/billing/_overview.md: source missing ref

Warnings:
  ⚠ .wiki/domains/billing/_overview.md: related_file 'src/old.ts' not found on disk
  ⚠ .wiki/domains/billing/_overview.md: item 'billing-003' has no sources
  ⚠ .wiki/domains/auth/_overview.md: dependency 'payments' not found in wiki
  ⚠ .wiki/domains/billing/_overview.md: item confirmed but note is inferred
  ⚠ .wiki/domains/billing/_overview.md: last_reviewed is in the future
```

### 4. Code de sortie

- 0 : pas d'erreur (warnings possibles)
- 1 : au moins une erreur

### 5. Mode strict optionnel

`project-wiki validate --strict` : les warnings deviennent des erreurs.

## Fichiers a modifier

- `src/wiki/validate.rs` — enrichissement
- `src/cli.rs` — flag --strict

## Criteres de validation

### CV-1 : Domaine manquant en dependance
- Note billing depend de "payments", "payments" n'existe pas
- Warning emis

### CV-2 : Related_file inexistant
- Item reference "src/old.ts" qui n'existe pas
- Warning emis

### CV-3 : Incoherence nom dossier / champ domain
- Dossier `.wiki/domains/billing/` mais front-matter `domain: invoicing`
- Erreur emise

### CV-4 : Sortie structuree
- Erreurs et warnings separes
- Chaque message contient le chemin du fichier concerne

### CV-5 : Mode strict
- Warning "related_file not found" devient erreur avec --strict

### CV-6 : Code de sortie correct
- Erreurs -> code 1
- Warnings seulement -> code 0
- Strict + warnings -> code 1

## Tests a ecrire

```
test_validate_missing_dependency_warning
test_validate_related_file_not_on_disk
test_validate_domain_name_mismatch
test_validate_structured_output
test_validate_exit_code_errors
test_validate_exit_code_warnings_only
test_validate_strict_mode
test_validate_all_clean
```
