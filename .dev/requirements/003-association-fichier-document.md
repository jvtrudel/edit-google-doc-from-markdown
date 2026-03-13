# REQ-003 — Association fichier Markdown ↔ Google Doc

**Date :** 2026-03-13  
**Statut :** Proposé  
**Type :** Fonctionnel  
**Priorité :** Critique

## Description

Le système doit maintenir une association persistante entre un fichier Markdown local et un Google Doc distant. Cette association permet d'identifier quel document correspond à quel fichier lors des opérations de push et pull.

L'association est de type **1:1** — un fichier Markdown correspond à exactement un Google Doc.

## Critères d'acceptation

- [ ] Un fichier Markdown peut être lié à un Google Doc (par son identifiant)
- [ ] L'association est persistante entre les exécutions (stockée dans un fichier de configuration ou métadonnées)
- [ ] Un fichier Markdown peut être délié d'un Google Doc
- [ ] Les associations existantes peuvent être listées
- [ ] Un push/pull sans association explicite crée automatiquement l'association

## Besoins associés

- `NEED-001` — Synchronisation Markdown ↔ Google Docs

## Fonctionnalités associées

- `FEAT-001` — Publication Markdown → Google Doc
- `FEAT-002` — Récupération Google Doc → Markdown
