# Release Note — v0.4.0

**Date :** 2026-03-15
**Branche :** `impl/push-to-google`

---

## Push Markdown → Google Docs

La commande `nou push` est maintenant opérationnelle.

### Utilisation

```bash
nou push fichier.md --doc-id 1Abc...xyz   # première fois
nou push                                   # fois suivantes (mémorisé)
nou push --force                           # ignorer les conflits
```

### Ce qui est converti

| Markdown            | Google Docs                      |
|---------------------|----------------------------------|
| `# Titre`           | Style Titre 1 (natif)            |
| `## Titre`          | Style Titre 2 (natif)            |
| `**gras**`          | Gras                             |
| `*italique*`        | Italique                         |
| `[texte](url)`      | Lien hypertexte                  |
| `- item`            | Liste à puces                    |
| `1. item`           | Liste numérotée                  |
| ` ```code``` `      | Texte brut (formaté comme code)  |
| `---`               | Texte `---`                      |

### Comportement de sécurité

Si le document distant a été modifié depuis le dernier push/pull, la commande refuse et demande `--force`.

### Ce qui reste hors portée (rondes ultérieures)

- Préservation du style visuel (police, couleur, taille) lors du push
- Commande `nou status`
- Signalement des pertes d'information (images, tableaux)


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
