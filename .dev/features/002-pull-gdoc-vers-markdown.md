# FEAT-002 — Récupération Google Doc → Markdown (pull)

**Date :** 2026-03-13  
**Statut :** Proposée  
**Priorité :** Critique

## Description

L'utilisateur peut récupérer le contenu d'un Google Doc distant et le convertir en fichier Markdown local via une commande CLI manuelle.

Le pull effectue les opérations suivantes :
1. Vérifier l'association fichier ↔ document
2. Détecter les conflits (modification des deux côtés depuis la dernière sync)
3. Extraire le contenu du Google Doc et le convertir en Markdown
4. Sauvegarder les informations de style Google Docs séparément (pour réapplication lors d'un push futur)
5. Écrire le fichier Markdown local
6. Signaler les éventuelles pertes d'information

## Commande CLI

```bash
nou pull <fichier.md> [--doc-id <id>] [--force]
```

- `<fichier.md>` : chemin du fichier Markdown de destination
- `--doc-id <id>` : identifiant du Google Doc source (optionnel si l'association existe)
- `--force` : écraser le fichier local même en cas de conflit détecté

## Requis implémentés

- `REQ-002` — Conversion Google Docs → Markdown (contenu)
- `REQ-003` — Association fichier ↔ document
- `REQ-004` — Détection de perte d'information
- `REQ-005` — Séparation contenu / style
- `REQ-006` — Détection de conflits
- `REQ-007` — Authentification par compte de service
- `REQ-008` — Transport via l'API Google (lecture/écriture distante)

## Besoins associés

- `NEED-001` — Synchronisation Markdown ↔ Google Docs
