//! Logique de synchronisation : push, pull, détection de conflits
//!
//! Ce module orchestre les opérations de synchronisation en coordonnant :
//! - Le transport (google_docs)
//! - La conversion (converter)
//! - Le style (style)
//! - L'association fichier ↔ document (mapping)

use anyhow::{Result, bail};
use chrono::Utc;
use std::fs;
use std::path::Path;
use tracing::{info, warn};

use crate::converter;
use crate::error::SyncError;
use crate::google_docs::GoogleDocsClient;
use crate::mapping::{self, SyncMetadata};
use crate::markdown;

/// Publie un fichier Markdown vers un Google Doc (FEAT-001)
///
/// Étapes :
/// 1. Vérifier l'association fichier ↔ document (REQ-003)
/// 2. Détecter les conflits (REQ-006)
/// 3. Convertir le contenu Markdown → requêtes Google Docs (REQ-001)
/// 4. Gérer le style (REQ-005)
/// 5. Envoyer via l'API (REQ-008)
/// 6. Signaler les pertes d'information (REQ-004)
pub async fn push(fichier: &Path, doc_id: Option<&str>, force: bool) -> Result<()> {
    info!(
        "Push: {} → Google Doc (doc_id={:?}, force={})",
        fichier.display(),
        doc_id,
        force
    );
    todo!("Implémenter le push Markdown → Google Doc")
}

/// Récupère un Google Doc en fichier Markdown (FEAT-002)
///
/// Étapes :
/// 1. Résoudre le doc_id (argument ou persistance)
/// 2. Lire le document via l'API (REQ-008)
/// 3. Convertir le contenu Google Docs → Markdown (REQ-002)
/// 4. Vérification simplifiée des conflits
/// 5. Écrire le fichier Markdown (REQ-009 — stabilité)
/// 6. Sauvegarder les métadonnées de synchronisation
pub async fn pull(fichier: &Path, doc_id: Option<&str>, force: bool) -> Result<()> {
    info!(
        "Pull: Google Doc → {} (doc_id={:?}, force={})",
        fichier.display(),
        doc_id,
        force
    );

    // 1. Résoudre le doc_id
    let resolved_doc_id = match doc_id {
        Some(id) => id.to_string(),
        None => match mapping::get_document_id(fichier)? {
            Some(id) => id,
            None => bail!(SyncError::NoMapping {
                path: fichier.display().to_string(),
            }),
        },
    };
    info!("Document ID résolu : {}", resolved_doc_id);

    // 2. Initialiser le client Google Docs
    let sa_key_path = std::env::var("SERVICE_ACCOUNT_KEY_PATH")
        .unwrap_or_else(|_| "service-account.json".to_string());
    let client = GoogleDocsClient::new(&sa_key_path).await?;

    // 3. Lire le document distant
    info!("Lecture du document Google Docs...");
    let document = client.get_document(&resolved_doc_id).await?;
    let revision_id = document.revision_id.clone();

    // 4. Convertir en Markdown
    info!("Conversion Google Docs → Markdown...");
    let conversion = converter::gdoc_to_markdown(&document)?;
    let markdown_content = markdown::render(&conversion.result);

    // Signaler les pertes d'information
    for loss in &conversion.losses {
        warn!(
            "Perte d'information : {} ({:?})",
            loss.description, loss.kind
        );
    }

    // 5. Vérification simplifiée des conflits
    if fichier.exists()
        && !force
        && let Some(metadata) = mapping::load_metadata(fichier)?
        && let Some(last_sync_str) = &metadata.last_sync
        && let Ok(last_sync) = chrono::DateTime::parse_from_rfc3339(last_sync_str)
    {
        let file_modified = fs::metadata(fichier)?.modified()?;
        let file_modified_dt: chrono::DateTime<Utc> = file_modified.into();

        if file_modified_dt > last_sync {
            bail!(SyncError::Conflict {
                last_sync: last_sync_str.clone(),
                local_modified: file_modified_dt.to_rfc3339(),
                remote_modified: "non vérifié".to_string(),
            });
        }
    }

    // 6. Écrire le fichier Markdown
    if let Some(parent) = fichier.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    fs::write(fichier, &markdown_content)?;
    info!("Fichier écrit : {}", fichier.display());

    // 7. Sauvegarder les métadonnées
    let now = Utc::now().to_rfc3339();
    let metadata = SyncMetadata {
        markdown_path: fichier.to_string_lossy().to_string(),
        document_id: resolved_doc_id.clone(),
        last_sync: Some(now),
        last_markdown_hash: None,
        last_revision_id: revision_id,
    };
    mapping::save_metadata(&metadata)?;
    mapping::set_current_file(fichier)?;

    // Si doc_id était fourni en argument, s'assurer que l'association est persistée
    if doc_id.is_some() {
        mapping::set_document_id(fichier, &resolved_doc_id)?;
    }

    info!(
        "Pull réussi : {} ({} bloc(s) convertis, {} perte(s))",
        fichier.display(),
        conversion.result.len(),
        conversion.losses.len()
    );

    Ok(())
}

/// Affiche l'état de synchronisation d'un fichier (FEAT-003)
///
/// Informations affichées :
/// - Association fichier ↔ document
/// - Dates de dernière modification (locale et distante)
/// - État : synchronisé / modifié localement / modifié à distance / conflit
pub async fn status(fichier: &Path) -> Result<()> {
    info!("Status: {}", fichier.display());
    todo!("Implémenter l'affichage du statut de synchronisation")
}
