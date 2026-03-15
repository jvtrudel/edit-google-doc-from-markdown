# REQ-014 — Stratégie de mise à jour du document

**Date :** 2026-03-14  
**Statut :** Proposé  
**Type :** Fonctionnel  
**Priorité :** Important

## Description

Lors d'un push (Markdown → Google Doc) ou d'un pull (Google Doc → Markdown), le système doit appliquer une **stratégie de mise à jour** pour modifier le contenu de la destination.

### Stratégies définies

| Stratégie | Comportement | Avantages | Inconvénients |
|---|---|---|---|
| `rewrite` **(défaut)** | Effacer tout le contenu de la destination, puis insérer le nouveau contenu intégralement | Simple, déterministe, aucun artefact résiduel | Perd le style existant, recrée tout à chaque fois |
| `incremental` | Comparer l'ancien et le nouveau contenu, modifier uniquement les parties qui ont changé | Préserve le style des parties non modifiées, plus efficace en réseau | Complexe, risque d'artefacts, nécessite un diff structurel |

### Ordre de priorité

1. Option CLI (`--strategy rewrite|incremental`) — priorité maximale
2. Variable d'environnement (`NOU_UPDATE_STRATEGY`)
3. Valeur par défaut : `rewrite`

### Notes

- La stratégie `rewrite` est suffisante tant que la préservation du style (REQ-005) n'est pas implémentée.
- La stratégie `incremental` deviendra nécessaire quand le style sera pris en compte, car une réécriture complète détruirait le formatage appliqué par les collaborateurs dans Google Docs.
- Pour le pull, `rewrite` signifie simplement remplacer le fichier Markdown local (comportement actuel).

## Critères d'acceptation

- [ ] La stratégie par défaut est `rewrite`
- [ ] En mode `rewrite` pour un push : tout le contenu du Google Doc est supprimé puis réinséré
- [ ] En mode `rewrite` pour un pull : le fichier Markdown local est entièrement remplacé
- [ ] L'option `--strategy` est documentée dans l'aide CLI
- [ ] La stratégie `incremental` est reconnue mais peut retourner une erreur « non implémenté » en attendant

## Besoins associés

- `NEED-001` — Synchronisation Markdown ↔ Google Docs

## Fonctionnalités associées

- `FEAT-001` — Publication Markdown → Google Doc
- `FEAT-002` — Récupération Google Doc → Markdown
