# REQ-012 — Mode silencieux global

**Date :** 2026-03-14  
**Statut :** Implémenté  
**Type :** Non fonctionnel  
**Priorité :** Souhaitable

## Description

L'outil offre deux modes d'affichage contrôlés par l'option globale `-s`/`--silent` :

- **Mode par défaut (verbeux)** : des commentaires détaillés sont affichés lors de l'exécution (informations de progression, avertissements, détails des opérations).
- **Mode silencieux (`-s`/`--silent`)** : seuls les messages d'erreur sont affichés. Aucun commentaire informatif ou de progression n'est émis.

Ce mode est **global** : il s'applique à toutes les sous-commandes (`push`, `pull`, `status`, etc.) sans avoir à le redéfinir pour chacune.

## Critères d'acceptation

- [ ] Par défaut, l'exécution d'une commande affiche des commentaires détaillés (niveau debug/info/warn)
- [ ] L'option `-s` ou `--silent` est disponible pour toutes les sous-commandes
- [ ] En mode silencieux, seuls les messages d'erreur sont affichés
- [ ] L'option peut être placée avant ou après la sous-commande (`nou -s pull` ou `nou pull -s`)

## Besoins associés

_Aucun besoin spécifique — amélioration de l'expérience utilisateur._

## Fonctionnalités associées

_Transversal — s'applique à toutes les fonctionnalités CLI._
