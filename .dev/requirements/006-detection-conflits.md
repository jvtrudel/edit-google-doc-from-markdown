# REQ-006 — Détection de conflits

**Date :** 2026-03-13  
**Statut :** Proposé  
**Type :** Fonctionnel  
**Priorité :** Important

## Description

Le système doit détecter les situations de **conflit** — lorsque le fichier Markdown ET le Google Doc ont été modifiés depuis la dernière synchronisation.

En cas de conflit détecté, le système doit :
- **Signaler** le conflit à l'utilisateur
- **Refuser** d'effectuer l'opération (push ou pull)
- **Afficher** les informations nécessaires pour comprendre le conflit (dates de dernière modification, résumé des différences si possible)

Le système ne propose **pas** de mécanisme de résolution automatique à ce stade. La résolution est laissée à l'utilisateur.

## Critères d'acceptation

- [ ] Le système détecte qu'un fichier Markdown a été modifié depuis le dernier push/pull
- [ ] Le système détecte qu'un Google Doc a été modifié depuis le dernier push/pull
- [ ] Si les deux ont été modifiés, l'opération est refusée avec un message d'erreur clair
- [ ] Le message d'erreur indique les dates de dernière modification des deux côtés
- [ ] Un mécanisme de forçage existe (`--force`) pour écraser un côté en connaissance de cause

## Besoins associés

- `NEED-001` — Synchronisation Markdown ↔ Google Docs

## Fonctionnalités associées

- `FEAT-001` — Publication Markdown → Google Doc
- `FEAT-002` — Récupération Google Doc → Markdown
