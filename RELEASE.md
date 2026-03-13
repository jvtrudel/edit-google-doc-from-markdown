# Release Note — v0.1.0

Première version du projet edit-google-doc-from-markdown.

## Quoi de neuf

Ce release pose les fondations méthodologiques du projet :

- **Méthodologie de développement** : un cycle en 6 phases
  (définir → planifier → implémenter → vérifier → consolider → publier)
  guide chaque amélioration du projet.

- **10 commandes slash** utilisables dans VS Code + Copilot Chat
  pour naviguer le cycle de développement.

- **CLI `nou`** (prototype) : un script shell fournissant des
  raccourcis pour les opérations courantes (status, ticket, doc).

- **Documentation structurée** : chaque type de document (besoin,
  requis, fonctionnalité, spécification) a son dossier, son README
  et son template.

## Limitations

- Le CLI `nou` est un prototype : les commandes `run`, `dev init`
  et `dev log` ne sont pas encore fonctionnelles.
- L'agent IA (code Rust) n'a pas été modifié dans cette version.
