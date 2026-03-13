# Analyse : Commandes Slash, Skills et Conventions

**Date** : 2026-03-12  
**Contexte** : Développement assisté par IA dans VS Code + GitHub Copilot (modèle Claude Opus 4.6)

---

## 1. Qu'est-ce qu'une commande slash ?

Une **commande slash** est un raccourci préfixé par `/` qui déclenche une action prédéfinie dans un chat IA. C'est un **prompt réutilisable** encapsulé derrière un nom court.

Au lieu d'écrire à chaque fois :

> "Analyse le code sélectionné, vérifie les conventions Rust du projet, la gestion d'erreurs avec anyhow..."

On tape simplement :

```
/review
```

### Lien avec les "Skills" au sens de Claude

Chez Anthropic, le concept équivalent s'appelle **Custom Slash Commands** dans Claude Code (le CLI). Ce sont des fichiers Markdown contenant un **prompt template** que Claude exécute comme une "compétence" (skill).

Un **skill** est donc :
- Un prompt structuré et persistant
- Associé à un nom invocable via `/`
- Paramétrable (via `$ARGUMENTS` dans Claude Code)
- Versionnable (fichier dans le dépôt)

L'idée fondamentale : **transformer un savoir-faire récurrent en un artefact réutilisable et partageable**.

---

## 2. Convention VS Code + GitHub Copilot

### Commandes intégrées

VS Code Copilot Chat propose des commandes slash built-in :

| Commande | Description |
|---|---|
| `/explain` | Explique le code sélectionné |
| `/fix` | Propose un correctif pour le code |
| `/tests` | Génère des tests unitaires |
| `/doc` | Génère de la documentation |
| `/new` | Crée un nouveau fichier/projet |
| `/newNotebook` | Crée un Jupyter Notebook |
| `/search` | Génère des paramètres de recherche workspace |

### Commandes slash personnalisées

Depuis VS Code 1.99+, Copilot supporte les **commandes personnalisées via des fichiers prompt** :

**Emplacement** :
```
.github/prompts/
├── mon-skill.prompt.md
├── review-rust.prompt.md
└── commit-fr.prompt.md
```

**Format** : fichier Markdown avec front matter YAML

```markdown
---
mode: edit
description: "Description courte affichée dans le menu"
---
Contenu du prompt ici.
Le code sélectionné ou le fichier ouvert est automatiquement inclus comme contexte.
```

**Invocation** dans le chat Copilot :
```
/mon-skill [contexte additionnel optionnel]
```

**Propriétés du front matter** :

| Propriété | Valeurs | Description |
|---|---|---|
| `mode` | `edit`, `ask`, `agent` | Mode d'interaction Copilot |
| `description` | texte | Description affichée dans l'autocomplétion |

> **Prérequis** : activer le setting `"chat.promptFiles": true` dans VS Code.

---

## 3. Convention de Claude Code (CLI)

Claude Code utilise une convention différente basée sur des fichiers Markdown purs :

### Emplacements

| Portée | Chemin | Invocation |
|---|---|---|
| **Projet** | `.claude/commands/nom.md` | `/project:nom` |
| **Utilisateur** | `~/.claude/commands/nom.md` | `/user:nom` |

### Format

Pas de front matter. Le fichier est du Markdown pur. Les arguments de l'utilisateur sont injectés via la variable `$ARGUMENTS` :

```markdown
Analyse le diff git stagé et génère un message de commit conventionnel en français.

Format : type(scope): description

Contexte additionnel : $ARGUMENTS
```

### Sous-répertoires

Les sous-répertoires créent des **espaces de noms** :

```
.claude/commands/
├── git/
│   ├── commit.md        → /project:git:commit
│   └── review.md        → /project:git:review
└── code/
    └── new-tool.md      → /project:code:new-tool
```

### Différences clés entre les deux conventions

| Aspect | VS Code Copilot | Claude Code |
|---|---|---|
| **Emplacement** | `.github/prompts/` | `.claude/commands/` |
| **Extension** | `.prompt.md` | `.md` |
| **Front matter** | YAML (mode, description) | Aucun |
| **Arguments** | Contexte implicite | `$ARGUMENTS` explicite |
| **Invocation** | `/nom` | `/project:nom` ou `/user:nom` |
| **Sous-répertoires** | Non supporté (à date) | Oui, crée un namespace |

---

## 4. Exemples implémentés (convention Claude)

### Exemple 1 : Revue de code Rust

```markdown
# filepath: .claude/commands/review.md

Tu es un reviewer expert Rust pour ce projet d'agent IA éditant des Google Docs.

Analyse le code fourni et vérifie :

## Conformité au projet
- Commentaires et messages en **français**
- Propagation d'erreurs via `anyhow::Result`
- Types d'erreur custom via `thiserror`
- Logging via `tracing` (macros `info!`, `debug!`, `error!`) — jamais `println!`
- Variables sensibles jamais en dur (toujours via `.env` / `dotenvy`)

## Qualité Rust
- Gestion correcte de l'async (tokio)
- Pas de `.unwrap()` en production (utiliser `?` ou `.context()`)
- Ownership et lifetimes idiomatiques
- Conformité clippy

## Sécurité
- Pas de secrets exposés
- Validation des entrées utilisateur
- Gestion des erreurs réseau (timeouts, retries)

Fournis un rapport structuré :
1. **Résumé** (1-2 lignes)
2. **Problèmes critiques** 🔴
3. **Avertissements** 🟡
4. **Suggestions** 🟢
5. **Score** /10

Contexte additionnel : $ARGUMENTS
```

**Invocation (Claude Code)** : `/project:review src/agent.rs`

---

### Exemple 2 : Créer un nouvel outil pour l'agent

```markdown
# filepath: .claude/commands/new-tool.md

Crée un nouvel outil (tool) pour l'agent Claude dans ce projet.

L'outil doit :
1. Être défini dans `src/tools.rs` avec son schéma JSON conforme au format Anthropic `tool_use`
2. Avoir un handler dans `src/agent.rs` dans le match des appels d'outils
3. Suivre le pattern existant des autres outils

Nom et description de l'outil : $ARGUMENTS

## Contraintes
- Le schéma JSON doit inclure `name`, `description`, `input_schema` avec les propriétés typées
- Le handler retourne un `ToolResult` (défini dans `src/tools.rs`)
- Commentaires en français
- Erreurs propagées avec `anyhow::Result`
- Logs avec `tracing`

## Structure à générer

Dans `tools.rs` : ajouter la définition JSON du tool dans la liste des outils.
Dans `agent.rs` : ajouter le bras de match pour le `tool_use`.

Montre le code complet à ajouter dans chaque fichier.
```

**Invocation (Claude Code)** : `/project:new-tool lire le contenu brut d'un Google Doc`

---

## 5. Adaptation pour VS Code + Copilot via liens symboliques

### Problème

Les deux conventions utilisent des emplacements et extensions différents :

```
.claude/commands/review.md       ← Claude Code
.github/prompts/review.prompt.md ← VS Code Copilot
```

### Solution : liens symboliques

On crée les fichiers source dans `.claude/commands/` (convention Claude, plus simple), puis on crée des **liens symboliques** dans `.github/prompts/` avec l'extension `.prompt.md`.

```bash
#!/bin/bash
# Script : scripts/sync-slash-commands.sh
# Crée des liens symboliques des commandes Claude vers le format Copilot

set -euo pipefail

CLAUDE_DIR=".claude/commands"
COPILOT_DIR=".github/prompts"

mkdir -p "$COPILOT_DIR"

for cmd_file in "$CLAUDE_DIR"/*.md; do
    [ -f "$cmd_file" ] || continue

    name=$(basename "$cmd_file" .md)
    link="$COPILOT_DIR/${name}.prompt.md"

    # Calculer le chemin relatif pour le lien symbolique
    relative_path=$(realpath --relative-to="$COPILOT_DIR" "$cmd_file")

    # Supprimer l'ancien lien s'il existe
    [ -L "$link" ] && rm "$link"

    ln -s "$relative_path" "$link"
    echo "🔗 $link -> $relative_path"
done

echo "✅ Synchronisation terminée."
```

### Résultat dans l'arborescence

```
.claude/commands/
├── review.md              ← source de vérité
└── new-tool.md

.github/prompts/
├── review.prompt.md       ← lien symbolique → ../../.claude/commands/review.md
└── new-tool.prompt.md     ← lien symbolique → ../../.claude/commands/new-tool.md
```

### Utilisation selon l'environnement

| Environnement | Commande |
|---|---|
| **Claude Code (terminal)** | `/project:review src/agent.rs` |
| **VS Code Copilot Chat** | `/review` (avec le fichier ouvert comme contexte) |

### Limites connues

1. **`$ARGUMENTS`** : Copilot ne substitue pas `$ARGUMENTS`. Le texte apparaît tel quel dans le prompt, ce qui fonctionne quand même car le modèle comprend que c'est un placeholder pour le contexte fourni par l'utilisateur.
2. **Front matter manquant** : les fichiers Claude n'ont pas de front matter YAML. Copilot utilisera les valeurs par défaut (ce qui convient dans la majorité des cas).
3. **Sous-répertoires** : si on utilise des sous-répertoires dans `.claude/commands/`, il faut aplatir les noms pour Copilot (ex: `git/commit.md` → `git-commit.prompt.md`).

### Workflow recommandé

1. Créer/modifier les commandes dans `.claude/commands/`
2. Exécuter `scripts/sync-slash-commands.sh`
3. Les commandes sont disponibles dans les deux environnements
4. Committer les liens symboliques (Git les supporte nativement)
