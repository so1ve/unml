use std::process::Command;

use anyhow::Result;

pub fn run_clean() -> Result<()> {
    println!("Cleaning build artifacts...");

    let status = Command::new("cargo").args(["clean"]).status()?;

    if !status.success() {
        anyhow::bail!("Clean failed");
    }

    println!("Cleaned successfully");
    Ok(())
}
