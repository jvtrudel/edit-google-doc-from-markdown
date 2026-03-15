# Plan — Ronde 008 : Implémenter la commande Pull

## TL;DR

Implémenter la commande `nou pull` de bout en bout : conversion Google Docs → Markdown, persistance de l'association fichier ↔ document, orchestration du pull avec vérification simplifiée des conflits. Le travail touche 5 modules (`markdown.rs`, `converter.rs`, `mapping.rs`, `sync.rs`, `cli.rs`/`main.rs`) et produit 3 fichiers de tests. L'ordre d'implémentation suit les dépendances : modules indépendants d'abord (persistance, conversion), puis CLI, puis orchestration.

Le style est hors périmètre. Seul le contenu sémantique est converti.

---

## Étapes

### Étape 1 — Persistance : `mapping.rs`

Module indépendant, requis par toutes les étapes suivantes.

**Format de stockage :** fichier `.nou/state.json` au répertoire courant. Structure JSON :

```json
{
  "files": {
    "chemin/relatif/fichier.md": {
      "markdown_path": "chemin/relatif/fichier.md",
      "document_id": "1abc...",
      "last_sync": "2026-03-14T10:00:00Z",
      "last_markdown_hash": null,
      "last_revision_id": null
    }
  }
}
```

**Fonctions à implémenter dans** [src/mapping.rs](src/mapping.rs) :

1. `load_metadata(markdown_path)` → Lire `.nou/state.json`, chercher l'entrée correspondant au chemin. Retourner `Ok(None)` si le fichier ou l'entrée n'existe pas.
2. `save_metadata(metadata)` → Lire le fichier existant (ou créer `{}`), insérer/mettre à jour l'entrée, réécrire le fichier. Créer le répertoire `.nou/` si absent.
3. `set_document_id(markdown_path, document_id)` → Charger ou créer un `SyncMetadata`, setter le `document_id`, appeler `save_metadata()`.

**Ajouter** `.nou/` au `.gitignore` du projet (si un `.gitignore` existe, sinon en créer un).

---

### Étape 2 — Rendu Markdown : `markdown.rs::render()`

Module indépendant. Convertit `Vec<MdNode>` en `String` Markdown suivant les conventions REQ-009.

**Implémenter dans** [src/markdown.rs](src/markdown.rs) la fonction `render()` :

- Parcourir chaque `MdNode` et produire le texte Markdown correspondant
- Conventions de formatage :
  - Titres : `# `, `## `, etc.
  - Listes non-ordonnées : `- item`
  - Listes ordonnées : `1. item` (numérotation séquentielle à partir de `start`)
  - Un saut de ligne vide entre chaque bloc
  - Trailing newline en fin de document
- Fonction auxiliaire `render_inline(inlines: &[MdInline])` → `String` :
  - `Text(s)` → `s`
  - `Bold(inner)` → `**{render_inline(inner)}**`
  - `Italic(inner)` → `*{render_inline(inner)}*`
  - `Link { text, url }` → `[text](url)`
  - `Code(s)` → `` `s` ``
  - `LineBreak` → `\n` (ou deux espaces + newline si préféré)
- Listes imbriquées : indenter avec 2 espaces par niveau de profondeur
- Blocs de code : ` ```language\ncode\n``` `

Laisser `parse()` comme `todo!()` — non nécessaire pour le pull.

---

### Étape 3 — Conversion Google Docs → Markdown : `converter.rs::gdoc_to_markdown()`

Dépend de l'étape 2 (utilise les types `MdNode`/`MdInline`).

**Implémenter dans** [src/converter.rs](src/converter.rs) la fonction `gdoc_to_markdown()` :

La structure d'un `google_docs1::api::Document` :
- `document.body.content` → `Vec<StructuralElement>`
- Chaque `StructuralElement` a un champ `paragraph: Option<Paragraph>`
- Chaque `Paragraph` a `elements: Vec<ParagraphElement>` et `paragraph_style: Option<ParagraphStyle>`
- `ParagraphStyle.named_style_type` → `"HEADING_1"` à `"HEADING_6"`, `"NORMAL_TEXT"`, etc.
- Chaque `ParagraphElement` a `text_run: Option<TextRun>` avec `content: String` et `text_style: Option<TextStyle>`
- `TextStyle` : `bold: Option<bool>`, `italic: Option<bool>`, `link: Option<Link>`, etc.

**Algorithme :**

1. Extraire `body.content` (ou retourner un vecteur vide si le body est absent)
2. Pour chaque `StructuralElement` contenant un `Paragraph` :
   a. Lire `paragraph_style.named_style_type` pour déterminer le type (heading vs paragraphe)
   b. Pour chaque `ParagraphElement` contenant un `TextRun` :
      - Ignorer le `\n` final de chaque paragraphe (Google Docs en ajoute un)
      - Lire `text_style` pour détecter `bold`, `italic`, `link`
      - Créer l'`MdInline` approprié (avec imbrication `Bold(Italic(...))` si applicable)
   c. Construire le `MdNode` correspondant (`Heading` ou `Paragraph`)
3. Ignorer les éléments non supportés (tables, images, etc.) et les ajouter à `losses`
4. Retourner `ConversionResult { result: Vec<MdNode>, losses }`

**Gestion des listes :** Les listes Google Docs ne sont pas des éléments structurels distincts — ce sont des paragraphes avec `bullet: Some(Bullet)` dans le `Paragraph`. Il faudra :
- Détecter les paragraphes consécutifs avec un `bullet`
- Les regrouper en `UnorderedList` ou `OrderedList` selon le type de liste
- Utiliser `bullet.nesting_level` pour la profondeur
- Le `list_id` et les propriétés de liste dans `document.lists` permettent de distinguer les types

Laisser `markdown_to_gdoc_requests()` comme `todo!()`.

---

### Étape 4 — CLI : `cli.rs` et `main.rs`

Dépend de l'étape 1 (persistance pour résolution fichier optionnel).

**Modifications dans** [src/cli.rs](src/cli.rs) :

1. Rendre `fichier` optionnel dans `Pull` : `fichier: Option<PathBuf>`
2. Ajouter l'alias `-f` à `--force` dans `Pull` : `#[arg(short = 'f', long, ...)]`
3. Ne pas toucher à `Push` et `Status` pour l'instant

**Modifications dans** [src/main.rs](src/main.rs) :

1. Dans le bras `Command::Pull`, résoudre le fichier :
   - Si `fichier` est `Some(path)` → utiliser `path`
   - Si `fichier` est `None` → charger la dernière association via `mapping::load_metadata()` sur le fichier courant (lire `.nou/state.json` pour trouver le dernier fichier utilisé)
   - Si aucun fichier trouvé → afficher une erreur claire et quitter
2. Ajuster le `info!()` et l'appel à `sync::pull()`

**Note :** Pour résoudre le « fichier courant » quand aucun argument n'est fourni, ajouter une entrée spéciale `_current` dans le state.json, ou un champ `current_file` au niveau racine. Exemple :

```json
{
  "current_file": "chemin/relatif/fichier.md",
  "files": { ... }
}
```

Ajouter dans `mapping.rs` :
- `get_current_file()` → `Result<Option<PathBuf>>`
- `set_current_file(path)` → `Result<()>`

---

### Étape 5 — Orchestration du pull : `sync.rs::pull()`

Dépend des étapes 1, 2, 3, 4.

**Implémenter dans** [src/sync.rs](src/sync.rs) la fonction `pull()` :

1. **Résoudre le `doc_id`** :
   - Si fourni en argument → l'utiliser
   - Sinon → `mapping::get_document_id(fichier)?`
   - Si toujours `None` → retourner `SyncError::NoMapping`

2. **Initialiser le client Google Docs** :
   - Lire le chemin du compte de service depuis `std::env::var("SERVICE_ACCOUNT_KEY_PATH")` (ou `.env`)
   - Créer `GoogleDocsClient::new(path).await?`

3. **Lire le document distant** :
   - `client.get_document(doc_id).await?`

4. **Convertir en Markdown** :
   - `converter::gdoc_to_markdown(&document)?`
   - `markdown::render(&conversion_result.result)`

5. **Vérification simplifiée des conflits** :
   - Si le fichier local existe ET qu'une métadonnée `last_sync` existe :
     - Comparer la date de modification du fichier local (`fs::metadata(fichier)?.modified()`) avec `last_sync`
     - Si le fichier local est plus récent que `last_sync` et `!force` → retourner `SyncError::Conflict` (avec message adapté, pas besoin de `remote_modified` pour l'instant)
   - Si le fichier n'existe pas → pas de conflit (premier pull)

6. **Écrire le fichier Markdown** :
   - `fs::write(fichier, &markdown_content)?`
   - Créer les répertoires parents si nécessaire

7. **Sauvegarder les métadonnées** :
   - Créer/mettre à jour `SyncMetadata` avec `last_sync = now()`, `document_id`, `markdown_path`
   - `mapping::save_metadata(&metadata)?`
   - `mapping::set_current_file(fichier)?`
   - Si `doc_id` était fourni en argument → `mapping::set_document_id(fichier, doc_id)?`

8. **Afficher un rapport minimal** :
   - `info!("Pull réussi : {fichier}")` avec le nombre de blocs convertis
   - Si `losses` non vide → `warn!()` pour chaque perte

Laisser `push()` et `status()` comme `todo!()`.

---

### Étape 6 — Tests unitaires

Dépend des étapes 1, 2, 3.

**Créer** `tests/` ou utiliser `#[cfg(test)] mod tests` dans chaque module.

1. **Test conversion** (dans `converter.rs` ou `tests/converter_test.rs`) :
   - Construire un `Document` minimal en Rust (avec un paragraphe, un heading, du texte bold/italic)
   - Appeler `gdoc_to_markdown()` et vérifier les `MdNode` retournés
   - Vérifier que le nombre de `losses` est 0 pour un document simple

2. **Test render** (dans `markdown.rs` ou `tests/markdown_test.rs`) :
   - Construire des `Vec<MdNode>` représentant différents contenus
   - Appeler `render()` et vérifier le Markdown produit
   - Appeler `render()` deux fois et vérifier l'identité (stabilité/déterminisme, REQ-009)

3. **Test persistance** (dans `mapping.rs` ou `tests/mapping_test.rs`) :
   - Créer un `SyncMetadata`, appeler `save_metadata()`, puis `load_metadata()`, vérifier l'égalité
   - Tester `set_document_id()` puis `get_document_id()`
   - Utiliser un répertoire temporaire (`tempfile` crate) pour isoler les tests

**Ajouter la dépendance** `tempfile` dans `[dev-dependencies]` de `Cargo.toml`.

---

### Étape 7 — Vérification de compilation et `.gitignore`

1. Ajouter `.nou/` au `.gitignore`
2. `cargo check` — vérifier qu'il n'y a pas d'erreur de compilation
3. `cargo test` — vérifier que les tests passent
4. `cargo clippy` — vérifier les warnings
5. Tester manuellement si un compte de service est disponible : `cargo run -- pull test.md --doc-id <un-doc-id>`

---

## Vérification

1. **Compilation** : `cargo check` sans erreur
2. **Tests** : `cargo test` — tous les tests passent
3. **Linting** : `cargo clippy` — pas de warning
4. **Test manuel** (si compte de service disponible) :
   - `cargo run -- pull test.md --doc-id <DOC_ID>` crée `test.md` avec le contenu du Google Doc
   - `cargo run -- pull` (sans argument) utilise la persistance
   - Modifier `test.md`, puis `cargo run -- pull` → refus (fichier local plus récent)
   - `cargo run -- pull --force` → écrase le fichier local
5. **Stabilité** : deux pulls consécutifs du même document produisent un fichier identique

## Décisions

- **Format de persistance** : `.nou/state.json` centralisé plutôt que `.nou.json` à côté de chaque fichier — plus simple, un seul fichier à gérer
- **Champ `current_file`** : ajouté au JSON racine pour permettre `nou pull` sans argument
- **Listes Google Docs** : regroupement des paragraphes avec `bullet` — complexité moyenne, mais nécessaire pour un rendu correct
- **Erreur de conflit simplifiée** : on adapte `SyncError::Conflict` pour fonctionner avec uniquement la date locale vs `last_sync` (pas besoin de `remote_modified` dans cette ronde)
- **`parse()` reste `todo!()`** : non nécessaire pour le pull, sera implémenté avec le push
- **`tempfile` en dev-dependency** : nécessaire pour les tests de persistance sans polluer le filesystem
