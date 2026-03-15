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

use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};

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

/// Contexte de l'inline stack pour le parsage
enum InlineCtx {
    Paragraph,
    Heading(u8),
    Item,
    Bold,
    Italic,
    Link(String),
}

/// Collecte le texte brut d'une liste d'inlines (pour les liens)
fn collect_text(inlines: &[MdInline]) -> String {
    let mut s = String::new();
    for inline in inlines {
        match inline {
            MdInline::Text(t) => s.push_str(t),
            MdInline::Bold(inner) | MdInline::Italic(inner) => s.push_str(&collect_text(inner)),
            MdInline::Link { text, .. } => s.push_str(text),
            MdInline::Code(c) => s.push_str(c),
            MdInline::LineBreak => s.push(' '),
        }
    }
    s
}

/// Parse un fichier Markdown en représentation intermédiaire
pub fn parse(markdown: &str) -> Result<Vec<MdNode>> {
    let parser = Parser::new(markdown);
    let mut nodes = Vec::new();

    // Stack de contextes inline imbriqués : (contexte, inlines_accumulés)
    let mut inline_stack: Vec<(InlineCtx, Vec<MdInline>)> = Vec::new();

    // Stack pour les listes : (ordered, start, items)
    let mut list_stack: Vec<(bool, u64, Vec<Vec<MdNode>>)> = Vec::new();

    // Bloc de code en cours
    let mut code_block: Option<(Option<String>, String)> = None;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    inline_stack.push((InlineCtx::Heading(level as u8), Vec::new()));
                }
                Tag::Paragraph => {
                    inline_stack.push((InlineCtx::Paragraph, Vec::new()));
                }
                Tag::List(Some(start)) => {
                    list_stack.push((true, start, Vec::new()));
                }
                Tag::List(None) => {
                    list_stack.push((false, 1, Vec::new()));
                }
                Tag::Item => {
                    inline_stack.push((InlineCtx::Item, Vec::new()));
                }
                Tag::CodeBlock(kind) => {
                    let lang = match kind {
                        CodeBlockKind::Fenced(s) => {
                            let s = s.trim().to_string();
                            if s.is_empty() { None } else { Some(s) }
                        }
                        CodeBlockKind::Indented => None,
                    };
                    code_block = Some((lang, String::new()));
                }
                Tag::Strong => {
                    inline_stack.push((InlineCtx::Bold, Vec::new()));
                }
                Tag::Emphasis => {
                    inline_stack.push((InlineCtx::Italic, Vec::new()));
                }
                Tag::Link { dest_url, .. } => {
                    inline_stack.push((InlineCtx::Link(dest_url.to_string()), Vec::new()));
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Heading(_) => {
                    if let Some((InlineCtx::Heading(level), content)) = inline_stack.pop() {
                        nodes.push(MdNode::Heading { level, content });
                    }
                }
                TagEnd::Paragraph => {
                    if let Some((InlineCtx::Paragraph, content)) = inline_stack.pop() {
                        // Si le parent est un Item, ajouter le contenu à l'item
                        if matches!(inline_stack.last(), Some((InlineCtx::Item, _))) {
                            if let Some((_, item_inlines)) = inline_stack.last_mut() {
                                item_inlines.extend(content);
                            }
                        } else if !content.is_empty() {
                            nodes.push(MdNode::Paragraph { content });
                        }
                    }
                }
                TagEnd::List(_) => {
                    if let Some((ordered, start, items)) = list_stack.pop() {
                        if !items.is_empty() {
                            if ordered {
                                nodes.push(MdNode::OrderedList { start, items });
                            } else {
                                nodes.push(MdNode::UnorderedList { items });
                            }
                        }
                    }
                }
                TagEnd::Item => {
                    if let Some((InlineCtx::Item, content)) = inline_stack.pop() {
                        if let Some((_, _, items)) = list_stack.last_mut() {
                            items.push(vec![MdNode::Paragraph { content }]);
                        }
                    }
                }
                TagEnd::CodeBlock => {
                    if let Some((lang, code)) = code_block.take() {
                        nodes.push(MdNode::CodeBlock { language: lang, code });
                    }
                }
                TagEnd::Strong => {
                    if let Some((InlineCtx::Bold, inner)) = inline_stack.pop() {
                        if let Some((_, parent)) = inline_stack.last_mut() {
                            parent.push(MdInline::Bold(inner));
                        }
                    }
                }
                TagEnd::Emphasis => {
                    if let Some((InlineCtx::Italic, inner)) = inline_stack.pop() {
                        if let Some((_, parent)) = inline_stack.last_mut() {
                            parent.push(MdInline::Italic(inner));
                        }
                    }
                }
                TagEnd::Link => {
                    if let Some((InlineCtx::Link(url), inner)) = inline_stack.pop() {
                        let text = collect_text(&inner);
                        if let Some((_, parent)) = inline_stack.last_mut() {
                            parent.push(MdInline::Link { text, url });
                        }
                    }
                }
                _ => {}
            },
            Event::Text(text) => {
                if let Some((_, ref mut code)) = code_block {
                    code.push_str(&text);
                } else if let Some((_, inlines)) = inline_stack.last_mut() {
                    inlines.push(MdInline::Text(text.to_string()));
                }
            }
            Event::Code(code) => {
                if let Some((_, inlines)) = inline_stack.last_mut() {
                    inlines.push(MdInline::Code(code.to_string()));
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                if let Some((_, inlines)) = inline_stack.last_mut() {
                    inlines.push(MdInline::LineBreak);
                }
            }
            Event::Rule => {
                nodes.push(MdNode::HorizontalRule);
            }
            _ => {}
        }
    }
    Ok(nodes)
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

    // --- Tests pour parse() ---

    #[test]
    fn test_parse_heading() {
        let nodes = parse("# Titre principal\n").unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(
            nodes[0],
            MdNode::Heading {
                level: 1,
                content: vec![MdInline::Text("Titre principal".to_string())],
            }
        );
    }

    #[test]
    fn test_parse_heading_levels() {
        for level in 1u8..=6 {
            let md = format!("{} Titre\n", "#".repeat(level as usize));
            let nodes = parse(&md).unwrap();
            assert_eq!(nodes.len(), 1);
            assert!(matches!(&nodes[0], MdNode::Heading { level: l, .. } if *l == level));
        }
    }

    #[test]
    fn test_parse_paragraph() {
        let nodes = parse("Du texte simple.\n").unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(
            nodes[0],
            MdNode::Paragraph {
                content: vec![MdInline::Text("Du texte simple.".to_string())],
            }
        );
    }

    #[test]
    fn test_parse_bold() {
        let nodes = parse("Du **gras** ici.\n").unwrap();
        assert_eq!(nodes.len(), 1);
        if let MdNode::Paragraph { content } = &nodes[0] {
            assert!(content.iter().any(|i| matches!(i, MdInline::Bold(_))));
        } else {
            panic!("attendu un paragraphe");
        }
    }

    #[test]
    fn test_parse_italic() {
        let nodes = parse("Du *italique* ici.\n").unwrap();
        assert_eq!(nodes.len(), 1);
        if let MdNode::Paragraph { content } = &nodes[0] {
            assert!(content.iter().any(|i| matches!(i, MdInline::Italic(_))));
        } else {
            panic!("attendu un paragraphe");
        }
    }

    #[test]
    fn test_parse_link() {
        let nodes = parse("[Google](https://google.com)\n").unwrap();
        assert_eq!(nodes.len(), 1);
        if let MdNode::Paragraph { content } = &nodes[0] {
            assert!(content.iter().any(|i| matches!(i, MdInline::Link { url, .. } if url == "https://google.com")));
        } else {
            panic!("attendu un paragraphe");
        }
    }

    #[test]
    fn test_parse_unordered_list() {
        let nodes = parse("- Item A\n- Item B\n").unwrap();
        assert_eq!(nodes.len(), 1);
        if let MdNode::UnorderedList { items } = &nodes[0] {
            assert_eq!(items.len(), 2);
        } else {
            panic!("attendu une liste non-ordonnée");
        }
    }

    #[test]
    fn test_parse_ordered_list() {
        let nodes = parse("1. Premier\n2. Deuxième\n").unwrap();
        assert_eq!(nodes.len(), 1);
        if let MdNode::OrderedList { items, .. } = &nodes[0] {
            assert_eq!(items.len(), 2);
        } else {
            panic!("attendu une liste ordonnée");
        }
    }

    #[test]
    fn test_parse_code_block() {
        let nodes = parse("```rust\nfn main() {}\n```\n").unwrap();
        assert_eq!(nodes.len(), 1);
        assert!(matches!(
            &nodes[0],
            MdNode::CodeBlock { language: Some(lang), .. } if lang == "rust"
        ));
    }

    #[test]
    fn test_parse_horizontal_rule() {
        let nodes = parse("---\n").unwrap();
        assert!(nodes.contains(&MdNode::HorizontalRule));
    }

    #[test]
    fn test_parse_round_trip_heading() {
        let original = "# Mon titre\n";
        let nodes = parse(original).unwrap();
        let rendered = render(&nodes);
        assert_eq!(rendered, original);
    }

    #[test]
    fn test_parse_round_trip_paragraph() {
        let original = "Un paragraphe simple.\n";
        let nodes = parse(original).unwrap();
        let rendered = render(&nodes);
        assert_eq!(rendered, original);
    }

    #[test]
    fn test_parse_round_trip_unordered_list() {
        let original = "- Item 1\n- Item 2\n";
        let nodes = parse(original).unwrap();
        let rendered = render(&nodes);
        assert_eq!(rendered, original);
    }

    #[test]
    fn test_parse_round_trip_code_block() {
        let original = "```rust\nfn main() {}\n```\n";
        let nodes = parse(original).unwrap();
        let rendered = render(&nodes);
        assert_eq!(rendered, original);
    }
}
