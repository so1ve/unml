use anyhow::Result;
use clap::{Parser, Subcommand};

mod clean;
mod dev;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Development automation tasks for UNML", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the application in development mode with auto-reload
    Dev,
    /// Clean build artifacts
    Clean,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Dev => dev::run_dev()?,
        Commands::Clean => clean::run_clean()?,
    }

    Ok(())
}
