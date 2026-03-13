# Besoin 001 — Synchronisation Markdown ↔ Google Docs

**Date :** 2026-03-13  
**Statut :** Identifié  
**Source :** Jérémy Viau-Trudel (développeur, utilisateur principal)

## Description

En tant que développeur, je veux pouvoir rédiger et éditer mes documents en **Markdown dans un dépôt git**, tout en permettant à mes collaborateurs non-développeurs de consulter et éditer ces mêmes documents via **Google Docs**.

Les modifications faites d'un côté doivent pouvoir être synchronisées vers l'autre.

## Contexte

Deux populations d'utilisateurs avec des habitudes incompatibles :

| | Développeur | Collaborateur non-dev |
|---|---|---|
| **Outil préféré** | Éditeur de code + git | Google Docs |
| **Format préféré** | Markdown | Document riche (WYSIWYG) |
| **Force** | Versionnement, IA, automatisation | Collaboration en temps réel, accessibilité |
| **Compte** | GitHub / git | Gmail / Google Workspace |

Le Markdown est plus facile et confortable à :
- Éditer dans un IDE
- Manipuler avec l'IA (prompts, génération, transformation)
- Versionner avec git (diffs lisibles, merge, historique)

Le Google Doc est plus pratique pour :
- Partager avec des non-développeurs
- L'édition collaborative en temps réel
- La consultation sans outillage technique

**Le besoin fondamental est de concilier ces deux perspectives** sans forcer l'une des deux parties à adopter l'outil de l'autre.

## Critères de satisfaction

- [ ] Un fichier Markdown peut être publié vers un Google Doc
- [ ] Un Google Doc peut être récupéré en tant que fichier Markdown
- [ ] Les modifications faites dans le Markdown se reflètent dans le Google Doc (et inversement)
- [ ] L'utilisateur est informé lorsqu'une synchronisation implique une perte d'information (ex: formatage riche non représentable en Markdown)
- [ ] Le workflow reste naturel pour les deux parties : le développeur travaille dans git, le collaborateur dans Google Docs

## Principes directeurs

- **Synchronisation manuelle** : l'utilisateur déclenche explicitement push/pull (pas de watch/auto-sync)
- **Fichier unique** : une synchronisation porte sur un seul fichier ↔ un seul document
- **Séparation contenu / style** : le Markdown est le plus petit dénominateur commun (contenu). Le style Google Docs est une couche additionnelle préservée séparément.
- **Détection de conflits sans résolution** : le système signale les conflits mais ne tente pas de les résoudre automatiquement

## Requis associés

- `REQ-001` — Conversion Markdown → Google Docs (contenu)
- `REQ-002` — Conversion Google Docs → Markdown (contenu)
- `REQ-003` — Association fichier Markdown ↔ Google Doc
- `REQ-004` — Détection de perte d'information
- `REQ-005` — Séparation contenu / style et préservation du style Google Docs
- `REQ-006` — Détection de conflits
- `REQ-007` — Authentification par compte de service
- `REQ-008` — Transport via l'API Google (lecture/écriture distante)

## Fonctionnalités associées

- `FEAT-001` — Publication Markdown → Google Doc (push)
- `FEAT-002` — Récupération Google Doc → Markdown (pull)
- `FEAT-003` — Rapport de synchronisation
