# Ticket 007 — Changement de portée du projet

**Branche :** `feat/change-portee-du-projet`  
**Date :** 2026-03-13  
**Phase :** 3 — Implémentation (terminée)  
**Type :** Architecture / Documentation

---

## Contexte

L'ADR-001 a été généré automatiquement sans instruction précise. Il ne définit pas bien l'objectif de ce projet. Le projet vise à synchroniser manuellement un fichier Markdown avec un Google Doc, en séparant le contenu (Markdown) du style (Google Docs).

## Objectif

1. Définir le besoin utilisateur (NEED-001)
2. Définir les requis (REQ-001 à REQ-007)
3. Définir les fonctionnalités (FEAT-001 à FEAT-003)
4. Réécrire l'ADR-001 pour refléter la vraie portée du projet

## Principes directeurs (validation utilisateur)

- Synchronisation **manuelle** (pas de watch/auto-sync)
- Synchronisation d'un **seul fichier** à la fois
- **Détection** de conflits sans résolution automatique
- **Séparation contenu / style** : Markdown = contenu (plus petit dénominateur commun), Google Docs = contenu + style. Si le contenu Markdown n'a pas changé, réappliquer le style Google Docs.

## Livrables

- [x] `NEED-001` — Besoin : Synchronisation Markdown ↔ Google Docs
- [x] `NEED-002` — Besoin : Suivi de l'évolution du document via git
- [x] `REQ-001` à `REQ-009` — Requis
- [x] `FEAT-001` à `FEAT-003` — Fonctionnalités
- [x] `ADR-001` — Portée du projet (réécrit)
- [x] Restructuration du code (`agent.rs`/`tools.rs` supprimés, 6 nouveaux modules créés)
- [x] Mise à jour `Cargo.toml` (retrait `reqwest`, ajout `clap`/`pulldown-cmark`)
- [x] Mise à jour `CLAUDE.md` (architecture, stack technique)

## Documents créés

| Document | Fichier |
|---|---|
| NEED-001 | `.dev/needs/001-synchronisation-markdown-google-docs.md` |
| REQ-001 | `.dev/requirements/001-conversion-markdown-vers-gdoc.md` |
| REQ-002 | `.dev/requirements/002-conversion-gdoc-vers-markdown.md` |
| REQ-003 | `.dev/requirements/003-association-fichier-document.md` |
| REQ-004 | `.dev/requirements/004-detection-perte-information.md` |
| REQ-005 | `.dev/requirements/005-separation-contenu-style.md` |
| REQ-006 | `.dev/requirements/006-detection-conflits.md` |
| REQ-007 | `.dev/requirements/007-authentification-compte-service.md` |
| REQ-008 | `.dev/requirements/008-transport-api-google.md` |
| REQ-009 | `.dev/requirements/009-stabilite-markdown-genere.md` |
| NEED-002 | `.dev/needs/002-suivi-evolution-via-git.md` |
| FEAT-001 | `.dev/features/001-push-markdown-vers-gdoc.md` |
| FEAT-002 | `.dev/features/002-pull-gdoc-vers-markdown.md` |
| FEAT-003 | `.dev/features/003-rapport-synchronisation.md` |
