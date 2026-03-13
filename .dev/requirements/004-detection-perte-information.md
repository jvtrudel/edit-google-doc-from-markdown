# REQ-004 — Détection de perte d'information

**Date :** 2026-03-13  
**Statut :** Proposé  
**Type :** Fonctionnel  
**Priorité :** Important

## Description

Lors d'une conversion (dans les deux sens), le système doit détecter et signaler les éléments qui ne peuvent pas être fidèlement représentés dans le format cible.

Exemples de perte d'information :
- **Google Docs → Markdown** : images intégrées, commentaires, suggestions, tableaux complexes, couleurs de texte, polices personnalisées
- **Markdown → Google Docs** : aucune perte structurelle attendue (Markdown est le dénominateur commun)

Le système **signale** les pertes mais ne tente pas de les résoudre.

## Critères d'acceptation

- [ ] Lors d'un pull, les éléments Google Docs non convertibles sont listés avec leur type et leur position approximative
- [ ] Le rapport est affiché à l'utilisateur sur la sortie standard
- [ ] Le rapport distingue les pertes de contenu (information manquante) des pertes de style (présentation différente)

## Besoins associés

- `NEED-001` — Synchronisation Markdown ↔ Google Docs

## Fonctionnalités associées

- `FEAT-001` — Publication Markdown → Google Doc
- `FEAT-002` — Récupération Google Doc → Markdown
