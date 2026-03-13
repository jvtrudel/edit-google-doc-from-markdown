# FEAT-001 — Publication Markdown → Google Doc (push)

**Date :** 2026-03-13  
**Statut :** Proposée  
**Priorité :** Critique

## Description

L'utilisateur peut publier le contenu d'un fichier Markdown local vers un Google Doc distant via une commande CLI manuelle.

Le push effectue les opérations suivantes :
1. Vérifier l'association fichier ↔ document (la créer si nécessaire)
2. Détecter les conflits (modification des deux côtés depuis la dernière sync)
3. Convertir le contenu Markdown en contenu Google Docs
4. Si le contenu n'a pas changé : réappliquer le style Google Docs existant
5. Si le contenu a changé : appliquer le style aux parties inchangées, style par défaut aux nouvelles parties
6. Signaler les éventuelles pertes d'information

## Commande CLI

```bash
nou push <fichier.md> [--doc-id <id>] [--force]
```

- `<fichier.md>` : chemin du fichier Markdown à publier
- `--doc-id <id>` : identifiant du Google Doc cible (optionnel si l'association existe)
- `--force` : écraser le Google Doc même en cas de conflit détecté

## Requis implémentés

- `REQ-001` — Conversion Markdown → Google Docs (contenu)
- `REQ-003` — Association fichier ↔ document
- `REQ-004` — Détection de perte d'information
- `REQ-005` — Séparation contenu / style
- `REQ-006` — Détection de conflits
- `REQ-007` — Authentification par compte de service
- `REQ-008` — Transport via l'API Google (lecture/écriture distante)

## Besoins associés

- `NEED-001` — Synchronisation Markdown ↔ Google Docs
