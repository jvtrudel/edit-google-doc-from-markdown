# Ticket 008 — Implémenter la synchronisation de fichier

**Branche :** `impl/sync-file`  
**Date :** 2026-03-14  
**Phase :** 5 — Consolidation  
**Type :** Implémentation  
**Version :** v0.3.0

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

### L1 — Transport : lecture du Google Doc (`google_docs.rs`)
- `get_document()` est déjà implémenté — vérifier qu'il fonctionne avec le compte de service

### L2 — Conversion Google Docs → Markdown (`converter.rs`, `markdown.rs`)
- Implémenter `gdoc_to_markdown()` : extraire le contenu sémantique d'un `Document` en `Vec<MdNode>`
- Implémenter `render()` : générer du Markdown stable et déterministe depuis `Vec<MdNode>`
- Éléments supportés : titres (h1-h6), paragraphes, gras, italique, listes, liens, code, lignes horizontales
- Le parsage Markdown (`parse()`) reste un stub — non nécessaire pour le pull

### L3 — Persistance de l'association (`mapping.rs`)
- Implémenter `load_metadata()` / `save_metadata()` : stockage dans `.nou/state.json`
- Implémenter `set_document_id()` : créer/mettre à jour l'association
- Premier appel avec `--doc-id` crée l'association; les suivants l'utilisent

### L4 — Orchestration du pull (`sync.rs`)
- Implémenter `pull()` :
  1. Résoudre le `doc_id` (argument ou persistance)
  2. Lire le Google Doc via le transport
  3. Convertir en Markdown
  4. Vérification simplifiée : si le fichier local est plus récent que la dernière sync, refuser (sauf `--force`)
  5. Écrire le fichier Markdown
  6. Sauvegarder les métadonnées de synchronisation

### L5 — CLI (`cli.rs`, `main.rs`)
- Le fichier `<fichier.md>` devient optionnel (utilise la persistance si absent)
- Ajouter `-f` comme alias de `--force`

### L6 — Tests
- Test unitaire : conversion d'un `Document` Google Docs minimal → Markdown
- Test unitaire : `render()` produit du Markdown stable (même entrée = même sortie)
- Test unitaire : persistance (save/load round-trip)

## Hors périmètre (rondes ultérieures)

- Style Google Docs (REQ-005) — ni extraction ni réapplication
- Détection de conflits bidirectionnelle complète (REQ-006) — seule la vérification « fichier local plus récent » est faite
- Push (FEAT-001)
- Gestion multi-fichiers (REQ-011) — un seul fichier courant pour l'instant
- Signalement des pertes d'information (REQ-004) — reporté

## Comportement attendu

- Une première commande `nou pull FILE.md --doc-id DOC_ID` récupère le contenu d'un fichier google drive. Si le fichier markdow n'existe pas, le créer.
- une seconde commande `nou pull` utilise la persistance pour savoir quel fichier local et drive utiliser.
- Lors d'une seconde commande `nou pull`, importer les modifications.
- Lors d'une seconde commande `nou pull`, si la modification locale est postérieur à la modification sur drive, refuser de modifier le fichier local.
- l'option -f/--force met à jour le fichier local, même si il a été modifié après le fichier remote
