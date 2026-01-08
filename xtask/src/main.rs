use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use anyhow::Result;
use clap::{Parser, Subcommand};
use xtask_watch::Watch;

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
        Commands::Dev { watch } => run_dev(watch)?,
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

fn run_dev(watch: bool) -> Result<()> {
    let workspace_root = workspace_root()?;

    let crates_dir = workspace_root.join("crates");

    // Best-effort: ensure Ctrl+C kills the spawned dev app window too.
    // - Non-watch: we own the `cargo run` child pid.
    // - Watch: xtask-watch owns the child; we fall back to killing `unml.exe`.
    let child_pid = Arc::new(AtomicU32::new(0));
    install_ctrlc_handler(child_pid.clone())?;

    if !watch {
        println!("Starting development mode...");
        let mut child = spawn_dev_command(&workspace_root).spawn()?;
        child_pid.store(child.id(), Ordering::SeqCst);

        let status = child.wait()?;
        child_pid.store(0, Ordering::SeqCst);
        if !status.success() {
            anyhow::bail!("Development mode failed");
        }
        return Ok(());
    }

    println!("Starting development mode (watching crates/) ...");
    if !crates_dir.exists() {
        anyhow::bail!("Missing crates directory: {}", crates_dir.display());
    }

    let watch = Watch::default().watch_path(&crates_dir);
    let command = spawn_dev_command(&workspace_root);
    watch.run(command)?;

    Ok(())
}

fn workspace_root() -> Result<PathBuf> {
    let xtask_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    xtask_dir
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| anyhow::anyhow!("Failed to determine workspace root"))
}

fn spawn_dev_command(workspace_root: &Path) -> Command {
    let mut command = Command::new("cargo");
    command
        .current_dir(workspace_root)
        .args(["run", "--package", "unml-gui", "--bin", "unml"]);
    command
}

fn install_ctrlc_handler(child_pid: Arc<AtomicU32>) -> Result<()> {
    ctrlc::set_handler(move || {
        let pid = child_pid.load(Ordering::SeqCst);
        if pid != 0 {
            let _ = kill_process_tree(pid);
        } else {
            let _ = kill_dev_app_fallback();
        }

        // 130 is a conventional exit code for SIGINT.
        std::process::exit(130);
    })?;

    Ok(())
}

fn kill_process_tree(pid: u32) -> Result<()> {
    #[cfg(windows)]
    {
        let status = Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/T", "/F"])
            .status()?;

        if !status.success() {
            anyhow::bail!("taskkill failed for pid {pid}");
        }

        return Ok(());
    }

    #[cfg(unix)]
    {
        let status = Command::new("kill")
            .args(["-TERM", &pid.to_string()])
            .status()?;
        if !status.success() {
            anyhow::bail!("kill failed for pid {pid}");
        }
        return Ok(());
    }

    #[allow(unreachable_code)]
    Ok(())
}

fn kill_dev_app_fallback() -> Result<()> {
    #[cfg(windows)]
    {
        // Watch mode: we don't have the child PID (xtask-watch owns it), so kill by
        // image name. This is best-effort and intended for dev only.
        let _ = Command::new("taskkill")
            .args(["/IM", "unml.exe", "/F"])
            .status();
        Ok(())
    }

    #[cfg(not(windows))]
    {
        Ok(())
    }
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
