# Ticket 009 — Implémenter la commande Push (Markdown → Google Docs)

**Branche :** `impl/push-to-google`  
**Date :** 2026-03-14  
**Phase :** 4 — Vérifier l'amélioration
**Type :** Implémentation

---

## Contexte

La ronde 008 a implémenté la commande `pull` (Google Docs → Markdown). Il reste 8 `todo!()` dans le code. Cette ronde implémente la direction inverse : le **push** d'un fichier Markdown vers un Google Doc existant.

Les modules de persistance (`mapping.rs`), de transport (`google_docs.rs`) et de rendu (`markdown.rs::render()`) sont déjà fonctionnels.

## Objectif

Mettre en place la commande `nou push` : lire un fichier Markdown local, le convertir en requêtes Google Docs, et mettre à jour le document distant.

## Requis impactés

- `REQ-001` — Conversion Markdown → Google Docs (contenu)
- `REQ-003` — Association fichier ↔ document (déjà implémenté, réutilisé)
- `REQ-007` — Authentification par compte de service (déjà implémenté)
- `REQ-008` — Transport via l'API Google (déjà implémenté — `batch_update()` existe)

## Fonctionnalités impactées

- `FEAT-001` — Push Markdown → Google Doc

## todo!() à résoudre

| Module | Fonction | Description |
|---|---|---|
| `markdown.rs` | `parse()` | Parsage Markdown → `Vec<MdNode>` via pulldown-cmark |
| `converter.rs` | `markdown_to_gdoc_requests()` | Conversion `Vec<MdNode>` → requêtes `batchUpdate` |
| `sync.rs` | `push()` | Orchestration complète du push |

## Livrables proposés

### L1 — Parsage Markdown (`markdown.rs::parse()`)
- Implémenter `parse()` via `pulldown-cmark`
- Éléments supportés : titres (h1-h6), paragraphes, gras, italique, liens, listes ordonnées/non ordonnées, blocs de code, lignes horizontales
- Entrée : `&str` (contenu Markdown)
- Sortie : `Vec<MdNode>` (même représentation intermédiaire que pour le pull)

### L2 — Conversion Markdown → Google Docs (`converter.rs::markdown_to_gdoc_requests()`)
- Implémenter la conversion `Vec<MdNode>` → `Vec<Request>` (Google Docs batchUpdate)
- Stratégie : effacer tout le contenu existant, puis insérer le nouveau contenu
- Éléments : titres, paragraphes, texte formaté (gras, italique), liens, listes, blocs de code, lignes horizontales

### L3 — Orchestration du push (`sync.rs::push()`)
1. Lire le fichier Markdown local
2. Le parser via `markdown::parse()`
3. Résoudre le `doc_id` (argument ou persistance)
4. Vérification de conflit simplifiée (similaire au pull)
5. Convertir en requêtes Google Docs
6. Envoyer via `google_docs::batch_update()`
7. Sauvegarder les métadonnées de synchronisation

### L4 — CLI (`cli.rs`)
- Ajouter l'alias `-f` pour `--force` dans Push (cohérence avec Pull)
- Rendre `fichier` optionnel dans Push (cohérence avec Pull)

### L5 — Tests
- Tests unitaires : `parse()` → round-trip avec `render()` (parse puis render doit redonner le même Markdown)
- Tests unitaires : `markdown_to_gdoc_requests()` → vérifier la structure des requêtes générées
- Tests unitaires : `parse()` isolé — chaque type d'élément Markdown

## Hors périmètre (rondes ultérieures)

- Préservation du style (`style.rs`) — les 4 `todo!()` restent
- Commande `status` (`sync.rs::status()`)
- Détection de conflits bidirectionnelle complète (REQ-006)
- Création d'un nouveau Google Doc (seulement mise à jour d'un doc existant)
- Signalement des pertes d'information lors du push (REQ-004)

## Comportement attendu

- `nou push fichier.md --doc-id DOC_ID` : première fois — parse le Markdown, remplace le contenu du Google Doc, crée l'association
- `nou push` : les fois suivantes — utilise la persistance pour le fichier et le doc_id
- Si le fichier distant a été modifié depuis le dernier sync, refuser (sauf `--force`)
- L'option `-f`/`--force` écrase le contenu distant même en cas de conflit


## Questions ouvertes

1. **Stratégie de mise à jour** : effacer tout le contenu + réinsérer, ou diff incrémental ? (Proposition : effacer + réinsérer, plus simple pour une v1)
2. **Faut-il rendre `fichier` optionnel dans Push** comme dans Pull ? (Proposition : oui, cohérence)
