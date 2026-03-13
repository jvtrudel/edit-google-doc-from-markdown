# Plan — Ronde 007 : Changement de portée du projet

**Date :** 2026-03-13  
**Branche :** `feat/change-portee-du-projet`

---

## Résumé

La Phase 1 a produit la documentation définissant la nouvelle portée : 2 besoins, 9 requis, 3 fonctionnalités, et un ADR-001 réécrit. Le projet passe d'un « agent IA qui édite Google Docs » à un « outil CLI de synchronisation Markdown ↔ Google Docs ».

Cette phase planifie les changements nécessaires pour **aligner le code et la documentation technique** avec la nouvelle portée. L'implémentation complète des fonctionnalités (push, pull, conversion) se fera dans des rondes ultérieures.

## Analyse de l'écart

### Code actuel (738 lignes)

| Fichier | Rôle actuel | Alignement avec nouvelle portée |
|---|---|---|
| `main.rs` (67 l.) | Point d'entrée agent IA : charge clé Anthropic, crée `ClaudeAgent`, lit requête utilisateur | **À refaire** — remplacer par CLI avec sous-commandes (push, pull, status) |
| `agent.rs` (311 l.) | Boucle agentique : appels API Anthropic, gestion tool_use/tool_result | **À retirer** — plus d'agent IA |
| `tools.rs` (132 l.) | Définitions JSON des outils Claude + struct `ToolResult` | **À retirer** — plus de tool_use |
| `google_docs.rs` (228 l.) | Client Google Docs API : read, insert, delete, replace, create | **À conserver/adapter** — c'est la couche transport (REQ-008) |

### CLAUDE.md

La section « Architecture et structure du code » décrit encore l'architecture agent. Elle doit être mise à jour pour refléter la nouvelle structure modulaire.

### Cargo.toml

- **À retirer** : `reqwest` (utilisé pour l'API Anthropic uniquement)
- **À ajouter** : `pulldown-cmark` (parseur Markdown), `pulldown-cmark-to-cmark` (Markdown → texte), `clap` (CLI)

## Plan d'exécution

### Tâche 1 — Restructurer le code source

**Pourquoi :** Le code actuel est organisé autour d'un agent IA. La nouvelle architecture doit refléter les trois couches identifiées dans l'ADR-001 : transport (API), conversion (Markdown ↔ structure Google Docs), et style (préservation/réapplication).

**Nouvelle structure :**

```
src/
├── main.rs            # Point d'entrée CLI (clap) : push, pull, status
├── cli.rs             # Définition des sous-commandes et arguments
├── google_docs.rs     # Transport : lecture/écriture via API Google Docs (adapté de l'existant)
├── markdown.rs        # Parsage et génération de Markdown (pulldown-cmark)
├── converter.rs       # Conversion bidirectionnelle Markdown ↔ structure Google Docs
├── style.rs           # Extraction, sauvegarde et réapplication du style Google Docs
├── sync.rs            # Logique de synchronisation : push, pull, détection de conflits
├── mapping.rs         # Association fichier ↔ document, métadonnées de sync
└── error.rs           # Types d'erreur custom (thiserror)
```

**Actions :**
- Supprimer `agent.rs` et `tools.rs`
- Adapter `google_docs.rs` (retirer les opérations agent-spécifiques, garder le transport)
- Créer les nouveaux modules avec des stubs (interfaces publiques, types, `todo!()` pour les implémentations)
- Réécrire `main.rs` avec clap et les sous-commandes

**Dépendances :** Aucune

### Tâche 2 — Mettre à jour Cargo.toml

**Pourquoi :** Les dépendances doivent refléter la nouvelle architecture.

**Actions :**
- Retirer `reqwest` (plus d'appels API Anthropic)
- Ajouter `pulldown-cmark` (parseur Markdown)
- Ajouter `clap` avec `derive` (CLI)
- Conserver : `google-docs1`, `yup-oauth2`, `hyper`, `hyper-rustls`, `serde`, `serde_json`, `tokio`, `anyhow`, `thiserror`, `tracing`, `tracing-subscriber`, `dotenvy`

**Dépendances :** Aucune (peut être fait en parallèle avec Tâche 1)

### Tâche 3 — Mettre à jour CLAUDE.md

**Pourquoi :** La section « Architecture et structure du code » et la section « Stack technique » décrivent encore l'architecture agent. Elles doivent refléter la nouvelle structure.

**Actions :**
- Mettre à jour la section « Architecture et structure du code » avec la nouvelle arborescence
- Mettre à jour la section « Stack technique » (retirer reqwest/Anthropic, ajouter pulldown-cmark/clap)
- Mettre à jour la description du projet dans l'introduction

**Dépendances :** Tâche 1 (pour connaître la structure finale)

### Tâche 4 — Vérifier la compilation

**Pourquoi :** Le code doit compiler après la restructuration, même si les implémentations sont des stubs.

**Actions :**
- `cargo check` pour vérifier la compilation
- `cargo clippy` pour vérifier les avertissements
- Corriger les erreurs éventuelles

**Dépendances :** Tâches 1, 2

### Tâche 5 — Mettre à jour le ticket

**Pourquoi :** Le ticket doit refléter la phase courante et les livrables de cette ronde.

**Actions :**
- Passer la phase à « 2 → 3 » quand le plan est validé
- Ajouter les livrables d'implémentation

**Dépendances :** Validation du plan par l'utilisateur

## Ordre d'exécution

```
Tâche 2 (Cargo.toml)
    ↓
Tâche 1 (Restructurer le code)
    ↓
Tâche 4 (Vérifier compilation)
    ↓
Tâche 3 (CLAUDE.md)
    ↓
Tâche 5 (Ticket)
```

## Hors périmètre de cette ronde

Les éléments suivants seront implémentés dans des **rondes ultérieures** :

| Ronde future | Contenu |
|---|---|
| 008 (suggestion) | Implémentation de la conversion Markdown → structure Google Docs (REQ-001) et structure Google Docs → Markdown (REQ-002) |
| 009 (suggestion) | Implémentation du push (FEAT-001) et du pull (FEAT-002) avec transport API |
| 010 (suggestion) | Style : extraction, sauvegarde et réapplication (REQ-005) |
| 011 (suggestion) | Conflits : détection et rapport (REQ-006, FEAT-003) |

## Risques

- **Crate `pulldown-cmark`** : vérifier la compatibilité avec l'édition Rust 2024
- **Suppression de `agent.rs`** : perte du code agent. Si l'agent redevient pertinent plus tard, il faudrait le recréer. Mitigation : le code est dans l'historique git.
- **Stubs avec `todo!()`** : le code compile mais panique à l'exécution. C'est acceptable pour cette ronde (restructuration).
