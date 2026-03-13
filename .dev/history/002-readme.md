# Prompt 2 — Création du README

**Date :** 2026-03-12  
**Modèle :** Claude Opus 4.6 (via GitHub Copilot dans VS Code)  
**Environnement :** VS Code, Linux, workspace `[workspace]`

## Prompt

> Produire un README.md qui inclut les instructions pour utiliser le code

## Résultat

Fichier `README.md` créé à la racine du projet avec les sections suivantes :

- **Architecture** — schéma de la structure `src/` et rôle de chaque module
- **Outils disponibles** — tableau des 5 outils (read_document, insert_text, delete_content_range, replace_all_text, create_document)
- **Prérequis** — Rust 1.75+, compte Anthropic, projet Google Cloud
- **Configuration** — guide pas-à-pas : clé API Anthropic, création de compte de service Google, partage des documents, fichier `.env`
- **Compilation** — `cargo build --release`
- **Utilisation** — 3 modes : argument CLI, stdin interactif, binaire compilé
- **Trouver l'ID d'un document** — explication avec exemple d'URL
- **Contrôle des logs** — exemples `RUST_LOG`
- **Exemples de requêtes** — créer, lire/résumer, modifier, édition complexe
- **Sécurité** — rappel de ne jamais committer `.env` ni le JSON du compte de service
