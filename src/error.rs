#![allow(dead_code)]
use thiserror::Error;

/// Erreurs spécifiques à la synchronisation Markdown ↔ Google Docs
#[derive(Error, Debug)]
pub enum SyncError {
    /// Conflit détecté : les deux côtés ont été modifiés
    #[error(
        "Conflit détecté : le fichier Markdown et le Google Doc ont été modifiés depuis la dernière synchronisation.\nDernière sync : {last_sync}\nModification locale : {local_modified}\nModification distante : {remote_modified}\nUtilisez --force pour écraser."
    )]
    Conflict {
        last_sync: String,
        local_modified: String,
        remote_modified: String,
    },

    /// Aucune association trouvée pour ce fichier (utilisé par sync.rs)
    #[error("Aucune association trouvée pour ce fichier Markdown. Utilisez --doc-id pour spécifier le Google Doc.")]
    NoDocId,

    /// Aucune association de mapping trouvée pour ce fichier (utilisé par pull)
    #[error("Aucune association trouvée pour '{path}'. Utilisez --doc-id pour spécifier le Google Doc.")]
    NoMapping { path: String },

    /// Le fichier Markdown n'existe pas
    #[error("Le fichier '{path}' n'existe pas.")]
    FileNotFound { path: String },

    /// Erreur de transport API Google
    #[error("Erreur API Google : {message}")]
    ApiError { message: String },

    /// Perte d'information détectée lors de la conversion
    #[error("Perte d'information détectée : {details}")]
    InformationLoss { details: String },
}
