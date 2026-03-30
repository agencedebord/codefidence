# 013 — README repositionne

## Statut : a faire

## Horizon : 30 jours — Semaine 1 (parallele)

## Dependances : aucune

## Contexte

Le README actuel survend le scan et le graph, et sous-definit l'usage diff-aware et le moment de valeur reel. Il doit etre realigne sur le positionnement : memoire decisionnelle repo-native pour les changements de code assistes par IA.

## Ce qui doit etre implemente

### 1. Repositionner le pitch

**Avant** (implicite) : "on genere de la documentation intelligente"

**Apres** : "on empeche un humain ou une IA de casser une regle implicite du projet"

### 2. Restructurer les sections

Ordre cible :
1. Pitch en une phrase
2. Le probleme (connaissances implicites, regressions metier)
3. La solution (memoire repo-native, injectee avant changement, reverifiee apres)
4. Quick start
5. Commandes principales (centrees sur `context`, `check-diff`, `detect-drift`)
6. Comment ca marche (memory items, confiance, taxonomie)
7. Commandes secondaires
8. Installation

### 3. Diminuer le role du graph

- Le graph n'est plus dans les features principales
- Il est mentionne comme vue auxiliaire
- Pas de screenshot ou schema du graph en hero

### 4. Mettre en avant les 3 moments

1. Avant edition : `context`
2. Apres diff : `check-diff`
3. En review : integration CI (a venir)

### 5. Etre honnete sur l'etat

- Mentionner que le produit est en phase de validation
- Ne pas promettre ce qui n'existe pas encore
- Lister les langages supportes sans survendre la profondeur

### 6. Exemples concrets

Inclure au moins un exemple de sortie de `context` et un de `check-diff` (meme simule si la commande n'est pas encore prete au moment de l'ecriture).

## Fichiers a modifier

- `README.md`

## Criteres de validation

### CV-1 : Positionnement clair
- La premiere phrase du README parle de protection contre les changements mal informes, pas de generation de documentation

### CV-2 : Probleme avant solution
- Le README explique le probleme AVANT de presenter le produit

### CV-3 : 3 moments visibles
- Les 3 moments d'usage (avant edition, apres diff, en review) sont clairement presentes

### CV-4 : Graph diminue
- Le graph n'est pas dans le hero ni dans les 3 premieres features

### CV-5 : Honnetete
- L'etat actuel est represente fidelement
- Pas de promesse de support multi-source profond

### CV-6 : Quick start fonctionnel
- Les commandes du quick start fonctionnent reellement

### CV-7 : Exemples de sortie
- Au moins un exemple de sortie `context`
- Au moins un exemple de sortie `check-diff`

## Ce n'est PAS dans le scope

- Refaire le CONTRIBUTING.md
- Refaire le CHANGELOG.md
- Changer le nom du produit
- Ajouter des badges ou du marketing

## Risques

- Le README doit etre mis a jour a nouveau apres l'implementation de `check-diff` si l'exemple etait simule.
