# REQ-013 — Stratégie de résolution de conflits

**Date :** 2026-03-14  
**Statut :** Proposé  
**Type :** Fonctionnel  
**Priorité :** Souhaitable

## Description

L'utilisateur doit pouvoir choisir la **stratégie de résolution de conflits** appliquée lors d'un push ou d'un pull. Cette stratégie détermine le comportement du système quand un conflit est détecté (cf. REQ-006).

La stratégie est contrôlée par une option CLI (`--on-conflict <stratégie>`) et/ou une variable d'environnement.

### Stratégies définies

| Stratégie | Comportement | Analogue git |
|---|---|---|
| `abort` **(défaut)** | Refuser l'opération, signaler le conflit. L'utilisateur doit résoudre manuellement. | `git merge --abort` |
| `force` | Écraser la destination sans poser de questions. Équivalent de `--force`. | `git push --force` |
| `ask` | Demander confirmation interactive à l'utilisateur avant de procéder. | — |
| `ours` | En cas de conflit, la source locale (Markdown) gagne toujours. Pour `push` : écrase le distant. Pour `pull` : refuse (le local est prioritaire). | `git merge -s ours` |
| `theirs` | En cas de conflit, la source distante (Google Doc) gagne toujours. Pour `pull` : écrase le local. Pour `push` : refuse (le distant est prioritaire). | `git merge -s theirs` |
| `backup` | Créer une copie de sauvegarde du côté qui sera écrasé (ex: `fichier.md.backup`), puis procéder à l'écrasement. | `git stash` (conceptuellement) |

### Ordre de priorité

1. Option CLI (`--on-conflict`) — priorité maximale
2. Variable d'environnement (`NOU_ON_CONFLICT`)
3. Valeur par défaut : `abort`

### Notes

- `--force` reste un raccourci pour `--on-conflict force`
- La stratégie `ask` n'est utilisable qu'en mode interactif (terminal). En mode non-interactif, elle se comporte comme `abort`.
- Les stratégies `ours` et `theirs` sont nommées du point de vue **local** (comme dans git) : `ours` = le fichier Markdown local, `theirs` = le Google Doc distant.

## Critères d'acceptation

- [ ] La stratégie par défaut est `abort` (aucune résolution automatique)
- [ ] L'option `--on-conflict` accepte les valeurs : `abort`, `force`, `ask`, `ours`, `theirs`, `backup`
- [ ] `--force` est un alias de `--on-conflict force`
- [ ] La variable d'environnement `NOU_ON_CONFLICT` est lue si l'option CLI n'est pas fournie
- [ ] Chaque stratégie produit un comportement cohérent pour push et pull
- [ ] La stratégie `backup` crée un fichier de sauvegarde avant d'écraser
- [ ] La stratégie `ask` affiche un prompt interactif demandant confirmation

## Besoins associés

- `NEED-001` — Synchronisation Markdown ↔ Google Docs

## Fonctionnalités associées

- `FEAT-001` — Publication Markdown → Google Doc
- `FEAT-002` — Récupération Google Doc → Markdown
