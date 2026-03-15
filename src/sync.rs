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
use google_docs1::api::Document;
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

    // 1. Lire le fichier Markdown
    let contenu = fs::read_to_string(fichier)?;

    // 2. Parser le Markdown
    let nodes = markdown::parse(&contenu)?;

    // 3. Résoudre le doc_id
    let doc_id = match doc_id {
        Some(id) => id.to_string(),
        None => mapping::get_doc_id_for_file(fichier)?.ok_or(SyncError::NoDocId)?
    };

    // 4. Vérification de conflit simplifiée
    let metadata = mapping::get_metadata_for_file(fichier)?;
    let sa_key_path = std::env::var("SERVICE_ACCOUNT_KEY_PATH")
        .unwrap_or_else(|_| "service-account.json".to_string());
    let client = GoogleDocsClient::new(&sa_key_path).await?;
    let remote_doc = client.get_document(&doc_id).await?;
    let remote_last_modified = mapping::get_remote_last_modified(&remote_doc)?;
    if let Some(meta) = &metadata {
        if !force {
            let remote_ts = remote_last_modified.as_deref().unwrap_or("");
            let local_ts = meta.last_sync.as_deref().unwrap_or("");
            if !local_ts.is_empty() && remote_ts > local_ts {
                bail!("Conflit : le document distant a été modifié depuis la dernière synchronisation. Utilisez --force pour écraser.");
            }
        }
    }

    // 5. Conversion en requêtes Google Docs
    let doc_end_index = get_body_end_index(&remote_doc);
    let conversion = converter::markdown_to_gdoc_requests(&nodes, doc_end_index)?;
    let requests = conversion.result;

    // 6. Envoi via batch_update
    client.batch_update(&doc_id, requests).await?;

    // 7. Sauvegarde des métadonnées de synchronisation
    let now = Utc::now();
    mapping::save_metadata_for_file(fichier, SyncMetadata {
        markdown_path: String::new(), // écrasé par save_metadata_for_file
        document_id: doc_id.clone(),
        last_sync: Some(now.to_rfc3339()),
        last_markdown_hash: None,
        last_revision_id: None,
    })?;

    info!("Push terminé : {} → {}", fichier.display(), doc_id);
    Ok(())
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

/// Retourne l'index de fin du corps du document (pour calculer la plage de suppression)
fn get_body_end_index(document: &Document) -> i32 {
    document
        .body
        .as_ref()
        .and_then(|b| b.content.as_ref())
        .and_then(|c| c.last())
        .and_then(|e| e.end_index)
        .unwrap_or(1)
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
