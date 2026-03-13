mod cli;
mod converter;
mod error;
mod google_docs;
mod mapping;
mod markdown;
mod style;
mod sync;

use anyhow::Result;
use clap::Parser;
use tracing::info;

use cli::{Cli, Command};

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

    let cli = Cli::parse();

    match cli.command {
        Command::Push { fichier, doc_id, force } => {
            info!("Push: {} → Google Docs", fichier.display());
            sync::push(&fichier, doc_id.as_deref(), force).await?;
        }
        Command::Pull { fichier, doc_id, force } => {
            info!("Pull: Google Docs → {}", fichier.display());
            sync::pull(&fichier, doc_id.as_deref(), force).await?;
        }
        Command::Status { fichier } => {
            info!("Status: {}", fichier.display());
            sync::status(&fichier).await?;
        }
    }

    Ok(())
}

