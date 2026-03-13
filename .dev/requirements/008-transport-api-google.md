# REQ-008 — Transport via l'API Google (lecture/écriture distante)

**Date :** 2026-03-13  
**Statut :** Proposé  
**Type :** Fonctionnel  
**Priorité :** Critique

## Description

Le système doit assurer le **transport** entre un fichier Markdown local et un Google Doc distant via l'API Google. Cela implique deux opérations fondamentales :

1. **Lecture distante** : récupérer le contenu et le style d'un Google Doc via l'API Google Docs (endpoint `documents.get`), incluant la structure complète du document (éléments, styles inline, styles de paragraphe).
2. **Écriture distante** : appliquer des modifications à un Google Doc via l'API Google Docs (endpoint `documents.batchUpdate`), en envoyant des requêtes structurées (insertions, suppressions, mises à jour de style).

Le transport est distinct de la conversion : il s'occupe uniquement de **lire et écrire** le document distant, pas de transformer le contenu entre les formats.

### Contraintes de l'API Google Docs

- L'API Google Docs opère par **requêtes batch** (`batchUpdate`) et non par remplacement complet du document.
- Les opérations sont **positionnelles** (basées sur des index de caractères dans le document).
- L'ordre des requêtes dans un batch est significatif (les index changent après chaque opération).
- Les quotas et limites de l'API doivent être respectés (rate limiting).

### Fichiers locaux

- Le fichier Markdown est lu et écrit sur le système de fichiers local.
- Les métadonnées de synchronisation (association, horodatages, style sauvegardé) sont stockées localement dans un format à définir.

## Critères d'acceptation

- [ ] Le système peut lire le contenu complet d'un Google Doc via l'API (structure, texte, styles)
- [ ] Le système peut écrire/modifier un Google Doc via `batchUpdate`
- [ ] Le système peut créer un nouveau Google Doc via l'API si aucun document cible n'existe
- [ ] Le système gère les erreurs réseau et les erreurs API (401, 403, 404, 429) avec des messages clairs
- [ ] Le système respecte les quotas de l'API (retry avec backoff exponentiel sur 429)
- [ ] Les fichiers locaux (Markdown, métadonnées) sont lus et écrits de manière atomique (pas de corruption en cas d'erreur)

## Besoins associés

- `NEED-001` — Synchronisation Markdown ↔ Google Docs

## Fonctionnalités associées

- `FEAT-001` — Publication Markdown → Google Doc (push)
- `FEAT-002` — Récupération Google Doc → Markdown (pull)
