#![allow(dead_code)]
//! Association fichier Markdown ↔ Google Doc et métadonnées de synchronisation
//!
//! Ce module gère :
//! - L'association persistante entre un fichier Markdown local et un Google Doc distant (REQ-003)
//! - Les métadonnées de synchronisation (horodatages, hashes) pour la détection de conflits (REQ-006)
//! - L'état persistant du fichier de travail courant (REQ-010)
//!
//! Les données sont stockées dans `.nou/state.json` au répertoire courant.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Chemin du fichier d'état persistant
const STATE_FILE: &str = ".nou/state.json";

/// État global de synchronisation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncState {
    /// Fichier Markdown courant (pour les commandes sans argument)
    pub current_file: Option<String>,
    /// Métadonnées de synchronisation par fichier
    pub files: HashMap<String, SyncMetadata>,
}

/// Métadonnées de synchronisation pour un fichier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncMetadata {
    /// Chemin du fichier Markdown (relatif au répertoire de travail)
    pub markdown_path: String,
    /// Identifiant du Google Doc associé
    pub document_id: String,
    /// Horodatage de la dernière synchronisation (ISO 8601)
    pub last_sync: Option<String>,
    /// Hash du contenu Markdown lors de la dernière synchronisation
    pub last_markdown_hash: Option<String>,
    /// Revision ID du Google Doc lors de la dernière synchronisation
    pub last_revision_id: Option<String>,
}

/// Retourne le chemin du fichier d'état à utiliser
fn state_file_path() -> PathBuf {
    PathBuf::from(STATE_FILE)
}

/// Charge l'état global depuis le fichier d'état
fn load_state() -> Result<SyncState> {
    load_state_from(&state_file_path())
}

/// Charge l'état global depuis un chemin donné
fn load_state_from(path: &Path) -> Result<SyncState> {
    if !path.exists() {
        return Ok(SyncState::default());
    }
    let content = fs::read_to_string(path).context("Impossible de lire le fichier d'état")?;
    let state: SyncState =
        serde_json::from_str(&content).context("Impossible de parser le fichier d'état")?;
    Ok(state)
}

/// Sauvegarde l'état global dans le fichier d'état
fn save_state(state: &SyncState) -> Result<()> {
    save_state_to(state, &state_file_path())
}

/// Sauvegarde l'état global dans un chemin donné
fn save_state_to(state: &SyncState, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .context("Impossible de créer le répertoire parent du fichier d'état")?;
    }
    let content = serde_json::to_string_pretty(state).context("Impossible de sérialiser l'état")?;
    fs::write(path, content).context("Impossible d'écrire le fichier d'état")?;
    Ok(())
}

/// Normalise un chemin en chaîne relative pour servir de clé
fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

/// Charge les métadonnées de synchronisation pour un fichier Markdown
///
/// Retourne `Ok(None)` si aucune entrée n'existe pour ce fichier.
pub fn load_metadata(markdown_path: &Path) -> Result<Option<SyncMetadata>> {
    let state = load_state()?;
    let key = normalize_path(markdown_path);
    Ok(state.files.get(&key).cloned())
}

/// Sauvegarde les métadonnées de synchronisation pour un fichier
pub fn save_metadata(metadata: &SyncMetadata) -> Result<()> {
    let mut state = load_state()?;
    let key = metadata.markdown_path.clone();
    state.files.insert(key, metadata.clone());
    save_state(&state)
}

/// Récupère l'identifiant du Google Doc associé à un fichier Markdown
///
/// Retourne `None` si aucune association n'existe.
pub fn get_document_id(markdown_path: &Path) -> Result<Option<String>> {
    let metadata = load_metadata(markdown_path)?;
    Ok(metadata.map(|m| m.document_id))
}

/// Crée ou met à jour l'association entre un fichier Markdown et un Google Doc
pub fn set_document_id(markdown_path: &Path, document_id: &str) -> Result<()> {
    let mut state = load_state()?;
    let key = normalize_path(markdown_path);
    match state.files.get_mut(&key) {
        Some(metadata) => {
            metadata.document_id = document_id.to_string();
        }
        None => {
            state.files.insert(
                key.clone(),
                SyncMetadata {
                    markdown_path: key,
                    document_id: document_id.to_string(),
                    last_sync: None,
                    last_markdown_hash: None,
                    last_revision_id: None,
                },
            );
        }
    }
    save_state(&state)
}

/// Récupère le chemin du fichier courant (pour les commandes sans argument)
pub fn get_current_file() -> Result<Option<PathBuf>> {
    let state = load_state()?;
    Ok(state.current_file.map(PathBuf::from))
}

/// Définit le fichier courant
pub fn set_current_file(path: &Path) -> Result<()> {
    let mut state = load_state()?;
    state.current_file = Some(normalize_path(path));
    save_state(&state)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Crée un chemin de state dans un répertoire temporaire
    fn temp_state_path(dir: &tempfile::TempDir) -> PathBuf {
        dir.path().join(".nou").join("state.json")
    }

    #[test]
    fn test_save_load_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let state_path = temp_state_path(&dir);

        let metadata = SyncMetadata {
            markdown_path: "test.md".to_string(),
            document_id: "doc-123".to_string(),
            last_sync: Some("2026-03-14T10:00:00Z".to_string()),
            last_markdown_hash: None,
            last_revision_id: Some("rev-1".to_string()),
        };

        let mut state = SyncState::default();
        state.files.insert("test.md".to_string(), metadata);
        save_state_to(&state, &state_path).unwrap();

        let loaded = load_state_from(&state_path).unwrap();
        let loaded_meta = loaded.files.get("test.md").unwrap();

        assert_eq!(loaded_meta.markdown_path, "test.md");
        assert_eq!(loaded_meta.document_id, "doc-123");
        assert_eq!(
            loaded_meta.last_sync,
            Some("2026-03-14T10:00:00Z".to_string())
        );
        assert_eq!(loaded_meta.last_revision_id, Some("rev-1".to_string()));
    }

    #[test]
    fn test_load_nonexistent_returns_empty() {
        let dir = tempfile::tempdir().unwrap();
        let state_path = temp_state_path(&dir);

        let state = load_state_from(&state_path).unwrap();
        assert!(state.files.is_empty());
        assert!(state.current_file.is_none());
    }

    #[test]
    fn test_multiple_files_in_state() {
        let dir = tempfile::tempdir().unwrap();
        let state_path = temp_state_path(&dir);

        let mut state = SyncState::default();

        state.files.insert(
            "a.md".to_string(),
            SyncMetadata {
                markdown_path: "a.md".to_string(),
                document_id: "doc-a".to_string(),
                last_sync: None,
                last_markdown_hash: None,
                last_revision_id: None,
            },
        );
        state.files.insert(
            "b.md".to_string(),
            SyncMetadata {
                markdown_path: "b.md".to_string(),
                document_id: "doc-b".to_string(),
                last_sync: None,
                last_markdown_hash: None,
                last_revision_id: None,
            },
        );
        state.current_file = Some("b.md".to_string());

        save_state_to(&state, &state_path).unwrap();
        let loaded = load_state_from(&state_path).unwrap();

        assert_eq!(loaded.files.len(), 2);
        assert_eq!(loaded.files["a.md"].document_id, "doc-a");
        assert_eq!(loaded.files["b.md"].document_id, "doc-b");
        assert_eq!(loaded.current_file, Some("b.md".to_string()));
    }

    #[test]
    fn test_update_existing_entry() {
        let dir = tempfile::tempdir().unwrap();
        let state_path = temp_state_path(&dir);

        let mut state = SyncState::default();
        state.files.insert(
            "doc.md".to_string(),
            SyncMetadata {
                markdown_path: "doc.md".to_string(),
                document_id: "id-1".to_string(),
                last_sync: None,
                last_markdown_hash: None,
                last_revision_id: None,
            },
        );
        save_state_to(&state, &state_path).unwrap();

        // Mettre à jour le document_id
        let mut state = load_state_from(&state_path).unwrap();
        state.files.get_mut("doc.md").unwrap().document_id = "id-2".to_string();
        save_state_to(&state, &state_path).unwrap();

        let loaded = load_state_from(&state_path).unwrap();
        assert_eq!(loaded.files["doc.md"].document_id, "id-2");
    }

    #[test]
    fn test_state_json_format() {
        let dir = tempfile::tempdir().unwrap();
        let state_path = temp_state_path(&dir);

        let mut state = SyncState::default();
        state.current_file = Some("test.md".to_string());
        state.files.insert(
            "test.md".to_string(),
            SyncMetadata {
                markdown_path: "test.md".to_string(),
                document_id: "abc".to_string(),
                last_sync: None,
                last_markdown_hash: None,
                last_revision_id: None,
            },
        );
        save_state_to(&state, &state_path).unwrap();

        // Vérifier que le fichier est du JSON valide et lisible
        let content = fs::read_to_string(&state_path).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed["current_file"], "test.md");
    }
}
