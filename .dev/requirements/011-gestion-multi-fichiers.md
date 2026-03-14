# REQ-011 — Gestion multi-fichiers et sélection du fichier de travail

**Date :** 2026-03-14  
**Statut :** Proposé  
**Type :** Fonctionnel  
**Priorité :** Important

## Description

Le système doit pouvoir conserver en mémoire **plusieurs associations** fichier Markdown ↔ Google Doc et permettre à l'utilisateur de **changer de fichier de travail** sans perdre les associations existantes.

Chaque association enregistrée contient : le chemin du fichier Markdown, l'identifiant du Google Doc, et les métadonnées de synchronisation (horodatages, hash, etc.).

### Comportement attendu

- L'utilisateur peut enregistrer plusieurs fichiers (via `push` ou `pull` avec argument)
- Un seul fichier est le **fichier courant** à un instant donné (voir REQ-010)
- L'utilisateur peut changer de fichier courant avec une commande dédiée (ex: `nou use <fichier.md>`)
- La liste des fichiers enregistrés peut être consultée (ex: `nou list`)
- Un fichier peut être retiré de la liste (`nou unlink <fichier.md>`)

### Stockage

Les associations sont stockées dans un fichier de configuration local (ex: `.nou/config.json`) qui contient :
- La liste de toutes les associations connues
- L'identifiant du fichier courant

## Critères d'acceptation

- [ ] Plusieurs fichiers peuvent être enregistrés simultanément
- [ ] Chaque fichier conserve ses propres métadonnées de synchronisation
- [ ] L'utilisateur peut lister tous les fichiers enregistrés
- [ ] L'utilisateur peut changer de fichier courant (`nou use <fichier.md>`)
- [ ] L'utilisateur peut dissocier un fichier (`nou unlink <fichier.md>`)
- [ ] Les opérations sur un fichier non-courant restent possibles en passant le fichier en argument

## Besoins associés

- `NEED-001` — Synchronisation Markdown ↔ Google Docs

## Fonctionnalités associées

- `FEAT-001` — Publication Markdown → Google Doc (push)
- `FEAT-002` — Récupération Google Doc → Markdown (pull)
- `FEAT-003` — Rapport de synchronisation
