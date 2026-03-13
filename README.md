# nou — Synchronisation Markdown ↔ Google Docs

Outil CLI en Rust pour synchroniser un fichier Markdown avec un Google Doc.

**Tu travailles en Markdown dans ton éditeur et tu versionnes avec git. Tes collaborateurs travaillent dans Google Docs.** `nou` fait le pont entre les deux.

## Pourquoi

| | Toi (développeur) | Tes collaborateurs |
|---|---|---|
| **Outil** | Éditeur de code + git | Google Docs |
| **Format** | Markdown | Document riche (WYSIWYG) |
| **Force** | Versionnement, IA, diffs lisibles | Collaboration en temps réel, accessibilité |

Tu ne veux pas forcer tes collaborateurs à utiliser git. Ils ne veulent pas te forcer à utiliser Google Docs. `nou` synchronise les deux mondes.

## Comment ça marche

Le **Markdown** est le plus petit dénominateur commun — il représente le contenu. Le **Google Doc** ajoute une couche de style visuel par-dessus. `nou` les sépare :

- Le contenu vit dans le Markdown (et dans git)
- Le style vit dans le Google Doc (et est préservé lors des synchronisations)

```
fichier.md  ←→  Google Doc
  contenu         contenu + style
  (git)           (collaborateurs)
```

## Utilisation

```bash
# Publier ton Markdown vers un Google Doc
nou push mon-document.md --doc-id <ID_DU_DOCUMENT>

# Récupérer les modifications du Google Doc
nou pull mon-document.md

# Voir l'état de synchronisation
nou status mon-document.md
```

### Trouver l'ID d'un Google Doc

L'ID se trouve dans l'URL du document :

```
https://docs.google.com/document/d/1AbCdEfGhIjKlMnOpQrStUvWxYz0123456789ABCDEFG/edit
                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

### Gestion des conflits

Si le Markdown ET le Google Doc ont été modifiés depuis la dernière synchronisation, `nou` refuse l'opération et te prévient. Tu peux forcer avec `--force` si tu sais ce que tu fais.

### Perte d'information

Certains éléments Google Docs n'ont pas d'équivalent Markdown (images intégrées, commentaires, couleurs de texte). `nou` te signale ce qui sera perdu lors d'un pull.

## Installation

### Prérequis

- **Rust** 1.85+ : [rustup.rs](https://rustup.rs)
- Un projet **Google Cloud** avec l'API Google Docs activée et un compte de service

### 1. Compte de service Google

1. Ouvre la [Google Cloud Console](https://console.cloud.google.com)
2. Active l'**API Google Docs**
3. Crée un **compte de service** : *IAM & Admin → Comptes de service → Créer*
4. Génère une clé JSON : *Actions → Gérer les clés → Ajouter une clé → JSON*
5. Place le fichier JSON à la racine du projet (ex. `service-account.json`)

### 2. Partager les documents

Partage chaque Google Doc avec l'adresse e-mail du compte de service (ex. `mon-agent@mon-projet.iam.gserviceaccount.com`) en lui accordant le rôle **Éditeur**.

### 3. Configuration

```bash
cp .env.example .env
```

```dotenv
GOOGLE_SERVICE_ACCOUNT_KEY=service-account.json
RUST_LOG=edit_google_doc=info
```

### 4. Compilation

```bash
cargo build --release
```

## État du projet

Le projet est en développement actif. L'architecture est en place, l'implémentation des fonctionnalités est en cours.

| Fonctionnalité | État |
|---|---|
| CLI (push, pull, status) | ✅ Interface définie |
| Conversion Markdown → Google Docs | 🔲 À implémenter |
| Conversion Google Docs → Markdown | 🔲 À implémenter |
| Préservation du style | 🔲 À implémenter |
| Détection de conflits | 🔲 À implémenter |
| Transport API Google | ✅ Client prêt |

## Documentation

| Document | Description |
|---|---|
| [CLAUDE.md](CLAUDE.md) | Instructions pour le développement assisté par IA : méthodologie, conventions, commandes slash et CLI |
| [.dev/adr/001-portee-du-projet.md](.dev/adr/001-portee-du-projet.md) | Décision architecturale : portée et principes du projet |
| [.dev/needs/](.dev/needs/) | Besoins utilisateurs |
| [.dev/requirements/](.dev/requirements/) | Requis techniques (9 requis définis) |
| [.dev/features/](.dev/features/) | Fonctionnalités (3 features définies) |
| [CHANGELOG.md](CHANGELOG.md) | Historique des versions |

## Sécurité

- Ne committe **jamais** le fichier `.env` ni le fichier JSON du compte de service
- Ces fichiers sont déjà exclus via `.gitignore`
- Limite les permissions du compte de service au strict nécessaire

## Licence

À définir.
