# REQ-009 — Stabilité et déterminisme du Markdown généré

**Date :** 2026-03-13  
**Statut :** Proposé  
**Type :** Fonctionnel  
**Priorité :** Important

## Description

Lors de la conversion Google Docs → Markdown (pull), le Markdown produit doit être **stable et déterministe** :

1. **Déterminisme** : le même contenu Google Docs produit toujours exactement le même Markdown, indépendamment du moment ou du contexte d'exécution.
2. **Stabilité** : un pull suivi d'un push, sans modification intermédiaire, ne doit pas générer de changements dans le Google Doc. Inversement, un push suivi d'un pull ne doit pas modifier le fichier Markdown (round-trip stability).
3. **Diffs propres** : les modifications réelles du contenu doivent produire des diffs git lisibles et limités aux zones effectivement modifiées. Pas de reformatage global, pas de réordonnancement, pas de bruit.

### Exemples de bruit à éviter

- Changement d'indentation ou d'espacement sans raison sémantique
- Réordonnancement d'attributs ou de métadonnées
- Ajout/suppression de lignes vides parasites
- Variation dans la représentation des éléments équivalents (ex: `*gras*` vs `**gras**`)

### Conventions de formatage

Le Markdown généré doit suivre des conventions fixes et documentées :
- Titres avec `#` (pas de soulignement)
- Listes non-ordonnées avec `-`
- Emphase avec `*italique*` et `**gras**`
- Un saut de ligne entre chaque bloc
- Trailing newline en fin de fichier

## Critères d'acceptation

- [ ] Deux pulls consécutifs du même document (non modifié entre-temps) produisent un Markdown identique octet par octet
- [ ] Un cycle pull → push → pull sans modification ne génère aucun diff dans le fichier Markdown
- [ ] Les diffs git entre deux versions reflètent uniquement les modifications réelles du contenu
- [ ] Les conventions de formatage du Markdown généré sont documentées et appliquées systématiquement

## Besoins associés

- `NEED-002` — Suivi de l'évolution du document via git

## Fonctionnalités associées

- `FEAT-002` — Récupération Google Doc → Markdown (pull)
