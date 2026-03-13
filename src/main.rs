mod agent;
mod google_docs;
mod tools;

use anyhow::{Context, Result};
use tracing::info;

use agent::{AgentConfig, ClaudeAgent};
use google_docs::GoogleDocsClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialiser le logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("edit_google_doc=debug".parse().unwrap()),
        )
        .init();

    // Charger les variables d'environnement depuis .env
    dotenvy::dotenv().ok();

    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .context("La variable ANTHROPIC_API_KEY est requise")?;

    let service_account_path = std::env::var("GOOGLE_SERVICE_ACCOUNT_KEY")
        .unwrap_or_else(|_| "service-account.json".to_string());

    info!("Initialisation du client Google Docs...");
    let docs_client = GoogleDocsClient::new(&service_account_path).await?;

    let config = AgentConfig {
        api_key,
        ..Default::default()
    };

    let mut agent = ClaudeAgent::new(config, docs_client);

    // Lire la demande de l'utilisateur depuis les arguments ou stdin
    let user_request = get_user_request();

    info!("Lancement de l'agent...");
    let result = agent.run(user_request).await?;

    println!("\n=== Résultat final ===\n{}", result);

    Ok(())
}

/// Récupère la demande utilisateur depuis les arguments CLI ou stdin
fn get_user_request() -> String {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if !args.is_empty() {
        return args.join(" ");
    }

    println!("Entrez votre demande pour l'agent (puis appuyez sur Entrée):");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Impossible de lire l'entrée");

    input.trim().to_string()
}

