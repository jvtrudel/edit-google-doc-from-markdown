#![allow(dead_code)]
//! Conversion bidirectionnelle Markdown ↔ structure Google Docs
//!
//! Ce module convertit entre la représentation intermédiaire Markdown (MdNode)
//! et la structure Google Docs (google_docs1::api::Document).
//!
//! La conversion porte sur le **contenu** uniquement.
//! Le style est géré séparément par le module `style`.

use anyhow::Result;
use google_docs1::api::{
    CreateParagraphBulletsRequest, Document, Link, Location, Paragraph, ParagraphStyle,
    StructuralElement, TextStyle, UpdateParagraphStyleRequest, UpdateTextStyleRequest,
};
use google_docs1::FieldMask;
use std::collections::HashMap;
use tracing::debug;

use crate::markdown::{MdInline, MdNode};

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

/// Span de style inline : plage de caractères avec ses attributs visuels
struct InlineSpan {
    start: i32,
    end: i32,
    bold: bool,
    italic: bool,
    link: Option<String>,
}

/// Extrait le texte brut et les spans de style depuis des inlines Markdown.
/// `doc_offset` : position absolue dans le document du début de ce texte.
fn collect_inline_text(inlines: &[MdInline], doc_offset: i32) -> (String, Vec<InlineSpan>) {
    let mut text = String::new();
    let mut spans = Vec::new();
    collect_spans_rec(inlines, doc_offset, false, false, None, &mut text, &mut spans);
    (text, spans)
}

fn collect_spans_rec(
    inlines: &[MdInline],
    doc_offset: i32,
    bold: bool,
    italic: bool,
    link: Option<&str>,
    text: &mut String,
    spans: &mut Vec<InlineSpan>,
) {
    for inline in inlines {
        match inline {
            MdInline::Text(s) | MdInline::Code(s) => {
                let start = doc_offset + text.chars().count() as i32;
                text.push_str(s);
                let end = doc_offset + text.chars().count() as i32;
                if (bold || italic || link.is_some()) && start < end {
                    spans.push(InlineSpan { start, end, bold, italic, link: link.map(String::from) });
                }
            }
            MdInline::Bold(inner) => {
                collect_spans_rec(inner, doc_offset, true, italic, link, text, spans);
            }
            MdInline::Italic(inner) => {
                collect_spans_rec(inner, doc_offset, bold, true, link, text, spans);
            }
            MdInline::Link { text: link_text, url } => {
                let start = doc_offset + text.chars().count() as i32;
                text.push_str(link_text);
                let end = doc_offset + text.chars().count() as i32;
                if start < end {
                    spans.push(InlineSpan { start, end, bold, italic, link: Some(url.clone()) });
                }
            }
            MdInline::LineBreak => text.push('\n'),
        }
    }
}

/// Convertit des spans de style en requêtes UpdateTextStyle
fn make_style_requests(spans: &[InlineSpan]) -> Vec<google_docs1::api::Request> {
    use google_docs1::api::{Range, Request};
    spans.iter().filter_map(|span| {
        let mut field_names: Vec<&str> = Vec::new();
        if span.bold { field_names.push("bold"); }
        if span.italic { field_names.push("italic"); }
        if span.link.is_some() { field_names.push("link"); }
        if field_names.is_empty() { return None; }

        Some(Request {
            update_text_style: Some(UpdateTextStyleRequest {
                range: Some(Range {
                    start_index: Some(span.start),
                    end_index: Some(span.end),
                    segment_id: None,
                }),
                text_style: Some(TextStyle {
                    bold: span.bold.then_some(true),
                    italic: span.italic.then_some(true),
                    link: span.link.as_ref().map(|url| Link {
                        url: Some(url.clone()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                fields: Some(FieldMask::new(&field_names)),
            }),
            ..Default::default()
        })
    }).collect()
}

/// Insère un paragraphe de texte et retourne les requêtes (insert + styles inline)
fn insert_paragraph(
    content: &[MdInline],
    index: i32,
) -> (Vec<google_docs1::api::Request>, i32) {
    use google_docs1::api::{InsertTextRequest, Request};
    let (plain, spans) = collect_inline_text(content, index);
    let full_text = format!("{}\n", plain);
    let len = full_text.chars().count() as i32;
    let mut reqs = vec![Request {
        insert_text: Some(InsertTextRequest {
            text: Some(full_text),
            location: Some(Location { index: Some(index), segment_id: None }),
            end_of_segment_location: None,
        }),
        ..Default::default()
    }];
    reqs.extend(make_style_requests(&spans));
    (reqs, len)
}

/// Convertit une représentation Markdown en requêtes Google Docs batchUpdate
///
/// `doc_end_index` est l'index de fin du corps du document actuel (obtenu depuis `Document.body`).
/// Il est nécessaire pour calculer la plage de suppression du contenu existant.
///
/// Retourne les requêtes nécessaires pour recréer le contenu du document.
pub fn markdown_to_gdoc_requests(
    nodes: &[MdNode],
    doc_end_index: i32,
) -> Result<ConversionResult<Vec<google_docs1::api::Request>>> {
    use google_docs1::api::{DeleteContentRangeRequest, InsertTextRequest, Range, Request};
    let mut requests = Vec::new();
    let losses = Vec::new();

    // 1. Effacer tout le contenu du document (si non-vide)
    // L'API interdit de supprimer le dernier \n du Body, donc end_index = doc_end_index - 1
    if doc_end_index > 2 {
        requests.push(Request {
            delete_content_range: Some(DeleteContentRangeRequest {
                range: Some(Range {
                    start_index: Some(1),
                    end_index: Some(doc_end_index - 1),
                    segment_id: None,
                }),
            }),
            ..Default::default()
        });
    }

    // 2. Insérer le nouveau contenu
    let mut index: i32 = 1;

    for node in nodes {
        match node {
            MdNode::Heading { level, content } => {
                let (reqs, len) = insert_paragraph(content, index);
                requests.extend(reqs);
                let style_type = match level {
                    1 => "HEADING_1",
                    2 => "HEADING_2",
                    3 => "HEADING_3",
                    4 => "HEADING_4",
                    5 => "HEADING_5",
                    _ => "HEADING_6",
                };
                requests.push(Request {
                    update_paragraph_style: Some(UpdateParagraphStyleRequest {
                        range: Some(Range {
                            start_index: Some(index),
                            end_index: Some(index + len),
                            segment_id: None,
                        }),
                        paragraph_style: Some(ParagraphStyle {
                            named_style_type: Some(style_type.to_string()),
                            ..Default::default()
                        }),
                        fields: Some(FieldMask::new(&["namedStyleType"])),
                    }),
                    ..Default::default()
                });
                index += len;
            }
            MdNode::Paragraph { content } => {
                let (reqs, len) = insert_paragraph(content, index);
                requests.extend(reqs);
                index += len;
            }
            MdNode::UnorderedList { items } => {
                let list_start = index;
                for item in items {
                    let inlines: Vec<_> = item.iter().flat_map(|n| match n {
                        MdNode::Paragraph { content } => content.clone(),
                        _ => vec![],
                    }).collect();
                    let (reqs, len) = insert_paragraph(&inlines, index);
                    requests.extend(reqs);
                    index += len;
                }
                requests.push(Request {
                    create_paragraph_bullets: Some(CreateParagraphBulletsRequest {
                        range: Some(Range {
                            start_index: Some(list_start),
                            end_index: Some(index),
                            segment_id: None,
                        }),
                        bullet_preset: Some("BULLET_DISC_CIRCLE_SQUARE".to_string()),
                    }),
                    ..Default::default()
                });
            }
            MdNode::OrderedList { items, .. } => {
                let list_start = index;
                for item in items {
                    let inlines: Vec<_> = item.iter().flat_map(|n| match n {
                        MdNode::Paragraph { content } => content.clone(),
                        _ => vec![],
                    }).collect();
                    let (reqs, len) = insert_paragraph(&inlines, index);
                    requests.extend(reqs);
                    index += len;
                }
                requests.push(Request {
                    create_paragraph_bullets: Some(CreateParagraphBulletsRequest {
                        range: Some(Range {
                            start_index: Some(list_start),
                            end_index: Some(index),
                            segment_id: None,
                        }),
                        bullet_preset: Some("NUMBERED_DECIMAL_ALPHA_ROMAN".to_string()),
                    }),
                    ..Default::default()
                });
            }
            MdNode::CodeBlock { language, code } => {
                let text = match language {
                    Some(lang) => format!("```{}\n{}\n```\n", lang, code),
                    None => format!("```\n{}\n```\n", code),
                };
                let len = text.chars().count() as i32;
                requests.push(Request {
                    insert_text: Some(InsertTextRequest {
                        text: Some(text),
                        location: Some(Location { index: Some(index), segment_id: None }),
                        end_of_segment_location: None,
                    }),
                    ..Default::default()
                });
                index += len;
            }
            MdNode::HorizontalRule => {
                let text = "---\n".to_string();
                let len = text.chars().count() as i32;
                requests.push(Request {
                    insert_text: Some(InsertTextRequest {
                        text: Some(text),
                        location: Some(Location { index: Some(index), segment_id: None }),
                        end_of_segment_location: None,
                    }),
                    ..Default::default()
                });
                index += len;
            }
        }
    }

    Ok(ConversionResult { result: requests, losses })
}

/// Convertit un document Google Docs en représentation Markdown intermédiaire
///
/// Extrait le contenu sémantique du document. Le style est ignoré ici
/// (il est extrait séparément par le module `style`).
pub fn gdoc_to_markdown(document: &Document) -> Result<ConversionResult<Vec<MdNode>>> {
    let mut nodes: Vec<MdNode> = Vec::new();
    let mut losses: Vec<InformationLoss> = Vec::new();

    // Récupérer les éléments structurels du body
    let elements = match &document.body {
        Some(body) => body.content.as_deref().unwrap_or(&[]),
        None => &[],
    };

    // Récupérer la map des listes pour déterminer le type (ordonnée vs non-ordonnée)
    let lists = document.lists.as_ref();

    // Accumulateur pour regrouper les paragraphes-bullet consécutifs en listes
    let mut bullet_acc: Vec<BulletItem> = Vec::new();

    for element in elements {
        if let Some(paragraph) = &element.paragraph
            && paragraph.bullet.is_some()
        {
            // Ce paragraphe fait partie d'une liste — accumuler
            let item = extract_bullet_item(paragraph, lists, &mut losses);
            bullet_acc.push(item);
            continue;
        }

        // Si on arrive ici, le paragraphe n'est pas un bullet.
        // Vider l'accumulateur de liste d'abord.
        if !bullet_acc.is_empty() {
            let list_nodes = flush_bullet_accumulator(&mut bullet_acc);
            nodes.extend(list_nodes);
        }

        // Traiter l'élément courant
        if let Some(node) = convert_structural_element(element, &mut losses) {
            nodes.push(node);
        }
    }

    // Vider l'accumulateur de liste en fin de document
    if !bullet_acc.is_empty() {
        let list_nodes = flush_bullet_accumulator(&mut bullet_acc);
        nodes.extend(list_nodes);
    }

    Ok(ConversionResult {
        result: nodes,
        losses,
    })
}

/// Élément de liste accumulé avant regroupement
struct BulletItem {
    /// Contenu inline du paragraphe
    inlines: Vec<MdInline>,
    /// Niveau d'imbrication (0 = racine)
    nesting_level: i32,
    /// true si la liste est ordonnée
    ordered: bool,
    /// Numéro de départ pour les listes ordonnées
    start_number: u64,
    /// Identifiant de la liste
    list_id: String,
}

/// Extrait les informations d'un paragraphe-bullet
fn extract_bullet_item(
    paragraph: &Paragraph,
    lists: Option<&HashMap<String, google_docs1::api::List>>,
    losses: &mut Vec<InformationLoss>,
) -> BulletItem {
    let bullet = paragraph.bullet.as_ref().unwrap();
    let list_id = bullet.list_id.clone().unwrap_or_default();
    let nesting_level = bullet.nesting_level.unwrap_or(0);

    // Déterminer si la liste est ordonnée via les propriétés de liste
    let (ordered, start_number) = if let Some(lists_map) = lists {
        if let Some(list) = lists_map.get(&list_id) {
            if let Some(props) = &list.list_properties {
                if let Some(levels) = &props.nesting_levels {
                    if let Some(level) = levels.get(nesting_level as usize) {
                        // Si glyph_type est défini et que glyph_symbol ne l'est pas,
                        // c'est une liste ordonnée
                        let is_ordered = level.glyph_type.is_some() && level.glyph_symbol.is_none();
                        let start = level.start_number.unwrap_or(1).max(1) as u64;
                        (is_ordered, start)
                    } else {
                        (false, 1)
                    }
                } else {
                    (false, 1)
                }
            } else {
                (false, 1)
            }
        } else {
            (false, 1)
        }
    } else {
        (false, 1)
    };

    let inlines = extract_paragraph_inlines(paragraph, losses);

    BulletItem {
        inlines,
        nesting_level,
        ordered,
        start_number,
        list_id,
    }
}

/// Regroupe les éléments bullet accumulés en nœuds de liste MdNode
fn flush_bullet_accumulator(acc: &mut Vec<BulletItem>) -> Vec<MdNode> {
    let items: Vec<BulletItem> = std::mem::take(acc);
    build_list_nodes(&items, 0, 0, items.len())
}

/// Construit récursivement les nœuds de liste à partir des éléments bullet
fn build_list_nodes(
    items: &[BulletItem],
    target_level: i32,
    start: usize,
    end: usize,
) -> Vec<MdNode> {
    if start >= end {
        return Vec::new();
    }

    let mut result: Vec<MdNode> = Vec::new();
    let mut i = start;

    while i < end {
        let item = &items[i];

        if item.nesting_level < target_level {
            // On est sorti du niveau courant
            break;
        }

        if item.nesting_level == target_level {
            // Cet item est au bon niveau — trouver ses sous-items
            let mut sub_end = i + 1;
            while sub_end < end && items[sub_end].nesting_level > target_level {
                sub_end += 1;
            }

            // Construire le contenu de cet item
            let mut item_content = vec![MdNode::Paragraph {
                content: item.inlines.clone(),
            }];

            // Ajouter les sous-listes
            if sub_end > i + 1 {
                let sub_nodes = build_list_nodes(items, target_level + 1, i + 1, sub_end);
                item_content.extend(sub_nodes);
            }

            // Accumuler les items pour ce groupe de liste
            // On crée une seule liste par groupe consécutif de même type
            let is_new_list = result.is_empty() || !matches_list_type(result.last(), item.ordered);

            if is_new_list {
                if item.ordered {
                    result.push(MdNode::OrderedList {
                        start: item.start_number,
                        items: vec![item_content],
                    });
                } else {
                    result.push(MdNode::UnorderedList {
                        items: vec![item_content],
                    });
                }
            } else {
                // Ajouter à la liste existante
                match result.last_mut() {
                    Some(MdNode::OrderedList {
                        items: list_items, ..
                    })
                    | Some(MdNode::UnorderedList {
                        items: list_items, ..
                    }) => {
                        list_items.push(item_content);
                    }
                    _ => unreachable!(),
                }
            }

            i = sub_end;
        } else {
            // item.nesting_level > target_level — sous-items orphelins, les traiter comme racine
            i += 1;
        }
    }

    result
}

/// Vérifie si le dernier nœud est une liste du même type
fn matches_list_type(node: Option<&MdNode>, ordered: bool) -> bool {
    matches!(
        (node, ordered),
        (Some(MdNode::OrderedList { .. }), true) | (Some(MdNode::UnorderedList { .. }), false)
    )
}

/// Convertit un élément structurel en MdNode
fn convert_structural_element(
    element: &StructuralElement,
    losses: &mut Vec<InformationLoss>,
) -> Option<MdNode> {
    if let Some(paragraph) = &element.paragraph {
        return convert_paragraph(paragraph, losses);
    }

    if element.table.is_some() {
        losses.push(InformationLoss {
            kind: LossKind::Content,
            description: "Table ignorée (non supportée en Markdown simple)".to_string(),
            position: element.start_index.map(|i| format!("index {}", i)),
        });
    }

    if element.table_of_contents.is_some() {
        losses.push(InformationLoss {
            kind: LossKind::Content,
            description: "Table des matières ignorée".to_string(),
            position: element.start_index.map(|i| format!("index {}", i)),
        });
    }

    // section_break → ignoré silencieusement
    None
}

/// Convertit un paragraphe Google Docs en MdNode
fn convert_paragraph(paragraph: &Paragraph, losses: &mut Vec<InformationLoss>) -> Option<MdNode> {
    let inlines = extract_paragraph_inlines(paragraph, losses);

    // Paragraphe vide → ignorer
    if inlines.is_empty() {
        return None;
    }

    // Déterminer le type via named_style_type
    let style_type = paragraph
        .paragraph_style
        .as_ref()
        .and_then(|s| s.named_style_type.as_deref())
        .unwrap_or("NORMAL_TEXT");

    match style_type {
        "HEADING_1" => Some(MdNode::Heading {
            level: 1,
            content: inlines,
        }),
        "HEADING_2" => Some(MdNode::Heading {
            level: 2,
            content: inlines,
        }),
        "HEADING_3" => Some(MdNode::Heading {
            level: 3,
            content: inlines,
        }),
        "HEADING_4" => Some(MdNode::Heading {
            level: 4,
            content: inlines,
        }),
        "HEADING_5" => Some(MdNode::Heading {
            level: 5,
            content: inlines,
        }),
        "HEADING_6" => Some(MdNode::Heading {
            level: 6,
            content: inlines,
        }),
        _ => Some(MdNode::Paragraph { content: inlines }),
    }
}

/// Extrait les éléments inline d'un paragraphe
fn extract_paragraph_inlines(
    paragraph: &Paragraph,
    losses: &mut Vec<InformationLoss>,
) -> Vec<MdInline> {
    let elements = match &paragraph.elements {
        Some(elems) => elems,
        None => return Vec::new(),
    };

    let mut inlines: Vec<MdInline> = Vec::new();

    for elem in elements {
        if let Some(text_run) = &elem.text_run {
            let content = text_run.content.as_deref().unwrap_or("");

            // Google Docs ajoute un \n en fin de chaque paragraphe — l'ignorer
            let content = content.strip_suffix('\n').unwrap_or(content);

            if content.is_empty() {
                continue;
            }

            let inline = convert_text_run(content, &text_run.text_style);
            inlines.push(inline);
        } else if elem.inline_object_element.is_some() {
            losses.push(InformationLoss {
                kind: LossKind::Content,
                description: "Objet inline (image, etc.) ignoré".to_string(),
                position: elem.start_index.map(|i| format!("index {}", i)),
            });
        } else if elem.horizontal_rule.is_some() {
            // Les règles horizontales dans Google Docs sont des ParagraphElement,
            // mais on ne peut pas les intégrer comme inline.
            // On les ignore ici — elles devraient être traitées comme blocs.
            debug!(
                "Règle horizontale trouvée comme ParagraphElement — traitée comme MdInline::Text"
            );
        }
    }

    inlines
}

/// Convertit un TextRun en MdInline, en tenant compte du style (gras, italique, lien)
fn convert_text_run(content: &str, text_style: &Option<google_docs1::api::TextStyle>) -> MdInline {
    let base = MdInline::Text(content.to_string());

    let style = match text_style {
        Some(s) => s,
        None => return base,
    };

    // Vérifier s'il y a un lien
    if let Some(link) = &style.link
        && let Some(url) = &link.url
    {
        return MdInline::Link {
            text: content.to_string(),
            url: url.clone(),
        };
    }

    let is_bold = style.bold.unwrap_or(false);
    let is_italic = style.italic.unwrap_or(false);

    match (is_bold, is_italic) {
        (true, true) => MdInline::Bold(vec![MdInline::Italic(vec![MdInline::Text(
            content.to_string(),
        )])]),
        (true, false) => MdInline::Bold(vec![MdInline::Text(content.to_string())]),
        (false, true) => MdInline::Italic(vec![MdInline::Text(content.to_string())]),
        (false, false) => base,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use google_docs1::api::{
        Body, Document, Paragraph, ParagraphElement, ParagraphStyle, StructuralElement, TextRun,
        TextStyle,
    };

    /// Crée un Document minimal avec un seul paragraphe de texte
    fn make_simple_doc(text: &str) -> Document {
        Document {
            body: Some(Body {
                content: Some(vec![StructuralElement {
                    paragraph: Some(Paragraph {
                        elements: Some(vec![ParagraphElement {
                            text_run: Some(TextRun {
                                content: Some(format!("{}\n", text)),
                                text_style: None,
                                ..Default::default()
                            }),
                            ..Default::default()
                        }]),
                        paragraph_style: Some(ParagraphStyle {
                            named_style_type: Some("NORMAL_TEXT".to_string()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }]),
            }),
            ..Default::default()
        }
    }

    #[test]
    fn test_simple_paragraph() {
        let doc = make_simple_doc("Bonjour le monde");
        let result = gdoc_to_markdown(&doc).unwrap();

        assert_eq!(result.result.len(), 1);
        assert_eq!(
            result.result[0],
            MdNode::Paragraph {
                content: vec![MdInline::Text("Bonjour le monde".to_string())]
            }
        );
        assert!(result.losses.is_empty());
    }

    #[test]
    fn test_heading() {
        let doc = Document {
            body: Some(Body {
                content: Some(vec![StructuralElement {
                    paragraph: Some(Paragraph {
                        elements: Some(vec![ParagraphElement {
                            text_run: Some(TextRun {
                                content: Some("Mon titre\n".to_string()),
                                text_style: None,
                                ..Default::default()
                            }),
                            ..Default::default()
                        }]),
                        paragraph_style: Some(ParagraphStyle {
                            named_style_type: Some("HEADING_1".to_string()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }]),
            }),
            ..Default::default()
        };

        let result = gdoc_to_markdown(&doc).unwrap();
        assert_eq!(result.result.len(), 1);
        assert_eq!(
            result.result[0],
            MdNode::Heading {
                level: 1,
                content: vec![MdInline::Text("Mon titre".to_string())]
            }
        );
    }

    #[test]
    fn test_bold_text() {
        let doc = Document {
            body: Some(Body {
                content: Some(vec![StructuralElement {
                    paragraph: Some(Paragraph {
                        elements: Some(vec![ParagraphElement {
                            text_run: Some(TextRun {
                                content: Some("gras\n".to_string()),
                                text_style: Some(TextStyle {
                                    bold: Some(true),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }]),
                        paragraph_style: Some(ParagraphStyle {
                            named_style_type: Some("NORMAL_TEXT".to_string()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }]),
            }),
            ..Default::default()
        };

        let result = gdoc_to_markdown(&doc).unwrap();
        assert_eq!(
            result.result[0],
            MdNode::Paragraph {
                content: vec![MdInline::Bold(vec![MdInline::Text("gras".to_string())])]
            }
        );
    }

    #[test]
    fn test_empty_document() {
        let doc = Document {
            body: None,
            ..Default::default()
        };
        let result = gdoc_to_markdown(&doc).unwrap();
        assert!(result.result.is_empty());
        assert!(result.losses.is_empty());
    }

    #[test]
    fn test_conversion_then_render_stability() {
        let doc = make_simple_doc("Un paragraphe simple");
        let result = gdoc_to_markdown(&doc).unwrap();
        let md1 = crate::markdown::render(&result.result);
        let md2 = crate::markdown::render(&result.result);
        assert_eq!(md1, md2, "Le rendu doit être déterministe");
    }

    // --- Tests pour markdown_to_gdoc_requests() ---

    #[test]
    fn test_md_to_gdoc_first_request_is_delete() {
        let nodes = vec![crate::markdown::MdNode::Paragraph {
            content: vec![crate::markdown::MdInline::Text("Bonjour".to_string())],
        }];
        let result = markdown_to_gdoc_requests(&nodes, 100).unwrap();
        let requests = result.result;
        assert!(!requests.is_empty());
        assert!(requests[0].delete_content_range.is_some(), "La première requête doit effacer le contenu");
    }

    #[test]
    fn test_md_to_gdoc_paragraph_generates_insert() {
        let nodes = vec![crate::markdown::MdNode::Paragraph {
            content: vec![crate::markdown::MdInline::Text("Texte".to_string())],
        }];
        let result = markdown_to_gdoc_requests(&nodes, 100).unwrap();
        let requests = result.result;
        // delete + insert
        assert_eq!(requests.len(), 2);
        let insert = &requests[1];
        assert!(insert.insert_text.is_some());
        let insert_text = insert.insert_text.as_ref().unwrap();
        assert_eq!(insert_text.text.as_deref(), Some("Texte\n"));
    }

    #[test]
    fn test_md_to_gdoc_indexes_are_cumulative() {
        let nodes = vec![
            crate::markdown::MdNode::Paragraph {
                content: vec![crate::markdown::MdInline::Text("AB".to_string())],
            },
            crate::markdown::MdNode::Paragraph {
                content: vec![crate::markdown::MdInline::Text("CD".to_string())],
            },
        ];
        let result = markdown_to_gdoc_requests(&nodes, 100).unwrap();
        let requests = result.result;
        // delete + 2 inserts
        assert_eq!(requests.len(), 3);
        let idx1 = requests[1].insert_text.as_ref().unwrap().location.as_ref().unwrap().index.unwrap();
        let idx2 = requests[2].insert_text.as_ref().unwrap().location.as_ref().unwrap().index.unwrap();
        // "AB\n" = 3 chars, so second insert should be at index 1 + 3 = 4
        assert_eq!(idx1, 1);
        assert_eq!(idx2, 4);
    }

    #[test]
    fn test_md_to_gdoc_no_losses_for_basic_content() {
        let nodes = vec![
            crate::markdown::MdNode::Heading {
                level: 1,
                content: vec![crate::markdown::MdInline::Text("Titre".to_string())],
            },
            crate::markdown::MdNode::Paragraph {
                content: vec![crate::markdown::MdInline::Text("Contenu".to_string())],
            },
        ];
        let result = markdown_to_gdoc_requests(&nodes, 100).unwrap();
        assert!(result.losses.is_empty());
    }
}
