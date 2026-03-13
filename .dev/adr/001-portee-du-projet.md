# ADR-001 : Portée du projet edit-google-doc-from-markdown

**Date :** 2026-03-12  
**Statut :** En Cours
**Auteur :** Jérémy Viau-Trudel (assisté par Claude Opus 4.6)

## Contexte

Il existe un besoin d'automatiser l'édition de documents Google Docs à l'aide d'un agent IA capable de comprendre des instructions en langage naturel et de les traduire en opérations concrètes sur l'API Google Docs. L'objectif est de disposer d'un outil en ligne de commande simple et autonome, sans interface graphique.

## Décision

### Ce que le projet couvre

1. **Agent IA agentique** — Un programme CLI en Rust qui orchestre une boucle d'appels à l'API Anthropic (modèle Claude) avec le mécanisme tool use. L'agent reçoit une instruction utilisateur en langage naturel, planifie et exécute les étapes nécessaires via des appels d'outils, puis retourne le résultat final.

2. **Opérations Google Docs** — L'agent dispose des outils suivants pour interagir avec Google Docs :
   - Lire le contenu d'un document existant
   - Insérer du texte à une position donnée
   - Supprimer une plage de contenu
   - Remplacer toutes les occurrences d'un texte
   - Créer un nouveau document

3. **Authentification par compte de service** — L'accès aux documents se fait via un compte de service Google (clé JSON), sans interaction OAuth utilisateur. Les documents cibles doivent être partagés avec le compte de service.

4. **Interface CLI** — L'utilisateur interagit avec l'agent via la ligne de commande, soit en passant sa requête comme argument, soit via stdin en mode interactif.

5. **Langue française** — L'agent répond en français, les commentaires du code et la documentation sont rédigés en français.

### Ce que le projet ne couvre pas

- **Interface graphique** (web, desktop ou mobile)
- **Gestion multi-utilisateurs** ou authentification OAuth interactive
- **Autres services Google** (Sheets, Slides, Drive, Gmail, etc.)
- **Formatage riche avancé** (styles, tableaux, images intégrées) — seules les opérations textuelles de base sont supportées dans cette première version
- **Persistance de l'historique de conversation** — chaque exécution est indépendante, sans mémoire entre les sessions
- **Déploiement en tant que service** (API REST, webhook, bot Slack/Discord, etc.)

## Conséquences

- Le projet reste simple et focalisé : un seul binaire CLI, pas de serveur.
- L'ajout de nouveaux outils (formatage, listes, en-têtes) pourra se faire incrémentalement dans `tools.rs` et `google_docs.rs` sans modifier l'architecture.
- Si l'accès à d'autres services Google devient nécessaire, il faudra créer de nouveaux modules dédiés.
- Le choix du compte de service simplifie l'authentification mais limite l'accès aux seuls documents explicitement partagés.
