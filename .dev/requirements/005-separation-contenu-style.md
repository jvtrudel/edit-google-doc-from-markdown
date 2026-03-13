# REQ-005 — Séparation contenu / style et préservation du style Google Docs

**Date :** 2026-03-13  
**Statut :** Proposé  
**Type :** Fonctionnel  
**Priorité :** Important

## Description

Le système doit séparer le **contenu** (texte, structure sémantique) du **contenant** (style visuel : polices, couleurs, tailles, marges, etc.).

Le Markdown représente le **contenu** — c'est le plus petit dénominateur commun.

Le style Google Docs est une **couche additionnelle** au-dessus du contenu. Le système doit :

1. **Lors d'un pull** : extraire et sauvegarder les informations de style Google Docs séparément du contenu Markdown.
2. **Lors d'un push** : si le contenu Markdown n'a pas changé par rapport à la dernière synchronisation, **réappliquer le style Google Docs** tel qu'il était. Si le contenu a changé, appliquer le style par défaut aux parties modifiées et conserver le style des parties inchangées.

Ce principe garantit que les collaborateurs non-dev peuvent personnaliser le style dans Google Docs sans que celui-ci soit perdu à chaque synchronisation.

## Critères d'acceptation

- [ ] Les informations de style Google Docs sont sauvegardées lors d'un pull
- [ ] Un push d'un Markdown non modifié préserve intégralement le style du Google Doc
- [ ] Un push d'un Markdown modifié conserve le style des parties inchangées
- [ ] Les nouvelles parties (ajoutées dans le Markdown) reçoivent le style par défaut du document

## Besoins associés

- `NEED-001` — Synchronisation Markdown ↔ Google Docs

## Fonctionnalités associées

- `FEAT-001` — Publication Markdown → Google Doc
- `FEAT-002` — Récupération Google Doc → Markdown
