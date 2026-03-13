use anyhow::{Context, Result};
use google_docs1::api::BatchUpdateDocumentRequest;
use google_docs1::api::Document;
use google_docs1::api::Request;
use google_docs1::api::InsertTextRequest;
use google_docs1::api::DeleteContentRangeRequest;
use google_docs1::api::Range;
use google_docs1::api::ReplaceAllTextRequest;
use google_docs1::api::SubstringMatchCriteria;
use google_docs1::Docs;
use hyper::Client;
use hyper_rustls::HttpsConnectorBuilder;
use yup_oauth2::{ServiceAccountAuthenticator, read_service_account_key};

/// Client Google Docs utilisant un compte de service
pub struct GoogleDocsClient {
    hub: Docs<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
}

impl GoogleDocsClient {
    /// Crée un nouveau client en utilisant un fichier de clé de compte de service
    pub async fn new(service_account_key_path: &str) -> Result<Self> {
        let sa_key = read_service_account_key(service_account_key_path)
            .await
            .context("Impossible de lire le fichier de clé du compte de service")?;

        let auth = ServiceAccountAuthenticator::builder(sa_key)
            .build()
            .await
            .context("Impossible d'initialiser l'authentification Google")?;

        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build();

        let client = Client::builder().build::<_, hyper::Body>(connector);
        let hub = Docs::new(client, auth);

        Ok(Self { hub })
    }

    /// Lit le contenu d'un document et retourne son texte brut
    pub async fn read_document(&self, document_id: &str) -> Result<String> {
        let (_, doc) = self
            .hub
            .documents()
            .get(document_id)
            .doit()
            .await
            .context("Impossible de lire le document")?;

        let text = extract_text_from_document(&doc);
        Ok(text)
    }

    /// Lit le document complet (structure JSON)
    pub async fn get_document(&self, document_id: &str) -> Result<Document> {
        let (_, doc) = self
            .hub
            .documents()
            .get(document_id)
            .doit()
            .await
            .context("Impossible de lire le document")?;
        Ok(doc)
    }

    /// Insère du texte à un index donné
    pub async fn insert_text(&self, document_id: &str, text: &str, index: i32) -> Result<String> {
        let request = BatchUpdateDocumentRequest {
            requests: Some(vec![Request {
                insert_text: Some(InsertTextRequest {
                    text: Some(text.to_string()),
                    location: Some(google_docs1::api::Location {
                        index: Some(index),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }]),
            ..Default::default()
        };

        self.hub
            .documents()
            .batch_update(request, document_id)
            .doit()
            .await
            .context("Impossible d'insérer le texte")?;

        Ok(format!("Texte inséré à l'index {} avec succès.", index))
    }

    /// Supprime une plage de contenu
    pub async fn delete_content_range(
        &self,
        document_id: &str,
        start_index: i32,
        end_index: i32,
    ) -> Result<String> {
        let request = BatchUpdateDocumentRequest {
            requests: Some(vec![Request {
                delete_content_range: Some(DeleteContentRangeRequest {
                    range: Some(Range {
                        start_index: Some(start_index),
                        end_index: Some(end_index),
                        ..Default::default()
                    }),
                }),
                ..Default::default()
            }]),
            ..Default::default()
        };

        self.hub
            .documents()
            .batch_update(request, document_id)
            .doit()
            .await
            .context("Impossible de supprimer la plage")?;

        Ok(format!(
            "Plage [{}, {}] supprimée avec succès.",
            start_index, end_index
        ))
    }

    /// Remplace toutes les occurrences d'un texte
    pub async fn replace_all_text(
        &self,
        document_id: &str,
        search_text: &str,
        replacement_text: &str,
        match_case: bool,
    ) -> Result<String> {
        let request = BatchUpdateDocumentRequest {
            requests: Some(vec![Request {
                replace_all_text: Some(ReplaceAllTextRequest {
                    replace_text: Some(replacement_text.to_string()),
                    contains_text: Some(SubstringMatchCriteria {
                        text: Some(search_text.to_string()),
                        match_case: Some(match_case),
                    }),
                }),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let (_, response) = self
            .hub
            .documents()
            .batch_update(request, document_id)
            .doit()
            .await
            .context("Impossible de remplacer le texte")?;

        let count = response
            .replies
            .as_ref()
            .and_then(|r| r.first())
            .and_then(|r| r.replace_all_text.as_ref())
            .and_then(|r| r.occurrences_changed)
            .unwrap_or(0);

        Ok(format!(
            "{} occurrence(s) de '{}' remplacée(s) par '{}'.",
            count, search_text, replacement_text
        ))
    }

    /// Crée un nouveau document Google Docs
    pub async fn create_document(&self, title: &str) -> Result<String> {
        let doc = Document {
            title: Some(title.to_string()),
            ..Default::default()
        };

        let (_, created_doc) = self
            .hub
            .documents()
            .create(doc)
            .doit()
            .await
            .context("Impossible de créer le document")?;

        let doc_id = created_doc
            .document_id
            .unwrap_or_else(|| "inconnu".to_string());

        Ok(format!(
            "Document '{}' créé avec succès. ID: {}",
            title, doc_id
        ))
    }
}

/// Extrait le texte brut d'un document Google Docs
fn extract_text_from_document(doc: &Document) -> String {
    let mut text = String::new();

    if let Some(title) = &doc.title {
        text.push_str(&format!("=== {} ===\n\n", title));
    }

    if let Some(body) = &doc.body {
        if let Some(content) = &body.content {
            for element in content {
                if let Some(paragraph) = &element.paragraph {
                    if let Some(elements) = &paragraph.elements {
                        for pe in elements {
                            if let Some(text_run) = &pe.text_run {
                                if let Some(content) = &text_run.content {
                                    text.push_str(content);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    text
}
