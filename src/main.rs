mod cli;
mod converter;
mod error;
mod google_docs;
mod mapping;
mod markdown;
mod style;
mod sync;

use anyhow::{Result, bail};
use clap::Parser;
use tracing::info;

use cli::{Cli, Command};

#[tokio::main]
async fn main() -> Result<()> {
    // Charger les variables d'environnement depuis .env
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    // Initialiser le logging (silencieux = erreurs seulement)
    let default_level = if cli.silent { "error" } else { "debug" };
    let filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive(format!("edit_google_doc={}", default_level).parse().unwrap());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();

    match cli.command {
        Command::Push {
            fichier,
            doc_id,
            force,
        } => {
            // Résoudre le fichier : argument ou persistance
            let fichier = match fichier {
                Some(f) => f,
                None => match mapping::get_current_file()? {
                    Some(f) => f,
                    None => bail!(
                        "Aucun fichier spécifié et aucun fichier courant.\nUtilisez : nou push <fichier.md> --doc-id <id>"
                    ),
                },
            };
            info!("Push: {} → Google Docs", fichier.display());
            sync::push(&fichier, doc_id.as_deref(), force).await?;
        }
        Command::Pull {
            fichier,
            doc_id,
            force,
        } => {
            // Résoudre le fichier : argument ou persistance
            let fichier = match fichier {
                Some(f) => f,
                None => match mapping::get_current_file()? {
                    Some(f) => f,
                    None => bail!(
                        "Aucun fichier spécifié et aucun fichier courant.\nUtilisez : nou pull <fichier.md> --doc-id <id>"
                    ),
                },
            };
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
