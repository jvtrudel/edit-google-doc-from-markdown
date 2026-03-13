# REQ-001 — Conversion Markdown → Google Docs (contenu)

**Date :** 2026-03-13  
**Statut :** Proposé  
**Type :** Fonctionnel  
**Priorité :** Critique

## Description

Le système doit convertir le **contenu** d'un fichier Markdown en contenu Google Docs en préservant la structure sémantique : titres (h1-h6), paragraphes, gras, italique, listes (ordonnées/non-ordonnées), liens, blocs de code, lignes horizontales.

La conversion porte sur le **contenu** uniquement. Le style visuel (couleurs, polices, tailles personnalisées) n'est pas du ressort de cette conversion — il est géré séparément (voir REQ-005).

## Critères d'acceptation

- [ ] Un fichier Markdown valide produit un contenu Google Docs structurellement équivalent
- [ ] Les éléments Markdown suivants sont convertis : titres, paragraphes, gras, italique, listes ordonnées, listes non-ordonnées, liens, blocs de code, lignes horizontales
- [ ] La conversion est déterministe : le même Markdown produit toujours le même contenu

## Besoins associés

- `NEED-001` — Synchronisation Markdown ↔ Google Docs

## Fonctionnalités associées

- `FEAT-001` — Publication Markdown → Google Doc
