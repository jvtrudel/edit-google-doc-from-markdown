use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{debug, info};

use crate::google_docs::GoogleDocsClient;
use crate::tools::{tool_definitions, ToolResult};

/// Modèle Claude Opus 4.6 (le plus récent modèle Opus)
pub const CLAUDE_MODEL: &str = "claude-opus-4-6";

/// Nombre maximum d'itérations de la boucle agentique
const MAX_ITERATIONS: usize = 20;

/// Configuration de l'agent
pub struct AgentConfig {
    pub api_key: String,
    pub system_prompt: String,
    pub max_tokens: u32,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            system_prompt: "Tu es un assistant expert en édition de documents Google Docs. \
                Tu utilises les outils à ta disposition pour lire, créer et modifier des \
                documents de manière précise et efficace. Réponds toujours en français."
                .to_string(),
            max_tokens: 8192,
        }
    }
}

/// Message dans la conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: Value,
}

impl Message {
    pub fn user(text: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: json!(text.into()),
        }
    }

    pub fn assistant(content: Value) -> Self {
        Self {
            role: "assistant".to_string(),
            content,
        }
    }

    pub fn tool_results(results: Vec<ToolResult>) -> Self {
        let content: Vec<Value> = results
            .into_iter()
            .map(|r| {
                json!({
                    "type": "tool_result",
                    "tool_use_id": r.tool_use_id,
                    "content": r.content,
                    "is_error": r.is_error
                })
            })
            .collect();

        Self {
            role: "user".to_string(),
            content: json!(content),
        }
    }
}

/// Réponse de l'API Anthropic
#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    pub content: Vec<ContentBlock>,
    pub stop_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
}

/// Agent Claude avec boucle agentique complète
pub struct ClaudeAgent {
    config: AgentConfig,
    client: Client,
    docs_client: GoogleDocsClient,
    conversation: Vec<Message>,
}

impl ClaudeAgent {
    pub fn new(config: AgentConfig, docs_client: GoogleDocsClient) -> Self {
        Self {
            config,
            client: Client::new(),
            docs_client,
            conversation: Vec::new(),
        }
    }

    /// Lance l'agent avec un message utilisateur initial
    pub async fn run(&mut self, user_message: impl Into<String>) -> Result<String> {
        let user_msg = user_message.into();
        info!("Démarrage de l'agent avec le message: {}", user_msg);

        self.conversation.push(Message::user(user_msg));

        for iteration in 0..MAX_ITERATIONS {
            debug!("Itération {} de la boucle agentique", iteration + 1);

            let response = self.call_claude().await?;

            // Vérifier le stop_reason
            let stop_reason = response.stop_reason.as_deref().unwrap_or("");

            // Séparer les blocs texte et outil
            let mut text_parts: Vec<String> = Vec::new();
            let mut tool_calls: Vec<(String, String, Value)> = Vec::new(); // (id, name, input)

            for block in &response.content {
                match block {
                    ContentBlock::Text { text } => {
                        text_parts.push(text.clone());
                    }
                    ContentBlock::ToolUse { id, name, input } => {
                        tool_calls.push((id.clone(), name.clone(), input.clone()));
                    }
                }
            }

            // Ajouter la réponse de l'assistant à la conversation
            let assistant_content = build_assistant_content(&response.content);
            self.conversation.push(Message::assistant(assistant_content));

            if !text_parts.is_empty() {
                info!("Claude: {}", text_parts.join("\n"));
            }

            // Si pas d'appels d'outils, la réponse est finale
            if stop_reason == "end_turn" || tool_calls.is_empty() {
                let final_text = text_parts.join("\n");
                return Ok(final_text);
            }

            // Exécuter les outils
            info!("Exécution de {} outil(s)...", tool_calls.len());
            let mut tool_results: Vec<ToolResult> = Vec::new();

            for (tool_id, tool_name, tool_input) in &tool_calls {
                info!("Appel de l'outil: {} (id: {})", tool_name, tool_id);
                let result = self
                    .execute_tool(tool_id, tool_name, tool_input)
                    .await;
                tool_results.push(result);
            }

            // Ajouter les résultats des outils à la conversation
            self.conversation.push(Message::tool_results(tool_results));
        }

        Err(anyhow::anyhow!(
            "Nombre maximum d'itérations ({}) atteint",
            MAX_ITERATIONS
        ))
    }

    /// Appelle l'API Claude avec la conversation courante
    async fn call_claude(&self) -> Result<AnthropicResponse> {
        let messages: Vec<Value> = self
            .conversation
            .iter()
            .map(|m| {
                json!({
                    "role": m.role,
                    "content": m.content
                })
            })
            .collect();

        let body = json!({
            "model": CLAUDE_MODEL,
            "max_tokens": self.config.max_tokens,
            "system": self.config.system_prompt,
            "tools": tool_definitions(),
            "messages": messages
        });

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Erreur lors de l'appel à l'API Anthropic")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Erreur API Anthropic ({}): {}",
                status,
                error_text
            ));
        }

        let anthropic_response: AnthropicResponse = response
            .json()
            .await
            .context("Impossible de désérialiser la réponse Anthropic")?;

        Ok(anthropic_response)
    }

    /// Exécute un outil et retourne son résultat
    async fn execute_tool(
        &self,
        tool_id: &str,
        tool_name: &str,
        input: &Value,
    ) -> ToolResult {
        let result = match tool_name {
            "read_document" => {
                let doc_id = input["document_id"].as_str().unwrap_or_default();
                self.docs_client
                    .read_document(doc_id)
                    .await
                    .map_err(|e| e.to_string())
            }
            "insert_text" => {
                let doc_id = input["document_id"].as_str().unwrap_or_default();
                let text = input["text"].as_str().unwrap_or_default();
                let index = input["index"].as_i64().unwrap_or(1) as i32;
                self.docs_client
                    .insert_text(doc_id, text, index)
                    .await
                    .map_err(|e| e.to_string())
            }
            "delete_content_range" => {
                let doc_id = input["document_id"].as_str().unwrap_or_default();
                let start = input["start_index"].as_i64().unwrap_or(1) as i32;
                let end = input["end_index"].as_i64().unwrap_or(1) as i32;
                self.docs_client
                    .delete_content_range(doc_id, start, end)
                    .await
                    .map_err(|e| e.to_string())
            }
            "replace_all_text" => {
                let doc_id = input["document_id"].as_str().unwrap_or_default();
                let search = input["search_text"].as_str().unwrap_or_default();
                let replacement = input["replacement_text"].as_str().unwrap_or_default();
                let match_case = input["match_case"].as_bool().unwrap_or(false);
                self.docs_client
                    .replace_all_text(doc_id, search, replacement, match_case)
                    .await
                    .map_err(|e| e.to_string())
            }
            "create_document" => {
                let title = input["title"].as_str().unwrap_or("Nouveau document");
                self.docs_client
                    .create_document(title)
                    .await
                    .map_err(|e| e.to_string())
            }
            unknown => Err(format!("Outil inconnu: {}", unknown)),
        };

        match result {
            Ok(content) => ToolResult::success(tool_id, content),
            Err(err) => ToolResult::error(tool_id, err),
        }
    }
}

/// Construit le contenu assistant à partir des blocs de contenu
fn build_assistant_content(blocks: &[ContentBlock]) -> Value {
    let content: Vec<Value> = blocks
        .iter()
        .map(|block| match block {
            ContentBlock::Text { text } => json!({
                "type": "text",
                "text": text
            }),
            ContentBlock::ToolUse { id, name, input } => json!({
                "type": "tool_use",
                "id": id,
                "name": name,
                "input": input
            }),
        })
        .collect();

    json!(content)
}
