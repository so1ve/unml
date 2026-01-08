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
    Dev {
        /// Watch for changes under `crates/` and restart automatically
        #[arg(short, long, default_value_t = true)]
        watch: bool,
    },
    /// Clean build artifacts
    Clean,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Dev { watch } => dev::run_dev(watch)?,
        Commands::Clean => clean::run_clean()?,
    }

    Ok(())
}
