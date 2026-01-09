use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, mpsc};
use std::time::Duration;

use anyhow::Result;
use notify::{RecursiveMode, Watcher};

pub fn run_dev() -> Result<()> {
    let workspace_root = workspace_root()?;
    let crates_dir = workspace_root.join("crates");

    let child_pid = Arc::new(AtomicU32::new(0));
    install_ctrlc_handler(child_pid.clone())?;

    run_dev_watch(&workspace_root, &crates_dir, child_pid)
}

const IGNORED_PATHS: &[&str] = &["target", ".git"];

fn run_dev_watch(
    workspace_root: &Path,
    crates_dir: &Path,
    child_pid: Arc<AtomicU32>,
) -> Result<()> {
    println!("Starting development mode...");
    if !crates_dir.exists() {
        anyhow::bail!("Missing crates directory: {}", crates_dir.display());
    }

    // Watch for changes and restart the dev process.
    // Keep this intentionally simple: best-effort debounce and restart.
    let (tx, rx) = mpsc::channel::<()>();
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if let Ok(event) = res {
            // Ignore noisy paths.
            if event.paths.iter().any(|p| {
                p.components().any(|c| {
                    let s = c.as_os_str().to_string_lossy();
                    IGNORED_PATHS.contains(&s.as_ref())
                })
            }) {
                return;
            }

            // Coalesce events: send a unit signal; receiver will debounce/drain.
            let _ = tx.send(());
        }
    })?;

    watcher.watch(crates_dir, RecursiveMode::Recursive)?;

    loop {
        println!("(re)starting unml-gui...");

        let mut child = spawn_dev_command(workspace_root).spawn()?;
        child_pid.store(child.id(), Ordering::SeqCst);

        // Inner loop: wait for either file change or child exit.
        loop {
            if let Some(status) = child.try_wait()? {
                child_pid.store(0, Ordering::SeqCst);
                if status.success() {
                    // User closed the window (normal exit): stop dev watcher.
                    println!("dev app exited; stopping.");
                    return Ok(());
                }

                eprintln!("dev app exited with {status}; waiting for changes...");

                // If the app crashes/exits non-zero, restart only after a change to avoid
                // rapid crash loops.
                let _ = rx.recv();
                break;
            }

            match rx.recv_timeout(Duration::from_millis(200)) {
                Ok(()) => {
                    // Debounce: drain burst, then wait a tiny bit for writes to settle.
                    while rx.try_recv().is_ok() {}
                    std::thread::sleep(Duration::from_millis(150));

                    let pid = child.id();
                    let _ = kill_process_tree(pid);
                    let _ = child.wait();
                    child_pid.store(0, Ordering::SeqCst);
                    break;
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // keep polling
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    anyhow::bail!("watch channel disconnected");
                }
            }
        }
    }
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
            let _ = kill_own_children();
        }

        std::process::exit(0);
    })?;

    Ok(())
}

fn kill_process_tree(pid: u32) -> Result<()> {
    let config = kill_tree::Config {
        signal: "SIGKILL".to_string(),
        ..Default::default()
    };

    // Best-effort: by the time we try to kill, the process might already be gone.
    match kill_tree::blocking::kill_tree_with_config(pid, &config) {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("kill_tree failed for pid {pid}: {err}");
            Ok(())
        }
    }
}

fn kill_own_children() -> Result<()> {
    let current_process_id = std::process::id();
    let config = kill_tree::Config {
        signal: "SIGKILL".to_string(),
        include_target: false,
    };

    match kill_tree::blocking::kill_tree_with_config(current_process_id, &config) {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("kill_tree failed for current process children: {err}");
            Ok(())
        }
    }
}
