# 017 — Hardening scan TypeScript

## Statut : termine

## Horizon : 60 jours — Semaine 6

## Dependances : aucune

## Contexte

Le scan actuel supporte TS/JS mais traite tous les langages avec la meme profondeur. Pour le MVP cible TypeScript, le scan doit etre plus precis : meilleurs domaines, moins de faux positifs sur les dependances, meilleure detection des patterns TS courants.

## Ce qui doit etre implemente

### 1. Meilleure detection de domaines TypeScript

Patterns supplementaires a detecter :
- Next.js : `app/`, `pages/` avec conventions de routing
- Express : `routes/`, `controllers/`, `middleware/`
- NestJS : `modules/`, decorateurs `@Module`, `@Controller`
- Monorepos : `packages/*/src/`

### 2. Meilleure extraction d'imports TS

Ameliorer les regex existantes pour :
- `import type { X } from 'Y'` (type-only imports)
- `export * from 'Y'` (re-exports)
- Path aliases (`@/billing/invoice` -> resolution relative)
- Dynamic imports `import('Y')`

### 3. Moins de faux positifs

- Ne pas creer de dependance sur les imports de libraries externes (node_modules)
- Ne pas creer de dependance sur les types utilitaires globaux
- Distinguer import de type (faible) vs import de valeur (fort)

### 4. Meilleure extraction de modeles

- Interfaces TypeScript avec leurs champs
- Type aliases complexes
- Zod schemas (`z.object({...})`)
- Prisma models si detectes

### 5. Detection de tests plus robuste

- `*.test.ts`, `*.spec.ts`
- `__tests__/` directories
- `vitest`, `jest` patterns
- `describe()`, `it()`, `test()` dans le code

## Fichiers a modifier

- `src/init/scan/structure.rs` — patterns de domaine
- `src/init/scan/imports.rs` — extraction d'imports
- `src/init/scan/details.rs` — extraction de modeles

## Criteres de validation

### CV-1 : Next.js app router
- Repo Next.js avec `app/billing/page.tsx`
- Domaine billing detecte

### CV-2 : Path aliases resolus
- Import `@/billing/invoice` avec tsconfig alias
- Dependance correctement resolue

### CV-3 : Pas de dependance sur node_modules
- `import express from 'express'`
- Pas de domaine "express" cree

### CV-4 : Type imports distingues
- `import type { Invoice }` vs `import { createInvoice }`
- Le type import ne cree pas de dependance forte

### CV-5 : Zod schemas detectes
- `const InvoiceSchema = z.object({...})`
- Detecte comme modele

### CV-6 : Regression zero
- Les tests existants du scan passent toujours

## Tests a ecrire

```
test_scan_nextjs_app_router
test_scan_nextjs_pages_router
test_scan_express_routes
test_scan_monorepo_packages
test_scan_ts_type_only_import
test_scan_ts_reexport
test_scan_ts_path_alias
test_scan_ts_dynamic_import
test_scan_no_dependency_on_external
test_scan_zod_schema_detected
test_scan_prisma_model_detected
test_scan_vitest_patterns
test_regression_existing_scan_tests
```

## Risques

- Les path aliases necessitent de lire `tsconfig.json`. Cela ajoute de la complexite. Au MVP, on peut se contenter de resoudre `@/` comme racine du `src/`.
- Ne pas essayer de faire un compilateur TS. Les heuristiques suffisent.
