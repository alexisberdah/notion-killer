use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod parser;
mod mapper;

#[derive(Parser)]
#[command(name = "notion-import")]
#[command(about = "Import Notion workspaces into Notion Killer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Import from a Notion export ZIP file
    Zip {
        /// Path to the ZIP file
        #[arg(short, long)]
        file: PathBuf,

        /// Output directory for converted files
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Import directly from Notion API (requires OAuth)
    Api {
        /// Notion API token
        #[arg(short, long)]
        token: String,

        /// Workspace ID to import
        #[arg(short, long)]
        workspace: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Zip { file, output } => {
            println!("Importing from ZIP: {:?}", file);
            println!("Output directory: {:?}", output);
            // TODO: Implement ZIP import
            parser::parse_zip(&file, &output)?;
        }
        Commands::Api { token, workspace } => {
            println!("Importing from Notion API");
            if let Some(ws) = workspace {
                println!("Workspace: {}", ws);
            }
            // TODO: Implement API import
        }
    }

    Ok(())
}
