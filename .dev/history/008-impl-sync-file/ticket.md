# Ticket 008 — Implémenter la synchronisation de fichier

**Branche :** `impl/sync-file`  
**Date :** 2026-03-14  
**Phase :** 1 — Définir la portée de l'amélioration  
**Type :** Implémentation

---

## Contexte

La ronde 007 a restructuré le projet et créé l'architecture modulaire avec 8 modules Rust (stubs `todo!()`). Il faut maintenant implémenter les fonctionnalités : le push d'un fichier Markdown vers un Google Doc, le pull d'un Google Doc vers un fichier Markdown, et le status.

14 `todo!()` sont présents dans le code :
- `sync.rs` (3) : push, pull, status
- `converter.rs` (2) : Markdown → Google Docs, Google Docs → Markdown
- `markdown.rs` (2) : parse, render
- `style.rs` (4) : extract, save, load, reapply
- `mapping.rs` (3) : load, save, set

## Objectif

Mettre en place la récupération d'un fichier Markdown à partir d'un fichier Google Doc.

- Implémenter uniquement la Commande Pull


## Requis impactés

- `REQ-002` — Conversion Google Docs → Markdown (contenu)
- `REQ-007` — Authentification par compte de service
- `REQ-008` — Transport via l'API Google
- `REQ-009` — Stabilité du Markdown généré
- `REQ-010` — État persistant du fichier de travail

Pour l'instant, on ne gère pas le style. Seulement le contenu.
Pour l'instant, ne pas faire de gestion de conflit complète.

## Fonctionnalités impactées

- `FEAT-002` — Pull Google Doc → Markdown
- `FEAT-003` — Rapport de synchronisation

## Livrables

  - commande pull implémentée
  - implémenter les tests nécessaires

## Comportement attendu

- Une première commande `nou pull FILE.md --doc-id DOC_ID` récupère le contenu d'un fichier google drive. Si le fichier markdow n'existe pas, le créer.
- une seconde commande `nou pull` utilise la persistance pour savoir quel fichier local et drive utiliser.
- Lors d'une seconde commande `nou pull`, importer les modifications.
- Lors d'une seconde commande `nou pull`, si la modification locale est postérieur à la modification sur drive, refuser de modifier le fichier local.
- l'option -f/--force met à jour le fichier local, même si il a été modifié après le fichier remote
