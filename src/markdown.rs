#![allow(dead_code)]
//! Parsage et génération de Markdown (pulldown-cmark)
//!
//! Ce module fournit les fonctions pour :
//! - Parser un fichier Markdown en une représentation intermédiaire
//! - Générer du Markdown stable et déterministe à partir d'une représentation intermédiaire
//!
//! Le Markdown est le « plus petit dénominateur commun » — il représente le contenu
//! sans perte sémantique, mais sans le style visuel propre à Google Docs.

use anyhow::Result;

// Les imports pulldown-cmark seront activés lors de l'implémentation
// use pulldown_cmark::{Event, Parser, Tag, TagEnd, HeadingLevel};

/// Représentation intermédiaire d'un document Markdown
///
/// Structure arborescente qui capture la sémantique du document
/// sans les détails de formatage textuel.
#[derive(Debug, Clone, PartialEq)]
pub enum MdNode {
    /// Titre (niveau 1-6)
    Heading { level: u8, content: Vec<MdInline> },
    /// Paragraphe
    Paragraph { content: Vec<MdInline> },
    /// Liste non-ordonnée
    UnorderedList { items: Vec<Vec<MdNode>> },
    /// Liste ordonnée
    OrderedList { start: u64, items: Vec<Vec<MdNode>> },
    /// Bloc de code
    CodeBlock { language: Option<String>, code: String },
    /// Ligne horizontale
    HorizontalRule,
}

/// Éléments inline (à l'intérieur d'un paragraphe ou titre)
#[derive(Debug, Clone, PartialEq)]
pub enum MdInline {
    /// Texte brut
    Text(String),
    /// Texte en gras
    Bold(Vec<MdInline>),
    /// Texte en italique
    Italic(Vec<MdInline>),
    /// Lien
    Link { text: String, url: String },
    /// Code inline
    Code(String),
    /// Saut de ligne
    LineBreak,
}

/// Parse un fichier Markdown en représentation intermédiaire
pub fn parse(_markdown: &str) -> Result<Vec<MdNode>> {
    todo!("Implémenter le parsage Markdown → MdNode via pulldown-cmark")
}

/// Génère du Markdown stable et déterministe à partir de la représentation intermédiaire
///
/// Conventions de formatage (REQ-009) :
/// - Titres avec `#` (pas de soulignement)
/// - Listes non-ordonnées avec `-`
/// - Emphase avec `*italique*` et `**gras**`
/// - Un saut de ligne entre chaque bloc
/// - Trailing newline en fin de fichier
pub fn render(_nodes: &[MdNode]) -> String {
    todo!("Implémenter la génération MdNode → Markdown")
}
