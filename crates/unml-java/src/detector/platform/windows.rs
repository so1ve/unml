use std::path::PathBuf;
use std::{env, fs};

use winreg::RegKey;
use winreg::enums::*;

use crate::detector::probe::JavaProbe;
use crate::detector::scanner::DirectoryScanner;
use crate::{JavaInstallation, Result};

pub struct WindowsDetector;

impl WindowsDetector {
    pub async fn detect_from_registry() -> Result<Vec<JavaInstallation>> {
        let mut installations = Vec::new();

        let registry_paths = vec![
            (
                HKEY_LOCAL_MACHINE,
                r"SOFTWARE\JavaSoft\Java Runtime Environment",
            ),
            (
                HKEY_LOCAL_MACHINE,
                r"SOFTWARE\JavaSoft\Java Development Kit",
            ),
            (HKEY_LOCAL_MACHINE, r"SOFTWARE\JavaSoft\JRE"),
            (HKEY_LOCAL_MACHINE, r"SOFTWARE\JavaSoft\JDK"),
            (HKEY_LOCAL_MACHINE, r"SOFTWARE\Eclipse Adoptium\JRE"),
            (HKEY_LOCAL_MACHINE, r"SOFTWARE\Eclipse Adoptium\JDK"),
            (HKEY_LOCAL_MACHINE, r"SOFTWARE\Eclipse Foundation\JDK"),
            (HKEY_LOCAL_MACHINE, r"SOFTWARE\Azul Systems\Zulu"),
            (HKEY_LOCAL_MACHINE, r"SOFTWARE\Microsoft\JDK"),
            (HKEY_LOCAL_MACHINE, r"SOFTWARE\BellSoft\Liberica"),
        ];

        for (hkey, path) in registry_paths {
            if let Ok(key) = RegKey::predef(hkey).open_subkey(path) {
                for version_name in key.enum_keys().filter_map(|k| k.ok()) {
                    if let Ok(version_key) = key.open_subkey(&version_name)
                        && let Ok(java_home) = version_key.get_value::<String, _>("JavaHome")
                    {
                        let java_home_path = PathBuf::from(&java_home);
                        let executable = java_home_path
                            .join("bin")
                            .join(JavaInstallation::executable_name());

                        if let Ok(mut installation) = JavaProbe::probe(&executable).await {
                            installation.home = java_home_path;
                            installations.push(installation);
                        }
                    }
                }
            }
        }

        let oracle_javapath =
            PathBuf::from("C:\\Program Files\\Common Files\\Oracle\\Java\\javapath");
        if oracle_javapath.exists() {
            let executable = oracle_javapath.join(JavaInstallation::executable_name());
            if let Ok(installation) = JavaProbe::probe(&executable).await {
                if let Ok(real_path) = fs::canonicalize(&executable) {
                    if let Some(parent) = real_path.parent().and_then(|p| p.parent()) {
                        let mut resolved_installation = installation;
                        resolved_installation.home = parent.to_path_buf();
                        resolved_installation.executable = real_path;
                        installations.push(resolved_installation);
                    }
                } else {
                    installations.push(installation);
                }
            }
        }

        Ok(installations)
    }

    pub async fn detect_from_system() -> Result<Vec<JavaInstallation>> {
        let mut installations = Vec::new();

        let bases = vec![
            "C:\\Program Files\\Java",
            "C:\\Program Files (x86)\\Java",
            "C:\\Program Files\\Eclipse Adoptium",
            "C:\\Program Files\\Zulu",
            "C:\\Program Files\\Amazon Corretto",
            "C:\\Program Files\\Microsoft",
            "C:\\Program Files\\BellSoft",
            "C:\\Program Files\\Eclipse Foundation",
            "C:\\Program Files\\Common Files\\Oracle\\Java",
        ];

        for base in bases {
            if let Ok(found) = DirectoryScanner::scan(base).await {
                installations.extend(found);
            }
        }

        if let Ok(user_profile) = env::var("USERPROFILE") {
            let user_bases = vec![
                format!("{}\\scoop\\apps", user_profile),
                format!("{}\\.jdks", user_profile),
            ];

            for base in user_bases {
                if let Ok(found) = DirectoryScanner::scan(&base).await {
                    installations.extend(found);
                }
            }
        }

        if let Ok(appdata) = env::var("APPDATA") {
            let appdata_bases = vec![format!("{}\\.minecraft\\runtime", appdata)];

            for base in appdata_bases {
                if let Ok(found) = DirectoryScanner::scan_recursive(&base, 3).await {
                    installations.extend(found);
                }
            }
        }

        if let Ok(localappdata) = env::var("LOCALAPPDATA") {
            let local_bases = vec![
                format!("{}\\Programs", localappdata),
                format!("{}\\Microsoft\\WinGet\\Packages", localappdata),
            ];

            for base in local_bases {
                if let Ok(found) = DirectoryScanner::scan(&base).await {
                    installations.extend(found);
                }
            }
        }

        Ok(installations)
    }
}
