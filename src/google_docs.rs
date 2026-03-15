#![allow(dead_code)]
//! Transport : lecture/écriture de Google Docs via l'API Google (REQ-008)
//!
//! Ce module gère uniquement le transport — les opérations de lecture et
//! d'écriture sur l'API Google Docs. La conversion du contenu et la gestion
//! du style sont déléguées aux modules `converter` et `style`.

use anyhow::{Context, Result};
use google_docs1::Docs;
use google_docs1::api::BatchUpdateDocumentRequest;
use google_docs1::api::Document;
use google_docs1::api::Request;
use hyper::Client;
use hyper_rustls::HttpsConnectorBuilder;
use yup_oauth2::{ServiceAccountAuthenticator, read_service_account_key};

/// Client Google Docs utilisant un compte de service (REQ-007)
pub struct GoogleDocsClient {
    hub: Docs<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
}

impl GoogleDocsClient {
    /// Crée un nouveau client en utilisant un fichier de clé de compte de service
    pub async fn new(service_account_key_path: &str) -> Result<Self> {
        let key_path = std::path::Path::new(service_account_key_path);
        if !key_path.exists() {
            anyhow::bail!(
                "Fichier de clé du compte de service introuvable : '{}'. \
                 Définissez la variable d'environnement SERVICE_ACCOUNT_KEY_PATH \
                 ou placez le fichier 'service-account.json' dans le répertoire courant.",
                service_account_key_path
            );
        }

        let sa_key = read_service_account_key(service_account_key_path)
            .await
            .with_context(|| format!(
                "Impossible de lire le fichier de clé du compte de service '{}'. \
                 Vérifiez que le fichier est un JSON valide de compte de service Google.",
                service_account_key_path
            ))?;

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

    /// Lit le document complet (structure Google Docs)
    ///
    /// Retourne le document avec tout son contenu et ses styles,
    /// pour être traité par les modules `converter` et `style`.
    pub async fn get_document(&self, document_id: &str) -> Result<Document> {
        let (_, doc) = self
            .hub
            .documents()
            .get(document_id)
            .doit()
            .await
            .context(format!("Impossible de lire le document '{}'", document_id))?;
        Ok(doc)
    }

    /// Envoie un batch de requêtes pour modifier un document
    ///
    /// Les requêtes sont exécutées dans l'ordre. Les index sont positionnels
    /// et changent après chaque opération — l'appelant doit en tenir compte.
    pub async fn batch_update(&self, document_id: &str, requests: Vec<Request>) -> Result<()> {
        if requests.is_empty() {
            return Ok(());
        }

        let batch = BatchUpdateDocumentRequest {
            requests: Some(requests),
            ..Default::default()
        };

        self.hub
            .documents()
            .batch_update(batch, document_id)
            .doit()
            .await
            .context(format!(
                "Impossible d'appliquer les modifications au document '{}'",
                document_id
            ))?;

        Ok(())
    }

    /// Crée un nouveau document Google Docs vide
    ///
    /// Retourne l'identifiant du document créé.
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
            .ok_or_else(|| anyhow::anyhow!("Le document créé n'a pas d'identifiant"))?;

        Ok(doc_id)
    }
}
