#![allow(dead_code)]
//! Association fichier Markdown ↔ Google Doc et métadonnées de synchronisation
//!
//! Ce module gère :
//! - L'association persistante entre un fichier Markdown local et un Google Doc distant (REQ-003)
//! - Les métadonnées de synchronisation (horodatages, hashes) pour la détection de conflits (REQ-006)

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

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

/// Charge les métadonnées de synchronisation pour un fichier Markdown
///
/// Les métadonnées sont stockées dans un fichier `.nou.json` à côté du fichier Markdown,
/// ou dans un répertoire `.nou/` centralisé.
pub fn load_metadata(_markdown_path: &Path) -> Result<Option<SyncMetadata>> {
    todo!("Implémenter le chargement des métadonnées de synchronisation")
}

/// Sauvegarde les métadonnées de synchronisation
pub fn save_metadata(_metadata: &SyncMetadata) -> Result<()> {
    todo!("Implémenter la sauvegarde des métadonnées de synchronisation")
}

/// Récupère l'identifiant du Google Doc associé à un fichier Markdown
///
/// Retourne `None` si aucune association n'existe.
pub fn get_document_id(markdown_path: &Path) -> Result<Option<String>> {
    let metadata = load_metadata(markdown_path)?;
    Ok(metadata.map(|m| m.document_id))
}

/// Crée ou met à jour l'association entre un fichier Markdown et un Google Doc
pub fn set_document_id(_markdown_path: &Path, _document_id: &str) -> Result<()> {
    todo!("Implémenter la création/mise à jour de l'association")
}
