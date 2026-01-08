mod app;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("UNML - Minecraft Launcher");
    println!("Framework initialized successfully!");
    println!();
    println!("Available crates:");
    println!("  - unml-core: Core traits and types");
    println!("  - unml-java: Java detection and management");
    println!("  - unml-download: Download providers");
    println!("  - unml-launcher: Game launcher");
    println!("  - unml-mods: Mod management");
    println!("  - unml-auth: Authentication");
    println!();

    // 测试 Java 检测
    println!("Testing Java detection...");
    let detector = unml_java::JavaDetector::new();
    match detector.detect_all().await {
        Ok(installations) => {
            println!("Found {} Java installation(s):", installations.len());
            for java in installations {
                println!(
                    "  - Java {} ({}) at {:?}",
                    java.major_version,
                    java.vendor.as_deref().unwrap_or("Unknown"),
                    java.executable
                );
            }
        }
        Err(e) => {
            println!("  Error: {}", e);
        }
    }

    println!();
    println!("GUI will be implemented with GPUI in future updates.");

    Ok(())
}
