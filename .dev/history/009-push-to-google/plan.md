# Plan détaillé — Ronde 009 : Implémenter la commande Push (Markdown → Google Docs)

**Branche :** `impl/push-to-google`
**Date :** 2026-03-14
**Phase :** 2 — Planification

---

## Contexte et justification

La synchronisation bidirectionnelle Markdown ↔ Google Docs répond au besoin de concilier les workflows des développeurs (Markdown, git) et des collaborateurs non-dev (Google Docs). Après l'implémentation de la commande `pull`, il s'agit d'ajouter la commande `push` pour permettre la publication du contenu Markdown vers un Google Doc existant.

La portée respecte l'ADR-001 :
- Synchronisation manuelle, un fichier à la fois
- Détection de conflits (refus en cas de modification simultanée, sauf --force)
- Conversion du contenu (pas du style)
- Authentification par compte de service
- Interface CLI

---

## Décomposition en tâches

### Tâche 1 : Parsage Markdown (`markdown.rs::parse()`)
- Implémenter la fonction `parse()` via pulldown-cmark
- Supporter : titres (h1-h6), paragraphes, gras, italique, liens, listes ordonnées/non ordonnées, blocs de code, lignes horizontales
- Entrée : `&str` (contenu Markdown)
- Sortie : `Vec<MdNode>` (représentation intermédiaire)

### Tâche 2 : Conversion Markdown → Google Docs (`converter.rs::markdown_to_gdoc_requests()`)
- Implémenter la conversion `Vec<MdNode>` → `Vec<Request>` (Google Docs batchUpdate)
- Stratégie : effacer tout le contenu existant, puis insérer le nouveau contenu
- Supporter les mêmes éléments que pour le parsage

### Tâche 3 : Orchestration du push (`sync.rs::push()`)
- Lire le fichier Markdown local
- Parser via `markdown::parse()`
- Résoudre le `doc_id` (argument ou persistance)
- Vérification de conflit simplifiée (refus si modifié, sauf --force)
- Conversion en requêtes Google Docs
- Envoi via `google_docs::batch_update()`
- Sauvegarde des métadonnées de synchronisation

### Tâche 4 : CLI (`cli.rs`)
- Ajouter l'alias `-f` pour `--force` dans Push
- Rendre l'argument `fichier` optionnel dans Push

### Tâche 5 : Tests
- Tests unitaires : round-trip `parse()` ↔ `render()`
- Tests unitaires : structure des requêtes générées par `markdown_to_gdoc_requests()`
- Tests unitaires : `parse()` isolé pour chaque type d'élément

---

## Ordre d'exécution et dépendances

1. Implémenter `parse()` (Tâche 1)
2. Implémenter `markdown_to_gdoc_requests()` (Tâche 2)
3. Implémenter l'orchestration `push()` (Tâche 3)
4. Adapter la CLI (Tâche 4)
5. Écrire et exécuter les tests unitaires (Tâche 5)

- Tâche 1 et 2 sont indépendantes, mais Tâche 2 dépend du format produit par Tâche 1
- Tâche 3 dépend de Tâche 1 et 2
- Tâche 4 peut être réalisée en parallèle
- Tâche 5 dépend de Tâche 1 et 2

---

## Hors périmètre

- Préservation du style (`style.rs`) — à traiter dans une ronde ultérieure
- Commande `status` (`sync.rs::status()`)
- Détection de conflits bidirectionnelle complète
- Création d'un nouveau Google Doc
- Signalement des pertes d'information lors du push

---

## Validation

Ce plan respecte l'ADR-001, les requis et le ticket de la ronde. Il permet une implémentation cohérente, modulaire et testable de la commande `push`.

---

## Questions ouvertes

1. Stratégie de mise à jour : effacer + réinsérer (proposé pour v1)
2. Argument `fichier` optionnel dans Push (proposé pour cohérence)

---

**À valider par l'utilisateur avant implémentation.**
