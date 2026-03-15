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
    CodeBlock {
        language: Option<String>,
        code: String,
    },
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
pub fn render(nodes: &[MdNode]) -> String {
    let mut output = String::new();
    for (i, node) in nodes.iter().enumerate() {
        if i > 0 {
            output.push('\n');
        }
        render_node(node, &mut output, 0);
        output.push('\n');
    }
    output
}

/// Rend un nœud Markdown en texte, avec le niveau d'indentation donné
fn render_node(node: &MdNode, output: &mut String, indent: usize) {
    let prefix = "  ".repeat(indent);
    match node {
        MdNode::Heading { level, content } => {
            for _ in 0..*level {
                output.push('#');
            }
            output.push(' ');
            output.push_str(&render_inlines(content));
        }
        MdNode::Paragraph { content } => {
            output.push_str(&prefix);
            output.push_str(&render_inlines(content));
        }
        MdNode::UnorderedList { items } => {
            render_list_items(items, output, indent, None);
        }
        MdNode::OrderedList { start, items } => {
            render_list_items(items, output, indent, Some(*start));
        }
        MdNode::CodeBlock { language, code } => {
            output.push_str(&prefix);
            output.push_str("```");
            if let Some(lang) = language {
                output.push_str(lang);
            }
            output.push('\n');
            // Ajouter le préfixe d'indentation à chaque ligne du code
            for line in code.lines() {
                output.push_str(&prefix);
                output.push_str(line);
                output.push('\n');
            }
            // Si le code ne se termine pas par un newline, s'assurer qu'on en a un
            if !code.is_empty() && !code.ends_with('\n') {
                // Le dernier line() a déjà été ajouté avec \n
            }
            output.push_str(&prefix);
            output.push_str("```");
        }
        MdNode::HorizontalRule => {
            output.push_str(&prefix);
            output.push_str("---");
        }
    }
}

/// Rend les éléments d'une liste (ordonnée ou non)
fn render_list_items(
    items: &[Vec<MdNode>],
    output: &mut String,
    indent: usize,
    ordered_start: Option<u64>,
) {
    let prefix = "  ".repeat(indent);
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            output.push('\n');
        }
        let marker = match ordered_start {
            Some(start) => format!("{}. ", start + i as u64),
            None => "- ".to_string(),
        };
        for (j, node) in item.iter().enumerate() {
            if j == 0 {
                // Premier nœud de l'item : précédé du marqueur
                output.push_str(&prefix);
                output.push_str(&marker);
                match node {
                    MdNode::Paragraph { content } => {
                        output.push_str(&render_inlines(content));
                    }
                    // Sous-liste ou autre bloc : rendre avec indentation
                    _ => {
                        output.push('\n');
                        render_node(node, output, indent + 1);
                    }
                }
            } else {
                // Nœuds suivants dans le même item (sous-listes, etc.)
                output.push('\n');
                render_node(node, output, indent + 1);
            }
        }
    }
}

/// Convertit des éléments inline en texte Markdown
fn render_inlines(inlines: &[MdInline]) -> String {
    let mut output = String::new();
    for inline in inlines {
        match inline {
            MdInline::Text(s) => output.push_str(s),
            MdInline::Bold(inner) => {
                output.push_str("**");
                output.push_str(&render_inlines(inner));
                output.push_str("**");
            }
            MdInline::Italic(inner) => {
                output.push('*');
                output.push_str(&render_inlines(inner));
                output.push('*');
            }
            MdInline::Link { text, url } => {
                output.push('[');
                output.push_str(text);
                output.push_str("](");
                output.push_str(url);
                output.push(')');
            }
            MdInline::Code(s) => {
                output.push('`');
                output.push_str(s);
                output.push('`');
            }
            MdInline::LineBreak => {
                output.push_str("  \n");
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_heading() {
        let nodes = vec![MdNode::Heading {
            level: 2,
            content: vec![MdInline::Text("Mon titre".to_string())],
        }];
        assert_eq!(render(&nodes), "## Mon titre\n");
    }

    #[test]
    fn test_render_paragraph() {
        let nodes = vec![MdNode::Paragraph {
            content: vec![MdInline::Text("Hello world".to_string())],
        }];
        assert_eq!(render(&nodes), "Hello world\n");
    }

    #[test]
    fn test_render_bold_italic() {
        let nodes = vec![MdNode::Paragraph {
            content: vec![
                MdInline::Text("Normal ".to_string()),
                MdInline::Bold(vec![MdInline::Text("gras".to_string())]),
                MdInline::Text(" et ".to_string()),
                MdInline::Italic(vec![MdInline::Text("italique".to_string())]),
            ],
        }];
        assert_eq!(render(&nodes), "Normal **gras** et *italique*\n");
    }

    #[test]
    fn test_render_link() {
        let nodes = vec![MdNode::Paragraph {
            content: vec![MdInline::Link {
                text: "Google".to_string(),
                url: "https://google.com".to_string(),
            }],
        }];
        assert_eq!(render(&nodes), "[Google](https://google.com)\n");
    }

    #[test]
    fn test_render_unordered_list() {
        let nodes = vec![MdNode::UnorderedList {
            items: vec![
                vec![MdNode::Paragraph {
                    content: vec![MdInline::Text("Item 1".to_string())],
                }],
                vec![MdNode::Paragraph {
                    content: vec![MdInline::Text("Item 2".to_string())],
                }],
            ],
        }];
        assert_eq!(render(&nodes), "- Item 1\n- Item 2\n");
    }

    #[test]
    fn test_render_ordered_list() {
        let nodes = vec![MdNode::OrderedList {
            start: 1,
            items: vec![
                vec![MdNode::Paragraph {
                    content: vec![MdInline::Text("Premier".to_string())],
                }],
                vec![MdNode::Paragraph {
                    content: vec![MdInline::Text("Deuxième".to_string())],
                }],
            ],
        }];
        assert_eq!(render(&nodes), "1. Premier\n2. Deuxième\n");
    }

    #[test]
    fn test_render_code_block() {
        let nodes = vec![MdNode::CodeBlock {
            language: Some("rust".to_string()),
            code: "fn main() {}".to_string(),
        }];
        assert_eq!(render(&nodes), "```rust\nfn main() {}\n```\n");
    }

    #[test]
    fn test_render_horizontal_rule() {
        let nodes = vec![MdNode::HorizontalRule];
        assert_eq!(render(&nodes), "---\n");
    }

    #[test]
    fn test_render_determinism() {
        let nodes = vec![
            MdNode::Heading {
                level: 1,
                content: vec![MdInline::Text("Titre".to_string())],
            },
            MdNode::Paragraph {
                content: vec![
                    MdInline::Text("Du texte avec du ".to_string()),
                    MdInline::Bold(vec![MdInline::Text("gras".to_string())]),
                    MdInline::Text(".".to_string()),
                ],
            },
            MdNode::UnorderedList {
                items: vec![
                    vec![MdNode::Paragraph {
                        content: vec![MdInline::Text("A".to_string())],
                    }],
                    vec![MdNode::Paragraph {
                        content: vec![MdInline::Text("B".to_string())],
                    }],
                ],
            },
        ];

        let first = render(&nodes);
        let second = render(&nodes);
        assert_eq!(first, second, "render() doit être déterministe (REQ-009)");
    }

    #[test]
    fn test_render_multiple_blocks_separated() {
        let nodes = vec![
            MdNode::Heading {
                level: 1,
                content: vec![MdInline::Text("Titre".to_string())],
            },
            MdNode::Paragraph {
                content: vec![MdInline::Text("Contenu".to_string())],
            },
        ];
        let result = render(&nodes);
        assert_eq!(result, "# Titre\n\nContenu\n");
    }
}
