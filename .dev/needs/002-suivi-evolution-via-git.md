# Besoin 002 — Suivi de l'évolution du document via git

**Date :** 2026-03-13  
**Statut :** Identifié  
**Source :** Jérémy Viau-Trudel (développeur, utilisateur principal)

## Description

En tant que développeur, je veux pouvoir **suivre l'évolution des modifications** de mon document en utilisant les outils de versionnement que je maîtrise déjà (git). Le fichier Markdown étant du texte brut, il est naturellement compatible avec git : les diffs sont lisibles, les merges sont possibles, et l'historique complet est conservé.

Ce besoin renforce le choix du Markdown comme format de référence pour le contenu : c'est un format **diffable**, **mergeable** et **versionnable** — contrairement au format binaire/opaque de Google Docs.

## Contexte

Le développeur utilise déjà git pour versionner son code. Il veut appliquer les mêmes pratiques à ses documents :

- Voir **qui** a modifié **quoi** et **quand** (`git log`, `git blame`)
- Comparer deux versions d'un document (`git diff`)
- Revenir à une version antérieure (`git checkout`)
- Créer des branches pour des variantes ou brouillons
- Intégrer les modifications dans un flux de revue (pull requests)

Google Docs offre un historique de versions, mais il est linéaire, non branchable, et inaccessible aux outils du développeur.

## Critères de satisfaction

- [ ] Le fichier Markdown est un fichier texte brut versionnable avec git
- [ ] Les modifications entre deux synchronisations produisent des diffs lisibles et significatifs
- [ ] La synchronisation ne génère pas de changements parasites (reformatage inutile, réordonnancement, bruit dans les diffs)
- [ ] L'utilisateur peut utiliser son workflow git habituel (commit, diff, log, blame, branch, merge) sur le fichier Markdown

## Requis associés

- `REQ-001` — Conversion Markdown → Google Docs (la conversion doit être déterministe pour éviter le bruit dans les diffs)
- `REQ-002` — Conversion Google Docs → Markdown (la conversion doit être déterministe et stable)
- `REQ-009` — Stabilité et déterminisme du Markdown généré

## Fonctionnalités associées

- `FEAT-002` — Récupération Google Doc → Markdown (pull) — le Markdown produit doit être stable et diffable
