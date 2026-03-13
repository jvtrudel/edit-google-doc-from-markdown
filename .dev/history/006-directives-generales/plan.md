## Plan de Phase 2 — Planification

### A. Analyse critique de CLAUDE.md

#### Forces actuelles
- Cycle de développement en 6 phases bien structuré
- Séparation claire des rôles humain/IA à chaque phase
- Structure `.dev/` pour la documentation
- Conventions de code explicites (langue, erreurs, logs, secrets)

#### Faiblesses identifiées

| # | Problème | Impact |
|---|---|---|
| F3 | **Aucun template** pour les fichiers de documentation (ticket, besoin, requis, feature, spec) | Les dossiers sont vides, pas de guidance |
| F4 | **Section Commandes vide** — mentionne `nou` sans lister aucune commande concrète | Inutilisable |
| F5 | **Pas de convention de nommage git** — branches, commits, tags, versionnement non définis | Implicite uniquement |
| F6 | **Pas de section sur les commandes slash** — aucune mention de `.github/prompts/` ni de leur rôle dans le workflow | Outil invisible |
| F7 | **Fautes de frappe** multiples (« fonctionalité », « scturcturé », « assité », etc.) | Qualité |
| F8 | **Pas de lien entre les phases et les commandes slash** — l'IA ne sait pas quelle commande utiliser quand | Perte d'efficacité |

#### Comparaison avec les pratiques connues

| Pratique | État dans CLAUDE.md | Action |
|---|---|---|
| **ADR** (Architecture Decision Records) | ✅ Présent et utilisé | Aucune |
| **Trunk-based dev + feature branches** | ✅ Implicite (branches + merge) | Expliciter les conventions |
| **Conventional Commits** | ❌ Absent | Ajouter une convention de message de commit |
| **Semantic Versioning** | ❌ Absent (Phase 5 dit « définir la version » sans règle) | Ajouter |
| **Docs-as-Code** | ✅ Partiellement (`.dev/`) | Compléter avec templates |
| **Prompt Engineering patterns** | ❌ Pas de guidance pour rédiger les prompts | Hors portée pour cette ronde |

#### Modifications prévues pour CLAUDE.md

1. **Corriger les fautes** et la phrase inachevée
3. **Ajouter une section « Conventions Git »** : nommage des branches (`type/description`), conventional commits, semantic versioning
4. **Ajouter une section « Commandes slash »** : liste des commandes, quand les utiliser, lien avec les phases du cycle
5. **Compléter la section « Commandes `nou` »** avec les commandes concrètes
6. **Ajouter une description du rôle de chaque dossier `.dev/`** avec renvoi vers les templates

### B. Structure `.dev/` — Analyse et améliorations

#### État actuel

| Dossier | Contenu | Problème |
|---|---|---|
| `.dev/adr/` | 1 ADR + TODO.md | ✅ Fonctionnel |
| `.dev/analyses/` | 2 analyses | ✅ Fonctionnel |
| `.dev/history/` | 3 entrées + ronde 006 | ✅ Fonctionnel |
| `.dev/needs/` | Vide | ❌ Pas de template |
| `.dev/requirements/` | Vide | ❌ Pas de template |
| `.dev/features/` | Vide | ❌ Pas de template |
| `.dev/specs/` | Vide | ❌ Pas de template |

#### Actions prévues

1. **Créer un fichier `README.md`** dans chaque dossier vide expliquant :
   - Le rôle du dossier
   - Le format attendu des fichiers
   - Un exemple type
2. **Créer un template** `_template.md` dans chaque dossier pour que l'IA puisse s'en servir comme modèle
3. **Déplacer `TODO.md`** hors de `.dev/adr/` — un TODO n'est pas un ADR. Le mettre à la racine de `.dev/` ou dans `.dev/analyses/`

### C. Commandes slash — Définition complète

#### Existantes (à conserver et améliorer)

| Commande | Rôle | Phase(s) |
|---|---|---|
| `/current` | État de la ronde actuelle | Toutes |
| `/1-eval-ticket` | Évaluer le ticket en cours | 1 |
| `/0-init-enhancement` | Initialiser une nouvelle ronde | Pré-1 |
| `/ready-to-advance` | Vérifier si on peut avancer | Toutes |

#### Nouvelles commandes proposées

| Commande | Rôle | Phase(s) |
|---|---|---|
| `/2-plan` | Produire le plan de Phase 2 dans le ticket | 2 |
| `/3-implement` | Lancer l'implémentation selon le plan | 3 |
| `/4-verify` | Générer les instructions de vérification | 4 |
| `/5-consolidate` | Lancer Phase 5 (changelog, version, release note) | 5 |
| `/6-publish` | Instructions de merge et publication | 6 |
| `/log` | Écrire un résumé de ce qui a été fait dans la ronde d'amélioration courrante dans `.dev/history/NNN/log/` | Toutes |

#### Total : 10 commandes slash couvrant l'ensemble du cycle

### D. Commandes CLI `nou` — Prototype

#### Forme du prototype

Un **script shell `nou`** à la racine du projet (pas un binaire Rust — ce serait disproportionné pour un prototype factice). Chaque sous-commande affiche sa documentation et un message « non implémenté ».

#### Commandes proposées

```
nou help                    # Affiche l'aide générale
nou status                  # État du projet (branche, phase, ticket)

nou dev init [name]         # Créer une nouvelle ronde d'amélioration
nou dev tickets             # afficher la liste des tickets et dire son état (en cours, terminé, abandonné, en pause, en problème)
nou dev ticket              # Afficher le ticket de la ronde courante
nou dev phase               # Afficher la phase courante
nou dev logs                # Lister les entrées de log

nou doc needs               # Lister les besoins
nou doc requirements        # Lister les requis
nou doc features            # Lister les fonctionnalités

```

#### Livrables pour le prototype
- Fichier `nou` (script shell exécutable) à la racine
- Chaque commande affiche : description, usage, et « ⚠️ Non implémenté — prévu pour vX.Y »
- Documentation dans `CLAUDE.md` section Commandes

---

## Ordre d'exécution (Phase 3)

| Étape | Tâche | Dépend de |
|---|---|---|
| 1 | Créer les README.md et templates dans `.dev/` | — |
| 2 | Déplacer `.dev/adr/TODO.md` → `.dev/TODO.md` | — |
| 3 | Réécrire `CLAUDE.md` (corrections + nouvelles sections) | Étapes 1-2 |
| 4 | Créer les 6 nouvelles commandes slash | Étape 3 |
| 5 | Créer le script `nou` (prototype) | Étape 3 |
| 6 | Mettre à jour le ticket (marquer Phase 3 terminée) | Étapes 4-5 |

## Estimation

~1 session d'implémentation (1 ronde de prompts Phase 3)



