use std::process::Command;

use anyhow::Result;
use clap::{Parser, Subcommand};

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
    /// Build all workspace crates
    Build {
        /// Build in release mode
        #[arg(short, long)]
        release: bool,
    },
    /// Format all code using rustfmt
    Fmt {
        /// Check formatting without making changes
        #[arg(short, long)]
        check: bool,
    },
    /// Run clippy on all workspace crates
    Clippy {
        /// Automatically fix warnings where possible
        #[arg(short, long)]
        fix: bool,
    },
    /// Run all tests
    Test {
        /// Run tests in release mode
        #[arg(short, long)]
        release: bool,
    },
    /// Check all workspace crates
    Check {
        /// Check all features
        #[arg(long)]
        all_features: bool,
    },
    /// Sort all Cargo.toml
    Sort,
    /// Clean build artifacts
    Clean,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Dev => run_dev()?,
        Commands::Build { release } => run_build(release)?,
        Commands::Fmt { check } => run_fmt(check)?,
        Commands::Clippy { fix } => run_clippy(fix)?,
        Commands::Test { release } => run_test(release)?,
        Commands::Check { all_features } => run_check(all_features)?,
        Commands::Sort => run_sort()?,
        Commands::Clean => run_clean()?,
    }

    Ok(())
}

fn run_dev() -> Result<()> {
    println!("Starting development mode...");

    let status = Command::new("cargo")
        .args(["run", "--package", "unml-gui", "--bin", "unml"])
        .status()?;

    if !status.success() {
        anyhow::bail!("Development mode failed");
    }

    Ok(())
}

fn run_build(release: bool) -> Result<()> {
    println!("Building workspace...");

    let mut args = vec!["build", "--workspace"];
    if release {
        args.push("--release");
    }

    let status = Command::new("cargo").args(&args).status()?;

    if !status.success() {
        anyhow::bail!("Build failed");
    }

    println!("✅ Build completed successfully");
    Ok(())
}

fn run_fmt(check: bool) -> Result<()> {
    if check {
        println!("Checking code formatting...");
    } else {
        println!("Formatting code...");
    }

    let mut args = vec!["fmt", "--all"];
    if check {
        args.push("--check");
    }

    let status = Command::new("cargo").args(&args).status()?;

    if !status.success() {
        if check {
            anyhow::bail!("Code is not formatted correctly. Run 'cargo xtask fmt' to fix.");
        } else {
            anyhow::bail!("Formatting failed");
        }
    }

    if check {
        println!("Code formatting is correct");
    } else {
        println!("Code formatted successfully");
    }
    Ok(())
}

fn run_clippy(fix: bool) -> Result<()> {
    if fix {
        println!("🔧 Running clippy with auto-fix...");
    } else {
        println!("📎 Running clippy...");
    }

    let mut args = vec!["clippy", "--workspace", "--all-targets", "--all-features"];
    if fix {
        args.extend_from_slice(&["--fix", "--allow-dirty"]);
    } else {
        args.push("--");
        args.push("-D");
        args.push("warnings");
    }

    let status = Command::new("cargo").args(&args).status()?;

    if !status.success() {
        anyhow::bail!("Clippy found issues");
    }

    println!("Clippy checks passed");
    Ok(())
}

fn run_test(release: bool) -> Result<()> {
    println!("Running tests...");

    let mut args = vec!["test", "--workspace"];
    if release {
        args.push("--release");
    }

    let status = Command::new("cargo").args(&args).status()?;

    if !status.success() {
        anyhow::bail!("Tests failed");
    }

    println!("All tests passed");
    Ok(())
}

fn run_check(all_features: bool) -> Result<()> {
    println!("Checking workspace...");

    let mut args = vec!["check", "--workspace"];
    if all_features {
        args.push("--all-features");
    }

    let status = Command::new("cargo").args(&args).status()?;

    if !status.success() {
        anyhow::bail!("Check failed");
    }

    println!("Check completed successfully");
    Ok(())
}

fn run_sort() -> Result<()> {
    println!("Sorting Cargo.toml files...");

    let status = Command::new("cargo")
        .args(["sort", "--workspace"])
        .status()?;

    if !status.success() {
        anyhow::bail!("Sort failed");
    }

    println!("Cargo.toml files sorted successfully");
    Ok(())
}

fn run_clean() -> Result<()> {
    println!("Cleaning build artifacts...");

    let status = Command::new("cargo").args(["clean"]).status()?;

    if !status.success() {
        anyhow::bail!("Clean failed");
    }

    println!("Cleaned successfully");
    Ok(())
}
