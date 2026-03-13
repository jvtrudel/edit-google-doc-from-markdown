# Changelog

## v0.2.0 — 2026-03-13

### Changement de portée

Le projet passe d'un « agent IA qui édite Google Docs » à un **outil CLI de synchronisation Markdown ↔ Google Docs**. Le Markdown est le plus petit dénominateur commun (contenu), le style Google Docs est préservé séparément.

### Ajouté
- **Documentation de portée complète** :
  - `NEED-001` — Synchronisation Markdown ↔ Google Docs
  - `NEED-002` — Suivi de l'évolution du document via git
  - `REQ-001` à `REQ-009` — 9 requis (conversion, association, conflits, style, transport, stabilité)
  - `FEAT-001` à `FEAT-003` — 3 fonctionnalités (push, pull, rapport de synchronisation)
- **Nouveaux modules Rust** (stubs) :
  - `cli.rs` — Sous-commandes clap (push, pull, status)
  - `converter.rs` — Conversion bidirectionnelle Markdown ↔ Google Docs
  - `markdown.rs` — Parsage et génération Markdown (pulldown-cmark)
  - `style.rs` — Extraction et réapplication du style Google Docs
  - `sync.rs` — Logique de synchronisation (push, pull, conflits)
  - `mapping.rs` — Association fichier ↔ document et métadonnées
  - `error.rs` — Types d'erreur custom (thiserror)
- **Nouvelles dépendances** : `clap` v4 (CLI), `pulldown-cmark` v0.12 (parseur Markdown)

### Modifié
- `ADR-001` réécrit : nouvelle portée (synchronisation, pas agent IA)
- `google_docs.rs` adapté : 3 méthodes transport (`get_document`, `batch_update`, `create_document`)
- `main.rs` réécrit : CLI clap avec sous-commandes push/pull/status
- `CLAUDE.md` mis à jour : stack technique, architecture, commandes CLI
- `Cargo.toml` : retrait de `reqwest`, ajout de `clap` et `pulldown-cmark`

### Supprimé
- `agent.rs` — Boucle agentique Anthropic (311 lignes)
- `tools.rs` — Définitions d'outils Claude (132 lignes)
- Dépendance `reqwest` (client HTTP pour API Anthropic)

## v0.1.0 — 2026-03-13

### Ajouté
- Méthodologie de développement assisté par IA en 6 phases (CLAUDE.md)
- Conventions Git : branches (type/description), conventional commits, semantic versioning
- 10 commandes slash pour VS Code Copilot Chat :
  /current, /eval-ticket, /init-enhancement, /ready-to-advance,
  /plan, /implement, /verify, /consolidate, /publish, /log
- Prototype CLI `nou` (script shell) avec commandes :
  help, status, run, dev init/ticket/phase/log, doc needs/requirements/features
- Script d'activation `activate.sh`
- Structure documentaire `.dev/` avec README.md et _template.md
  pour : needs, requirements, features, specs
- ADR-001 : portée du projet
- 2 analyses : requirement-spec-feature, slash-commands-et-skills

### Modifié
- CLAUDE.md entièrement réécrit et structuré
- Historique déplacé de `.dev/prompt-history/` vers `.dev/history/`
- TODO.md déplacé de `.dev/adr/` vers `.dev/`
