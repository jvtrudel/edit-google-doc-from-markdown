# FEAT-003 — Rapport de synchronisation

**Date :** 2026-03-13  
**Statut :** Proposée  
**Priorité :** Important

## Description

Après chaque opération de push ou pull, le système affiche un rapport clair sur la sortie standard indiquant :

- Le résultat de l'opération (succès, conflit, erreur)
- Les éléments de contenu perdus ou non convertibles (le cas échéant)
- L'état de préservation du style (style réappliqué, style par défaut, etc.)
- Les horodatages de synchronisation

## Commande CLI

Le rapport est affiché automatiquement lors des commandes `push` et `pull`. Une commande dédiée permet de consulter l'état actuel :

```bash
nou status <fichier.md>
```

Affiche :
- L'association fichier ↔ document
- Les dates de dernière modification locale et distante
- L'état de synchronisation (synchronisé, modifié localement, modifié à distance, conflit)

## Requis implémentés

- `REQ-004` — Détection de perte d'information
- `REQ-006` — Détection de conflits

## Besoins associés

- `NEED-001` — Synchronisation Markdown ↔ Google Docs
