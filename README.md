# edit-google-doc

Agent IA développé en Rust qui utilise **Claude Opus 4.6** (via l'API Anthropic) pour lire et modifier des documents Google Docs de manière autonome grâce à une boucle agentique avec appels d'outils.

## Architecture

```
src/
├── main.rs          # Point d'entrée : CLI et initialisation
├── agent.rs         # Boucle agentique Claude (tool use loop)
├── google_docs.rs   # Client Google Docs API (compte de service)
└── tools.rs         # Définitions des outils exposés à Claude
```

### Outils disponibles

| Outil | Description |
|---|---|
| `read_document` | Lit le contenu textuel d'un document par son ID |
| `insert_text` | Insère du texte à un index précis |
| `delete_content_range` | Supprime une plage de contenu |
| `replace_all_text` | Remplace toutes les occurrences d'un texte |
| `create_document` | Crée un nouveau document Google Docs |

## Prérequis

- **Rust** 1.75+ : [rustup.rs](https://rustup.rs)
- Un compte **Anthropic** avec clé API : [console.anthropic.com](https://console.anthropic.com)
- Un projet **Google Cloud** avec l'API Google Docs activée et un compte de service

## Configuration

### 1. Clé API Anthropic

Récupère ta clé API sur [console.anthropic.com](https://console.anthropic.com/settings/keys).

### 2. Compte de service Google

1. Ouvre la [Google Cloud Console](https://console.cloud.google.com)
2. Active l'**API Google Docs**
3. Crée un **compte de service** : *IAM & Admin → Comptes de service → Créer*
4. Génère une clé JSON : *Actions → Gérer les clés → Ajouter une clé → JSON*
5. Télécharge le fichier JSON et place-le à la racine du projet (ex. `service-account.json`)

### 3. Partager les documents Google Docs

Pour que l'agent puisse accéder à un document, partage-le avec l'adresse e-mail du compte de service (ex. `mon-agent@mon-projet.iam.gserviceaccount.com`) en lui accordant le rôle **Éditeur**.

### 4. Variables d'environnement

Crée un fichier `.env` à partir du template :

```bash
cp .env.example .env
```

Remplis les valeurs dans `.env` :

```dotenv
ANTHROPIC_API_KEY=sk-ant-...
GOOGLE_SERVICE_ACCOUNT_KEY=service-account.json
RUST_LOG=edit_google_doc=info
```

## Compilation

```bash
cargo build --release
```

## Utilisation

### Via argument CLI

```bash
cargo run -- "Crée un document intitulé 'Compte-rendu réunion' et ajoute une introduction"
```

```bash
cargo run -- "Dans le document 1AbCdEfGhIjKlMnOpQrStUvWxYz0123456789ABCDEFG, remplace 'v1.0' par 'v2.0'"
```

### Via stdin (mode interactif)

```bash
cargo run
# > Entrez votre demande pour l'agent (puis appuyez sur Entrée):
# > Lis le document 1AbCdEfGhIjKlMnOpQrStUvWxYz0123456789ABCDEFG et résume son contenu
```

### Binaire compilé

```bash
./target/release/edit-google-doc "Insère un pied de page dans le document 1BxiMVs0XRA5..."
```

## Trouver l'ID d'un document Google Docs

L'ID se trouve dans l'URL du document :

```
https://docs.google.com/document/d/1AbCdEfGhIjKlMnOpQrStUvWxYz0123456789ABCDEFG/edit
                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                                    C'est cet identifiant
```

## Contrôle du niveau de logs

```bash
# Logs détaillés (debug)
RUST_LOG=edit_google_doc=debug cargo run -- "..."

# Logs minimaux (erreurs seulement)
RUST_LOG=error cargo run -- "..."

# Tous les logs (y compris les dépendances)
RUST_LOG=trace cargo run -- "..."
```

## Exemples de requêtes

```bash
# Créer un document
cargo run -- "Crée un document nommé 'Rapport mensuel mars 2026'"

# Lire et résumer
cargo run -- "Lis le document <ID> et liste tous les points d'action"

# Modifier du contenu
cargo run -- "Dans le document <ID>, remplace toutes les occurrences de 'Dupont' par 'Martin'"

# Édition complexe
cargo run -- "Ouvre le document <ID>, ajoute un titre 'Conclusion' à la fin, puis insère un paragraphe de résumé"
```

## Sécurité

- Ne committe **jamais** le fichier `.env` ni le fichier JSON du compte de service
- Ces fichiers sont déjà exclus via `.gitignore`
- Limite les permissions du compte de service au strict nécessaire (accès uniquement aux documents concernés)
