# TODO - Décisions à prendre et/ou à documenter

## conventions de gestion de code avec git et lien avec le cycle de développement

- voir la description de la méthodologie de développement dans le ichier `CLAUDE.md`

## information-centric development

- distinguer les données (ce sur quoi on opère), l'information (le contenu informationnel) et la connaissance (le savoir)
- les opérations doivent traiter l'information et non seulement la donnée
- la donnée est définit par sa structure, cette structure n'est pas un absolue et devrait pouvoir être exprimé dans différents format
- la structure de donnée de tout objet sur lequel on opère doit être analysée
- toute opération agit sur l'information et se caractérise par les transformations faites sur l'information
- l'utilisateur doit pouvoir passer d'un format à l'autre avec ou sans perte d'information.
- l'utilisateur doit être informé lorsque les opérations qui impliquent une perte d'information
- un format de donnée est une représentation de l'information

## CLI first, AI second

Pourquoi: interface humain-machine, UX, productivité 

**CLI first**

- On fournit à l'utilisateur des clis permettant d'exécuter les opérations courantes
- On distingue les opérations de développement et les opérations de l'utilisateur

**AI second**

- L'orsque l'on utilise l'IA, celui-ci doit également utiliser les CLIs
- on utilise des commandes slash

**Conventions:**

- le cli pour l'utilisateur se nomme `nou` => `nou COMMAND ...`
- les commandes de développement sont accessibles via `nou dev DEV_COMMAND ...`


