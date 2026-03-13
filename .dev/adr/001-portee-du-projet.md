# ADR-001 : Portée du projet — Synchronisation Markdown ↔ Google Docs

**Date :** 2026-03-12 (réécrit 2026-03-13)  
**Statut :** Accepté  
**Auteur :** Jérémy Viau-Trudel (assisté par Claude Opus 4.6)

## Contexte

Deux populations d'utilisateurs travaillent sur les mêmes documents avec des outils incompatibles :

- **Développeurs** : éditeur de code + git + Markdown. Avantages : versionnement, diffs lisibles, manipulation IA, automatisation.
- **Collaborateurs non-dev** : Google Docs. Avantages : collaboration en temps réel, accessibilité, WYSIWYG.

Le besoin fondamental est de **concilier ces deux perspectives** sans forcer l'une des parties à adopter l'outil de l'autre. Le Markdown sert de **plus petit dénominateur commun** pour le contenu, tandis que Google Docs ajoute une couche de style visuel par-dessus.

Voir : `NEED-001` (Synchronisation Markdown ↔ Google Docs), `NEED-002` (Suivi de l'évolution via git).

## Décision

### Ce que le projet couvre

1. **Synchronisation manuelle Markdown ↔ Google Docs** — Un outil CLI en Rust permettant de :
   - **Push** : publier le contenu d'un fichier Markdown local vers un Google Doc distant
   - **Pull** : récupérer le contenu d'un Google Doc distant en fichier Markdown local

2. **Séparation contenu / style** — Le Markdown représente le contenu (structure sémantique, texte). Le style Google Docs (polices, couleurs, tailles) est une couche additionnelle préservée séparément. Si le contenu Markdown n'a pas changé depuis la dernière synchronisation, le style Google Docs est réappliqué tel quel.

3. **Détection de conflits** — Le système détecte quand le Markdown ET le Google Doc ont été modifiés depuis la dernière synchronisation, et refuse l'opération avec un message clair. Pas de résolution automatique.

4. **Détection de perte d'information** — Le système signale les éléments qui ne peuvent pas être fidèlement représentés dans le format cible (ex: images, commentaires Google Docs → Markdown).

5. **Stabilité du Markdown généré** — La conversion Google Docs → Markdown est déterministe et stable. Les diffs git sont propres et reflètent uniquement les modifications réelles.

6. **Association 1:1** — Un fichier Markdown est associé à exactement un Google Doc. La synchronisation porte sur un seul fichier à la fois.

7. **Authentification par compte de service** — L'accès aux Google Docs se fait via un compte de service Google (clé JSON), sans interaction OAuth utilisateur.

8. **Transport via l'API Google** — Lecture (`documents.get`) et écriture (`documents.batchUpdate`) via l'API Google Docs, avec gestion des erreurs et des quotas.

9. **Interface CLI** — L'utilisateur interagit via la ligne de commande (`nou push`, `nou pull`, `nou status`).

10. **Langue française** — Le code, les commentaires et la documentation sont en français.

### Ce que le projet ne couvre pas

- **Synchronisation automatique** (watch, polling, webhooks) — synchronisation manuelle uniquement
- **Multi-fichiers** — un fichier à la fois
- **Résolution automatique de conflits** — détection uniquement
- **Interface graphique** (web, desktop, mobile)
- **Autres services Google** (Sheets, Slides, Drive, Gmail)
- **Formatage riche sans équivalent Markdown** (tableaux complexes, images intégrées) — signalé comme perte
- **Gestion multi-utilisateurs** ou authentification OAuth interactive
- **Déploiement en tant que service** (API REST, webhook, bot)

### Principes architecturaux

- **Markdown = contenu** : le fichier Markdown est la source de vérité pour le contenu sémantique
- **Google Docs = contenu + style** : le style est une couche additionnelle au-dessus du contenu
- **Diffs propres** : toute opération doit produire des diffs git lisibles et sans bruit
- **CLI first** : les opérations sont des commandes invocables en ligne de commande

## Conséquences

- Le projet est focalisé sur la **synchronisation bidirectionnelle** entre deux formats, pas sur l'édition IA de documents.
- L'architecture sépare clairement : transport (API), conversion (Markdown ↔ Google Docs structure), et style (préservation/réapplication).
- L'ajout futur de la résolution de conflits, du multi-fichiers ou de la synchronisation automatique pourra se faire incrémentalement sans changer l'architecture de base.
- Le choix du compte de service simplifie l'authentification mais limite l'accès aux documents explicitement partagés.

## Documents de référence

| Type | Documents |
|---|---|
| Besoins | `NEED-001`, `NEED-002` |
| Requis | `REQ-001` à `REQ-009` |
| Fonctionnalités | `FEAT-001` à `FEAT-003` |
