# REQ-007 — Authentification par compte de service

**Date :** 2026-03-13  
**Statut :** Proposé  
**Type :** Fonctionnel  
**Priorité :** Critique

## Description

L'accès aux Google Docs se fait via un **compte de service Google** (clé JSON). Aucune interaction OAuth utilisateur n'est requise. Les documents cibles doivent être partagés avec le compte de service.

## Critères d'acceptation

- [ ] Le système s'authentifie auprès de l'API Google Docs via une clé JSON de compte de service
- [ ] La clé est chargée depuis un chemin configurable (variable d'environnement ou fichier `.env`)
- [ ] Aucune interaction utilisateur n'est nécessaire pour l'authentification

## Besoins associés

- `NEED-001` — Synchronisation Markdown ↔ Google Docs

## Fonctionnalités associées

- `FEAT-001` — Publication Markdown → Google Doc
- `FEAT-002` — Récupération Google Doc → Markdown
