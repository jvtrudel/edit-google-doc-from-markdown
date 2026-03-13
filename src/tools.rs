use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Définit les outils disponibles pour l'agent Claude
pub fn tool_definitions() -> Vec<Value> {
    vec![
        json!({
            "name": "read_document",
            "description": "Lit le contenu complet d'un document Google Docs par son ID.",
            "input_schema": {
                "type": "object",
                "properties": {
                    "document_id": {
                        "type": "string",
                        "description": "L'ID du document Google Docs (trouvé dans l'URL)."
                    }
                },
                "required": ["document_id"]
            }
        }),
        json!({
            "name": "insert_text",
            "description": "Insère du texte à une position précise dans un document Google Docs.",
            "input_schema": {
                "type": "object",
                "properties": {
                    "document_id": {
                        "type": "string",
                        "description": "L'ID du document Google Docs."
                    },
                    "text": {
                        "type": "string",
                        "description": "Le texte à insérer."
                    },
                    "index": {
                        "type": "integer",
                        "description": "L'index (1-based) où insérer le texte."
                    }
                },
                "required": ["document_id", "text", "index"]
            }
        }),
        json!({
            "name": "delete_content_range",
            "description": "Supprime un segment de texte dans un document Google Docs.",
            "input_schema": {
                "type": "object",
                "properties": {
                    "document_id": {
                        "type": "string",
                        "description": "L'ID du document Google Docs."
                    },
                    "start_index": {
                        "type": "integer",
                        "description": "Index de début de la plage à supprimer (1-based)."
                    },
                    "end_index": {
                        "type": "integer",
                        "description": "Index de fin de la plage à supprimer (exclusif)."
                    }
                },
                "required": ["document_id", "start_index", "end_index"]
            }
        }),
        json!({
            "name": "replace_all_text",
            "description": "Remplace toutes les occurrences d'un texte dans un document Google Docs.",
            "input_schema": {
                "type": "object",
                "properties": {
                    "document_id": {
                        "type": "string",
                        "description": "L'ID du document Google Docs."
                    },
                    "search_text": {
                        "type": "string",
                        "description": "Le texte à rechercher."
                    },
                    "replacement_text": {
                        "type": "string",
                        "description": "Le texte de remplacement."
                    },
                    "match_case": {
                        "type": "boolean",
                        "description": "Si vrai, la recherche est sensible à la casse. Défaut: false."
                    }
                },
                "required": ["document_id", "search_text", "replacement_text"]
            }
        }),
        json!({
            "name": "create_document",
            "description": "Crée un nouveau document Google Docs avec un titre donné.",
            "input_schema": {
                "type": "object",
                "properties": {
                    "title": {
                        "type": "string",
                        "description": "Le titre du nouveau document."
                    }
                },
                "required": ["title"]
            }
        }),
    ]
}

/// Résultat d'un appel d'outil
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_use_id: String,
    pub content: String,
    pub is_error: bool,
}

impl ToolResult {
    pub fn success(tool_use_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            tool_use_id: tool_use_id.into(),
            content: content.into(),
            is_error: false,
        }
    }

    pub fn error(tool_use_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            tool_use_id: tool_use_id.into(),
            content: message.into(),
            is_error: true,
        }
    }
}
