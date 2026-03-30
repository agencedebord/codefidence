# 011 — check-diff : sortie texte et JSON

## Statut : termine

## Horizon : 30 jours — Semaine 3

## Dependances : 009, 010

## Contexte

`check-diff` doit produire une sortie lisible par un humain (texte) et une sortie machine (JSON). Les deux formats derivent du meme `CheckDiffResult`. La sortie texte est la priorite, la sortie JSON est necessaire pour les tests et la future integration CI.

## Ce qui doit etre implemente

### 1. Sortie texte (par defaut)

Format cible (spec check-diff v0) :

```text
[project-wiki] Diff check

2 fichiers analyses
1 domaine principal touche
Sensibilite: elevee

Domaines touches
  billing (primary) — 2 fichiers, 3 items
  auth (secondary) — 1 fichier, 1 item

Memoire prioritaire
  billing:
    [exception] Le client X utilise encore l'ancien calcul [confirmed]
    [decision] Pas de deduplication des lignes importees [verified]
    [business_rule] La facture est emise apres synchro [seen-in-code]
  auth:
    [exception] L'endpoint /legacy reste actif pour compat [confirmed]

Warnings
  ⚠ billing/_overview.md est stale (42 jours)
  ⚠ 1 item inferred dans auth

Actions recommandees
  → Relire .wiki/domains/billing/_overview.md
  → Verifier si l'exception 'Le client X...' reste valide

Fichiers non resolus
  config/deploy.yaml
```

### 2. Sortie JSON (`--json`)

Structure JSON (spec check-diff v0) :

```json
{
  "files_analyzed": 2,
  "sensitivity": "high",
  "domains": [
    {
      "name": "billing",
      "role": "primary",
      "files": ["src/billing/invoice.ts", "src/billing/service.ts"],
      "memory_items": [
        {
          "id": "billing-001",
          "type": "exception",
          "text": "Le client X utilise encore l'ancien calcul",
          "confidence": "confirmed",
          "directly_related": true,
          "source_note": ".wiki/domains/billing/_overview.md"
        }
      ],
      "warnings": [
        {
          "kind": "stale",
          "note": ".wiki/domains/billing/_overview.md",
          "days": 42
        }
      ]
    }
  ],
  "unresolved_files": ["config/deploy.yaml"],
  "suggested_actions": [
    "Relire .wiki/domains/billing/_overview.md"
  ]
}
```

### 3. Coloration console

- Sensibilite elevee : rouge
- Sensibilite moyenne : jaune
- Sensibilite faible : vert
- Types entre crochets : bold
- Warnings : jaune avec symbole ⚠
- Actions : cyan avec fleche →

Utiliser le crate `console` deja present dans les dependances.

### 4. Cas vide

Si aucun fichier modifie :
```text
[project-wiki] Diff check

Aucun fichier modifie detecte.
```

JSON : `{ "files_analyzed": 0, "sensitivity": "low", "domains": [], "unresolved_files": [], "suggested_actions": [] }`

### 5. Verbosity

- Normal : sortie compacte comme ci-dessus
- `-v` : ajouter les chemins complets des fichiers par domaine
- `-vv` : ajouter les sources de chaque memory_item

## Fichiers a modifier

- `src/wiki/check_diff.rs` — fonctions de formatage
- Eventuellement `src/ui/mod.rs` si des helpers d'affichage sont partages

## Criteres de validation

### CV-1 : Sortie texte complete
- Resultat avec 2 domaines, 4 items, 2 warnings, 1 fichier non resolu
- Toutes les sections presentes dans la sortie

### CV-2 : Sortie texte cas vide
- Aucun fichier modifie
- Message "Aucun fichier modifie detecte."

### CV-3 : Sortie JSON valide
- La sortie avec `--json` est du JSON parseable
- Tous les champs de la spec sont presents

### CV-4 : Sortie JSON cas vide
- JSON valide avec listes vides

### CV-5 : Coherence texte / JSON
- Meme input -> les informations dans le texte et le JSON sont identiques
- Le nombre de fichiers, domaines, items correspond

### CV-6 : Sensibilite coloree (si terminal)
- High -> rouge
- Medium -> jaune
- Low -> vert

### CV-7 : Limite d'items respectee
- max_items = 2 avec 5 items disponibles
- Sortie montre 2 items + mention "+3 autres"

### CV-8 : Verbosity -v
- Avec -v, les chemins des fichiers par domaine sont affiches

### CV-9 : JSON serialisable par serde
- Le CheckDiffResult se serialise directement avec serde_json

### CV-10 : Pas de couleur si stdout n'est pas un terminal
- Redirection vers fichier -> pas de codes ANSI

## Tests a ecrire

```
test_output_text_full
test_output_text_empty
test_output_text_single_domain
test_output_text_multi_domain
test_output_text_with_warnings
test_output_text_with_unresolved
test_output_text_sensitivity_label
test_output_json_valid
test_output_json_empty
test_output_json_full_structure
test_output_json_matches_text_content
test_output_max_items_truncation
test_output_verbosity_v
```

## Notes

- Utiliser des snapshot tests (crate `insta`, deja en dev-dependencies) pour valider le format de sortie texte.
- Le JSON doit etre stable : l'ordre des champs ne doit pas varier entre les runs.
