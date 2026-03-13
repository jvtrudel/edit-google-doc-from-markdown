# CLAUDE.md

Instructions pour les agents IA travaillant sur ce projet.

## Projet

Les décisions architecturales sont documentées dans `.dev/adr/` au format `NNN-description.md`.

L'`ADR-001` (`001-portee-du-projet.md`) décrit et explique le problème que tente de résoudre ce projet.

Avant toute chose, vérifier que ce qui est demandé et ce qui est produit respecte l'`ADR-001`.

Soulever les incohérences et les ambiguités entre les demandes et tous les ADR. En cas de problème ou d'incertitude, demander des clarifications et refuser de travailler.


## Méthodologie de développement

Le développement est assisté par IA et se fait par **ronde d'amélioration** — une séquence de prompts permettant de livrer un changement cohérent (nouvelle fonctionnalité, refactorisation, débogage, documentation, ajout de tests, amélioration de la sécurité, etc.).

Chaque ronde est décomposée en **6 phases**. Une documentation structurée est produite à chaque étape.

### Structure de documentation `.dev/`

Le répertoire `.dev/` contient toute la documentation du projet :

| Dossier | Rôle | Format |
|---|---|---|
| `.dev/adr/` | Décisions architecturales importantes | `NNN-description.md` |
| `.dev/analyses/` | Information produite pour aider le développeur (valeur informative) | `NNN-description.md` |
| `.dev/history/` | Historique structuré des rondes d'amélioration | `NNN-branche/ticket.md` |
| `.dev/needs/` | Besoins utilisateurs (le « pourquoi ») | `NNN-description.md` |
| `.dev/requirements/` | Requis : définition atomique du comportement attendu | `NNN-description.md` |
| `.dev/features/` | Fonctionnalités : capacités fournies à l'utilisateur | `NNN-description.md` |
| `.dev/specs/` | Spécifications : documentation détaillée pour valider la conformité | `NNN-description.md` |

Chaque dossier contient un `README.md` (rôle et conventions) et un `_template.md` (modèle de document).

### Cycle de développement

#### Phase 1 — Définir la portée de l'amélioration

L'humain explique à l'IA ce qui doit être fait. L'IA pose des questions et refuse de travailler si c'est ambigu.

- Chaque ronde se déroule dans une **branche git** dédiée.
- Un répertoire de travail `.dev/history/NNN-branche/` est créé avec un sous-dossier `log/`.
- Le travail est résumé dans un **ticket** (`.dev/history/NNN-branche/ticket.md`).
- On doit déterminer :
  - De quel type d'amélioration s'agit-il ?
  - Quels besoins utilisateurs sont impactés ?
  - Y a-t-il des requis à ajouter ou modifier ?
  - Quelles fonctionnalités sont impactées et comment ?
  - Y a-t-il lieu de modifier une spécification ?
- Le travail ne doit pas impacter trop de code ou trop de fonctionnalités. Proposer de scinder en plusieurs rondes si nécessaire.

**Transition :** l'IA décide quand la portée est suffisamment définie.

**Commande slash :** `/init-enhancement` pour initialiser la ronde, `/eval-ticket` pour évaluer le ticket.

#### Phase 2 — Planifier l'amélioration

L'IA produit un plan détaillé dans `.dev/history/NNN-branche/plan.md` qui explique ce qui sera fait, pourquoi, et dans quel ordre.

L'humain apporte des précisions si nécessaire.

**Transition :** l'humain décide quand le plan est acceptable.

**Commande slash :** `/plan` pour générer le plan, `/ready-to-advance` pour vérifier.

#### Phase 3 — Implémenter l'amélioration

L'IA implémente ce qui a été planifié, étape par étape.

**Commande slash :** `/implement` pour lancer l'implémentation.

#### Phase 4 — Vérifier l'amélioration

L'IA explique à l'humain comment vérifier les changements. L'humain vérifie et valide ou demande des corrections.

**Commande slash :** `/verify` pour générer les instructions de vérification.

#### Phase 5 — Consolider l'amélioration

Optionnellement : améliorer la documentation, restructurer le code, ajouter des tests.

Obligatoirement :
- Définir le numéro de la prochaine version (semantic versioning)
- Produire un changelog
- Produire un release note

**Commande slash :** `/consolidate` pour lancer la consolidation.

#### Phase 6 — Publier l'amélioration

- Merger la branche dans `main`
- Appliquer le tag de version
- Pousser le code et la nouvelle version

**Commande slash :** `/publish` pour les instructions de publication.

### Conventions Git

#### Branches

Format : `type/description-courte`

Types autorisés :
- `feat/` — nouvelle fonctionnalité
- `fix/` — correction de bug
- `refactor/` — refactorisation sans changement de comportement
- `docs/` — documentation uniquement
- `chore/` — maintenance, outillage, CI
- `test/` — ajout ou modification de tests

#### Messages de commit

Format **Conventional Commits** :

```
type(scope): description courte

Corps optionnel expliquant le pourquoi.
```

Types : `feat`, `fix`, `refactor`, `docs`, `chore`, `test`  
Scope : optionnel, indique le module impacté (ex: `agent`, `tools`, `google_docs`)  
Langue : **français**

Exemples :
```
feat(tools): ajouter l'outil de remplacement de texte
fix(agent): corriger la boucle infinie quand Claude ne retourne pas end_turn
docs: mettre à jour le cycle de développement dans CLAUDE.md
```

#### Versionnement

**Semantic Versioning** (`vMAJEUR.MINEUR.PATCH`) :
- `MAJEUR` : changements incompatibles
- `MINEUR` : nouvelles fonctionnalités rétrocompatibles
- `PATCH` : corrections de bugs

### Déroulement du développement

- Créer une nouvelle branche à partir de `main` pour initialiser une ronde d'amélioration.
- Identifier la phase de développement. Si c'est ambigu, demander au développeur.
- Travailler à résoudre chaque phase en dialoguant humain-IA.
- Un commit peut signaler la fin d'une phase (optionnel).
- Un merge dans `main` signale la fin d'une ronde d'amélioration.

### Historique des prompts

Les résumés des interactions avec l'agent sont conservés dans `.dev/history/NNN-branche/log/MMM-description.md`.

À chaque prompt, écrire un résumé de ce qui a été demandé et ce qui a été fait.

**Commande slash :** `/log` pour écrire une entrée de log.

## Commandes slash

Les commandes slash sont des prompts réutilisables stockés dans `.github/prompts/`. Elles sont invoquées dans le chat VS Code Copilot via `/nom-de-la-commande`.

### Liste des commandes

| Commande | Description | Phase(s) |
|---|---|---|
| `/current` | État de la ronde actuelle, phase en cours, prochaines étapes | Toutes |
| `/eval-ticket` | Évaluer la clarté et la complétude du ticket | 1, 2 |
| `/init-enhancement` | Initialiser une nouvelle ronde (répertoire + ticket) | Pré-1 |
| `/ready-to-advance` | Vérifier si la phase courante peut être terminée | Toutes |
| `/plan` | Produire le plan de Phase 2 | 2 |
| `/implement` | Lancer l'implémentation selon le plan | 3 |
| `/verify` | Générer les instructions de vérification | 4 |
| `/consolidate` | Lancer la consolidation (version, changelog, release note) | 5 |
| `/publish` | Instructions de merge et publication | 6 |
| `/log` | Écrire un résumé du prompt courant dans le log | Toutes |

### Ajout d'une commande slash

Créer un fichier `.github/prompts/nom.prompt.md` avec le format :

```markdown
---
description: "Description courte"
---

Instructions pour l'IA.
```

## Commandes CLI `nou`

Le CLI `nou` fournit des raccourcis pour les opérations courantes. L'IA doit utiliser ces commandes lorsqu'elles sont disponibles.

### Commandes pour l'utilisateur

```bash
nou help                    # Aide générale
nou status                  # État du projet (branche, phase, ticket)
nou run "instruction"       # Lancer l'agent IA avec une requête
```

### Commandes de développement

```bash
nou dev init [name]         # Créer une nouvelle ronde d'amélioration
nou dev ticket              # Afficher le ticket de la ronde courante
nou dev phase               # Afficher la phase courante
nou dev log "description"   # Écrire une entrée de log
```

### Commandes de documentation

```bash
nou doc needs               # Lister les besoins
nou doc requirements        # Lister les requis
nou doc features            # Lister les fonctionnalités
```

### Autres commandes utiles

```bash
cargo check                 # Vérifier la compilation
cargo build --release       # Compiler en release
cargo run -- "..."          # Lancer l'agent avec une requête
cargo clippy                # Linter
cargo fmt                   # Formater le code
```

## Directives pour l'implémentation

### Stack technique

- **Langage :** Rust (edition 2024)
- **Runtime async :** tokio
- **HTTP client :** reqwest (appels API Anthropic)
- **Google Docs :** google-docs1 v5 + yup-oauth2 v9 + hyper 0.14
- **Sérialisation :** serde / serde_json
- **Erreurs :** anyhow + thiserror
- **Logging :** tracing + tracing-subscriber
- **Config :** dotenvy (fichier `.env`)

### Architecture et structure du code

```
src/
├── main.rs          # Point d'entrée CLI, chargement config, lancement agent
├── agent.rs         # Boucle agentique : appels API Anthropic, gestion tool_use/tool_result
├── google_docs.rs   # Client Google Docs API (CRUD via compte de service)
└── tools.rs         # Définitions JSON des outils + struct ToolResult
```

### Conventions de code

- Langue du code : commentaires et messages en **français**
- Les erreurs utilisent `anyhow::Result` pour la propagation, `thiserror` pour les types d'erreur custom
- Les logs passent par `tracing` (macros `info!`, `debug!`, `error!`)
- Les variables sensibles (clés API, JSON service account) sont dans `.env` et **jamais committées**
- Chaque outil exposé à Claude est défini dans `tools.rs` avec un schéma JSON conforme au format Anthropic tool_use

## Fichiers sensibles (ne jamais committer)

- `.env`
- `service-account.json` / `*-service-account.json`
