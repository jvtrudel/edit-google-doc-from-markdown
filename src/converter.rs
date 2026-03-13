#![allow(dead_code)]
//! Conversion bidirectionnelle Markdown ↔ structure Google Docs
//!
//! Ce module convertit entre la représentation intermédiaire Markdown (MdNode)
//! et la structure Google Docs (google_docs1::api::Document).
//!
//! La conversion porte sur le **contenu** uniquement.
//! Le style est géré séparément par le module `style`.

use anyhow::Result;
use google_docs1::api::Document;

use crate::markdown::MdNode;

/// Résultat d'une conversion avec les éventuelles pertes d'information
pub struct ConversionResult<T> {
    /// Le résultat de la conversion
    pub result: T,
    /// Les pertes d'information détectées
    pub losses: Vec<InformationLoss>,
}

/// Description d'une perte d'information lors de la conversion
#[derive(Debug, Clone)]
pub struct InformationLoss {
    /// Type de perte (contenu ou style)
    pub kind: LossKind,
    /// Description de l'élément perdu
    pub description: String,
    /// Position approximative dans le document source
    pub position: Option<String>,
}

/// Type de perte d'information
#[derive(Debug, Clone, PartialEq)]
pub enum LossKind {
    /// Information de contenu manquante (ex: image, commentaire)
    Content,
    /// Information de style non représentable (ex: couleur, police)
    Style,
}

/// Convertit une représentation Markdown en requêtes Google Docs batchUpdate
///
/// Retourne les requêtes nécessaires pour recréer le contenu du document.
pub fn markdown_to_gdoc_requests(
    _nodes: &[MdNode],
) -> Result<ConversionResult<Vec<google_docs1::api::Request>>> {
    todo!("Implémenter la conversion MdNode → requêtes Google Docs batchUpdate")
}

/// Convertit un document Google Docs en représentation Markdown intermédiaire
///
/// Extrait le contenu sémantique du document. Le style est ignoré ici
/// (il est extrait séparément par le module `style`).
pub fn gdoc_to_markdown(
    _document: &Document,
) -> Result<ConversionResult<Vec<MdNode>>> {
    todo!("Implémenter la conversion Document Google Docs → MdNode")
}
