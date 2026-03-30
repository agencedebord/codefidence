# 010 — check-diff : calcul de sensibilite

## Statut : termine

## Horizon : 30 jours — Semaine 3

## Dependances : 009

## Contexte

`check-diff` doit calculer un niveau de sensibilite global pour le diff analyse. Ce niveau permet a l'utilisateur de savoir rapidement si le changement touche une zone risquee. Les regles sont simples et inspectables, pas un scoring opaque.

## Ce qui doit etre implemente

### 1. Enum Sensitivity

```rust
pub enum Sensitivity {
    Low,
    Medium,
    High,
}
```

Serialisation : `"low"`, `"medium"`, `"high"`.
Affichage : `"faible"`, `"moyenne"`, `"elevee"` (FR dans la sortie texte).

### 2. Regles de calcul

**High** si au moins une de ces conditions :
- Presence d'un item `exception` dans les domaines touches
- Presence d'un item `decision` dans les domaines touches
- Note stale (> staleness_days) dans un domaine touche
- Note `needs-validation` ou `inferred` dans un domaine touche

**Medium** si :
- Domaine touche avec des memory_items mais aucun signal "high"

**Low** si :
- Domaines touches sans memory_items exploitables
- Ou uniquement des fichiers non resolus

### 3. Integration dans le resultat

La sensibilite fait partie du `CheckDiffResult` :
```rust
pub struct CheckDiffResult {
    pub files_analyzed: usize,
    pub domains: Vec<DomainHit>,
    pub unresolved_files: Vec<String>,
    pub sensitivity: Sensitivity,
    pub suggested_actions: Vec<String>,
}
```

### 4. Actions suggerees

Generer des suggestions basees sur la sensibilite et les warnings :
- High + stale : "Relire {note_path} avant de continuer"
- High + exception : "Verifier si l'exception '{text}' reste valide"
- High + decision : "Verifier si la decision '{text}' reste valide"
- Medium : "Consulter la memoire du domaine {domain} si le changement est significatif"
- Low : aucune suggestion

Maximum 3 suggestions.

## Fichiers a modifier

- `src/wiki/check_diff.rs` — calcul de sensibilite et suggestions

## Criteres de validation

### CV-1 : High sur exception
- Domaine touche avec 1 exception
- Sensibilite = High

### CV-2 : High sur decision
- Domaine touche avec 1 decision, pas d'exception
- Sensibilite = High

### CV-3 : High sur stale
- Domaine touche, note stale, aucun item type dangereux
- Sensibilite = High

### CV-4 : High sur low confidence
- Domaine touche, note inferred
- Sensibilite = High

### CV-5 : Medium avec business_rule seul
- Domaine touche avec uniquement des business_rule, note non stale, confidence ok
- Sensibilite = Medium

### CV-6 : Low sans memoire
- Domaine touche sans memory_items
- Sensibilite = Low

### CV-7 : Low fichiers non resolus uniquement
- Tous les fichiers non resolus
- Sensibilite = Low

### CV-8 : Suggestion stale
- Note stale + High
- Suggestion contient "Relire" + path de la note

### CV-9 : Suggestion exception
- Exception presente
- Suggestion contient "Verifier si l'exception"

### CV-10 : Max 3 suggestions
- Multiple warnings
- Maximum 3 suggestions generees

### CV-11 : Aucune suggestion en Low
- Sensibilite Low
- Pas de suggestion

## Tests a ecrire

```
test_sensitivity_high_on_exception
test_sensitivity_high_on_decision
test_sensitivity_high_on_stale
test_sensitivity_high_on_low_confidence
test_sensitivity_medium_business_rule_only
test_sensitivity_low_no_memory
test_sensitivity_low_unresolved_only
test_suggestion_stale_note
test_suggestion_exception
test_suggestion_decision
test_suggestion_max_3
test_suggestion_none_on_low
test_sensitivity_serialization
```

## Risques

- Les regles sont volontairement simples. Resister a la tentation d'ajouter du scoring fin. Si les regles binaires ne suffisent pas, c'est un signal pour enrichir la memoire, pas pour complexifier le calcul.
