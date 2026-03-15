# Prompt 5 — Premier ADR (portée du projet)

**Date :** 2026-03-12  
**Modèle :** Claude Opus 4.6 (via GitHub Copilot dans VS Code)  
**Environnement :** VS Code, Linux

## Prompt

> Ajouter un premier ADR décrivant la portée du projet

## Résultat

Fichier `.dev/adr/001-portee-du-projet.md` créé au format ADR (Architecture Decision Record) avec :

- **Contexte** — besoin d'automatiser l'édition Google Docs via un agent IA en langage naturel
- **Ce que le projet couvre** — agent agentique CLI en Rust, 5 opérations Google Docs, authentification par compte de service, interface CLI, langue française
- **Ce que le projet ne couvre pas** — GUI, multi-utilisateurs, autres services Google, formatage riche avancé, persistance de conversation, déploiement en tant que service
- **Conséquences** — simplicité du projet, extensibilité incrémentale des outils, limitation aux documents partagés avec le compte de service

Ajouté également une section « ADR » dans `CLAUDE.md` pointant vers `.dev/adr/`.
