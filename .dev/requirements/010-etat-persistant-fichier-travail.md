# REQ-010 — État persistant du fichier de travail

**Date :** 2026-03-14  
**Statut :** Proposé  
**Type :** Fonctionnel  
**Priorité :** Important

## Description

Le système doit maintenir un **état persistant** qui garde en mémoire le fichier Markdown sur lequel l'utilisateur travaille. Cet état permet d'éviter de spécifier le chemin du fichier à chaque commande.

Une fois qu'un fichier est sélectionné (via une commande `push`, `pull` ou une commande dédiée), le système le retient comme fichier courant. Les commandes suivantes (`status`, `push`, `pull`) peuvent alors être invoquées sans argument de fichier.

### Comportement attendu

- L'état est stocké localement (fichier `.nou/state.json` ou similaire)
- L'état contient au minimum : le chemin du fichier Markdown courant et l'identifiant du Google Doc associé
- Si aucun fichier n'est sélectionné et qu'aucun argument n'est fourni, le système affiche un message d'erreur clair
- L'état peut être consulté via `nou status` (sans argument)
- L'état peut être modifié explicitement ou implicitement (lors d'un push/pull avec argument)

## Critères d'acceptation

- [ ] Après un `nou push fichier.md`, le fichier `fichier.md` est retenu comme fichier courant
- [ ] Un `nou status` sans argument affiche les informations du fichier courant
- [ ] Un `nou pull` sans argument récupère le Google Doc associé au fichier courant
- [ ] L'état persiste entre les exécutions du programme
- [ ] L'état peut être réinitialisé ou changé en spécifiant un autre fichier

## Besoins associés

- `NEED-001` — Synchronisation Markdown ↔ Google Docs

## Fonctionnalités associées

- `FEAT-001` — Publication Markdown → Google Doc (push)
- `FEAT-002` — Récupération Google Doc → Markdown (pull)
- `FEAT-003` — Rapport de synchronisation
