# Spécifications

Ce dossier contient les spécifications détaillées permettant de valider qu'une implémentation est conforme à ce qui est attendu.

## Rôle

Une **spécification** est un document détaillé qui décrit *comment* vérifier qu'une fonctionnalité ou un requis est correctement implémenté. C'est le contrat entre la conception et l'implémentation.

## Convention de nommage

`NNN-description-courte.md` (ex: `001-spec-insertion-texte.md`)

## Format

Voir `_template.md` pour le format attendu.

## Quand créer une spécification

- Quand une fonctionnalité est suffisamment complexe pour nécessiter des cas de test détaillés
- Quand le comportement attendu est ambigu sans description formelle
- Quand on veut pouvoir regénérer une implémentation équivalente à partir de la documentation

## Liens

- Une spécification vérifie un ou plusieurs **requis** (`.dev/requirements/`)
- Une spécification détaille une **fonctionnalité** (`.dev/features/`)
