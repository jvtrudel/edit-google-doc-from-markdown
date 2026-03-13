# Requis

Ce dossier contient les requis du système — des définitions atomiques de comment le système doit se comporter.

## Rôle

Un **requis** est une contrainte précise, testable et non ambiguë que le système doit satisfaire. Il décrit le « quoi » technique.

## Convention de nommage

`NNN-description-courte.md` (ex: `001-lire-contenu-google-doc.md`)

## Format

Voir `_template.md` pour le format attendu.

## Types de requis

- **Fonctionnel** : ce que le système doit faire
- **Non fonctionnel** : comment le système doit se comporter (performance, sécurité, etc.)

## Liens

- Un requis répond à un ou plusieurs **besoins** (`.dev/needs/`)
- Un requis est réalisé par une ou plusieurs **fonctionnalités** (`.dev/features/`)
- Un requis peut être vérifié via une **spécification** (`.dev/specs/`)
