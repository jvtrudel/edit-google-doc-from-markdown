# Changelog

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
