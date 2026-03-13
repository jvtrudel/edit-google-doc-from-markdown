# CLAUDE.md

Instructions pour les agents IA travaillant sur ce projet.

## Projet

Les décisions architecturales sont documentées dans `.dev/adr/` au format `NNN-description.md`.

L'`ADR-001` (`001-portee-du-projet.md`) décrit et explique le problème que tente de résoudre ce projet.

Avant toute chose, vérifier que ce qui est demandé et ce qui est produit respecte l'`ADR-001`.

Soulever les incohérences et les ambiguités entre les demandes et tous les ADR. En cas de problème ou d'incertitude, demander des clarifications et refuser de travailler.


## Méthodologie de développement

Le développement est assisté par IA et se fait par ronde de prompts permettant de livrer une amélioration cohérente (nouvelle fonctionalité, refactorisation, débogage, documentation/exemple, ajout de tests, amélioration de la sécurité, etc).

Une ronde d'amélioration est décomposée en plusieurs phases. Une documentation doit être produite.

Le répertoire `.dev/` contient les informations décrivant les améliorations et l'historique de ce qui a été fait.

- `.dev/adr`: Description des décisions importantes
- `.dev/analyses`: information produite pour aider le développeur. N'a qu'une valeur informative
- `.dev/prompt-history`: Historique scturcturé de ce qui a été fait avec l'IA
- `.dev/needs`: besoins: recensement et informations concernant les besoins des utilisateurs
- `.dev/requirements`: requis: définition atomique de comment doit se comporter le système
- `.dev/features`: fonctionalités: description des capacités fournis à l'utilisateur.
- `.dev/specs`: spécification: Documentation détaillée permettant de valider qu'une implémentation est conforme à ce qui est atttendu

Voici comment se déroule le développement:

### Cycle de développement

#### 1- Définir la portée de l'amélioration

Durant cette phase, l'humain transmet à l'IA 

- Chaque ronde d'amélioration se déroule dans une branche.
- Avant de commencer, on doit avoir répertoire de travail dans `.dev/history/NNN-branche/` où stocker les informations de travail assité par IA.
- Le travail à effectuer doit être résumé dans un ticket (`.dev/history/NNN-branche/ticket.md`)
- On doit savoir:
  - de quel type d'amélioration s'agit-il?
  - quels sont les besoins utilisateurs impactés?
  - Y a-t-il des requis à ajouter ou à modifier?
  - Quels seront les fonctionnalités impactées et de quelle manière?
  - Y a-t-il lieu de modifier la spécification?
- En cas de doute, demander des clarifications et refuser de travailler.
- Le travail ne doit pas impacter trop de code ou trop de fonctionnalités. Proposer de scinder le travail en plusieurs rondes d'amélioration.

L'IA décide quand on est prêt à passer à l'autre phase.

#### 2- Planifier l'amélioration

L'objectif de cette phase est de éfinir ce qui sera fait par l'IA. L'humain doit comprendre et donner son accord.

L'IA doit produire un document de planification (`.dev/history/NNN-branche/ticket.md`) qui explique ce qui sera fait et pourquoi.

L'humain apporte des précisions à sa demande si nécessaire.

L'humain décide si on peut passer à la prochaine étape.

#### 3- Implémenter l'amélioration

Ici, l'IA implémente ce qui a été planifié.

#### 4- Vérifier l'amélioration

Ici, l'IA explique à l'humain comment vérifier l'amélioration.

L'humain vérifie et valide ou non les changements.

#### 5- Consolider l'amélioration

Optionnellement, apporter les modifications nécessaires à la maintenance:

- améliorer la documentation,
- restructurer le code,
- ajouter des tests, etc.

Obligatoirement:
  - Définir le numéro de la prochaine version,
  - produire un changelog,
  - produire un release note.

#### 6- Publier l'amélioration

- Merger le code dans la branche main,
- appliquer la version, et
- pousser le code et la nouvelle version

### Déroulement du développement

- Créer une nouvelle branche à partir de main pour initialiser une nouvelle ronde d'amélioration.
- Identifier la phase de développement. Si c'est ambigue, demander au développeur.
- Travailler à résoudre chaque phase en dialoguant humain-AI.
- Un commit permet de signaler la fin d'une phase. Mais ce n'est pas obligatoire.
- Un merge dans main signale la fin d'une ronde d'amélioration.

### Historique des prompts

Les résumés des interactions avec l'agent sont conservés dans `.dev/history/NNN-branche/log/MMM-prompt-description`.

À chaque prompt écrire un résumé de ce qui a été demandé et ce qui a été fait.

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

### Architecture et Structure du code



```
src/
├── main.rs          # Point d'entrée CLI, chargement config, lancement agent
├── agent.rs         # Boucle agentique : appels API Anthropic, gestion tool_use/tool_result
├── google_docs.rs   # Client Google Docs API (CRUD via compte de service)
└── tools.rs         # Définitions JSON des outils + struct ToolResult
```

### Conventions

- Langue du code : commentaires et messages en **français**
- Les erreurs utilisent `anyhow::Result` pour la propagation, `thiserror` pour les types d'erreur custom
- Les logs passent par `tracing` (macros `info!`, `debug!`, `error!`)
- Les variables sensibles (clés API, JSON service account) sont dans `.env` et **jamais committées**
- Chaque outil exposé à Claude est défini dans `tools.rs` avec un schéma JSON conforme au format Anthropic tool_use

## Commandes

Lorsque possible, avoir recours aux commandes `nou`

### Commandes de développement

`nou dev DEV_COMMAND`

### Commandes de tests

`nou test TEST_COMMAND`

### Commandes pour l'utilisateur

`nou COMMAND`

### Autres commandes utiles

```bash
cargo check          # Vérifier la compilation
cargo build --release # Compiler en release
cargo run -- "..."   # Lancer l'agent avec une requête
cargo clippy         # Linter
cargo fmt            # Formater le code
```

## Fichiers sensibles (ne jamais committer)

- `.env`
- `service-account.json` / `*-service-account.json`
