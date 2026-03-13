# REQ-002 — Conversion Google Docs → Markdown (contenu)

**Date :** 2026-03-13  
**Statut :** Proposé  
**Type :** Fonctionnel  
**Priorité :** Critique

## Description

Le système doit extraire le **contenu** d'un Google Doc et le convertir en Markdown valide. Seul le contenu sémantique est extrait (structure, texte, emphase). Le style visuel spécifique à Google Docs (polices, couleurs, marges) n'est pas converti en Markdown — il est préservé séparément (voir REQ-005).

Le Markdown est le **plus petit dénominateur commun** : il représente le contenu sans perte sémantique, mais sans le style visuel propre à Google Docs.

## Critères d'acceptation

- [ ] Un Google Doc produit un fichier Markdown valide et lisible
- [ ] La structure sémantique est préservée : titres, paragraphes, gras, italique, listes, liens, code
- [ ] La conversion est déterministe : le même document produit toujours le même Markdown
- [ ] Les éléments Google Docs sans équivalent Markdown sont signalés (voir REQ-004)

## Besoins associés

- `NEED-001` — Synchronisation Markdown ↔ Google Docs

## Fonctionnalités associées

- `FEAT-002` — Récupération Google Doc → Markdown
