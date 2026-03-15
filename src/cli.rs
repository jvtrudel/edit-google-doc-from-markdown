use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Outil de synchronisation Markdown ↔ Google Docs
#[derive(Parser)]
#[command(
    name = "nou",
    version,
    about = "Synchronisation Markdown ↔ Google Docs"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Publier un fichier Markdown vers un Google Doc
    Push {
        /// Chemin du fichier Markdown à publier
        fichier: PathBuf,

        /// Identifiant du Google Doc cible (optionnel si l'association existe)
        #[arg(long)]
        doc_id: Option<String>,

        /// Écraser le Google Doc même en cas de conflit détecté
        #[arg(long, default_value_t = false)]
        force: bool,
    },

    /// Récupérer un Google Doc en fichier Markdown
    Pull {
        /// Chemin du fichier Markdown de destination (optionnel si l'association existe)
        fichier: Option<PathBuf>,

        /// Identifiant du Google Doc source (optionnel si l'association existe)
        #[arg(long)]
        doc_id: Option<String>,

        /// Écraser le fichier local même en cas de conflit détecté
        #[arg(short = 'f', long, default_value_t = false)]
        force: bool,
    },

    /// Afficher l'état de synchronisation d'un fichier
    Status {
        /// Chemin du fichier Markdown
        fichier: PathBuf,
    },
}
