use std::path::{Path, PathBuf};
use std::sync::Arc;

use tokio::sync::Mutex;

use super::probe::JavaProbe;
use crate::{JavaInstallation, Result};

const SKIP_NAMES: &[&str] = &[
    "node_modules",
    ".git",
    ".idea",
    ".vscode",
    "target",
    "build",
    "dist",
    "out",
    "system32",
    "syswow64",
    "$recycle.bin",
    "windows",
    "appdata",
];

pub struct DirectoryScanner;

impl DirectoryScanner {
    pub async fn scan(base: impl AsRef<Path>) -> Result<Vec<JavaInstallation>> {
        let base = base.as_ref();

        if !base.exists() {
            return Ok(Vec::new());
        }

        let entries = std::fs::read_dir(base)?;

        let mut candidates = Vec::new();
        for entry in entries {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let java_home = entry.path();
                let executable = java_home
                    .join("bin")
                    .join(JavaInstallation::executable_name());

                if executable.exists() {
                    candidates.push((java_home, executable));
                }
            }
        }

        Self::probe_candidates(candidates).await
    }

    pub async fn scan_recursive(
        base: impl AsRef<Path>,
        max_depth: u32,
    ) -> Result<Vec<JavaInstallation>> {
        let base = base.as_ref().to_path_buf();

        if !base.exists() {
            return Ok(Vec::new());
        }

        let candidates = Self::collect_candidates_iterative(&base, max_depth).await;

        Self::probe_candidates(candidates).await
    }

    async fn collect_candidates_iterative(base: &Path, max_depth: u32) -> Vec<(PathBuf, PathBuf)> {
        let mut candidates = Vec::new();
        let mut stack: Vec<(PathBuf, u32)> = vec![(base.to_path_buf(), 0)];

        while let Some((current_path, depth)) = stack.pop() {
            if depth > max_depth {
                continue;
            }

            let executable = current_path
                .join("bin")
                .join(JavaInstallation::executable_name());

            if executable.exists() {
                candidates.push((current_path, executable));
                continue;
            }

            let entries = match tokio::fs::read_dir(&current_path).await {
                Ok(entries) => entries,
                Err(_) => continue,
            };

            let mut entries = entries;
            while let Ok(Some(entry)) = entries.next_entry().await {
                let file_type = match entry.file_type().await {
                    Ok(ft) => ft,
                    Err(_) => continue,
                };

                if file_type.is_dir() {
                    let path = entry.path();
                    if !Self::should_skip_directory(&path) {
                        stack.push((path, depth + 1));
                    }
                }
            }
        }

        candidates
    }

    async fn probe_candidates(
        candidates: Vec<(PathBuf, PathBuf)>,
    ) -> Result<Vec<JavaInstallation>> {
        if candidates.is_empty() {
            return Ok(Vec::new());
        }

        const MAX_CONCURRENT: usize = 8;

        let installations = Arc::new(Mutex::new(Vec::new()));
        let mut handles = Vec::new();

        for (java_home, executable) in candidates {
            let installations = Arc::clone(&installations);

            let handle = tokio::spawn(async move {
                if let Ok(mut installation) = JavaProbe::probe(&executable).await {
                    installation.home = java_home;
                    installations.lock().await.push(installation);
                }
            });

            handles.push(handle);

            if handles.len() >= MAX_CONCURRENT {
                for handle in handles.drain(..) {
                    let _ = handle.await;
                }
            }
        }

        for handle in handles {
            let _ = handle.await;
        }

        let result = Arc::try_unwrap(installations)
            .expect("All tasks have completed, there should be no other references")
            .into_inner();

        Ok(result)
    }

    fn should_skip_directory(path: &Path) -> bool {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();

        SKIP_NAMES.iter().any(|s| name.contains(s))
    }
}
