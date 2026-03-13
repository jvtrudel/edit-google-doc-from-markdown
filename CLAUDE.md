# CLAUDE.md

Instructions pour les agents IA travaillant sur ce projet.

## Projet

Agent IA en Rust qui utilise Claude (API Anthropic) pour éditer des documents Google Docs via une boucle agentique (tool use loop).

## Stack technique

- **Langage :** Rust (edition 2024)
- **Runtime async :** tokio
- **HTTP client :** reqwest (appels API Anthropic)
- **Google Docs :** google-docs1 v5 + yup-oauth2 v9 + hyper 0.14
- **Sérialisation :** serde / serde_json
- **Erreurs :** anyhow + thiserror
- **Logging :** tracing + tracing-subscriber
- **Config :** dotenvy (fichier `.env`)

## Structure du code

```
src/
├── main.rs          # Point d'entrée CLI, chargement config, lancement agent
├── agent.rs         # Boucle agentique : appels API Anthropic, gestion tool_use/tool_result
├── google_docs.rs   # Client Google Docs API (CRUD via compte de service)
└── tools.rs         # Définitions JSON des outils + struct ToolResult
```

## Conventions

- Langue du code : commentaires et messages en **français**
- Les erreurs utilisent `anyhow::Result` pour la propagation, `thiserror` pour les types d'erreur custom
- Les logs passent par `tracing` (macros `info!`, `debug!`, `error!`)
- Les variables sensibles (clés API, JSON service account) sont dans `.env` et **jamais committées**
- Chaque outil exposé à Claude est défini dans `tools.rs` avec un schéma JSON conforme au format Anthropic tool_use

## Commandes utiles

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

## Historique des prompts

Les résumés des interactions avec l'agent sont conservés dans `.dev/prompt-history/` avec la convention de nommage `NNN-description.md`.

À chaque prompt écrire un résumé.

