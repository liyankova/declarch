use std::path::{Path, PathBuf};
use crate::utils::errors::{DeclarchError, Result};

/// Get declarch config directory (~/.config/declarch)
pub fn config_dir() -> Result<PathBuf> {
    let base = dirs::config_dir()
        .ok_or_else(|| DeclarchError::Other("Cannot determine config directory".to_string()))?;
    Ok(base.join("declarch"))
}

/// Get state file path (~/.config/declarch/.state.json)
pub fn state_file() -> Result<PathBuf> {
    Ok(config_dir()?.join(".state.json"))
}

/// Get hosts directory (~/.config/declarch/hosts)
pub fn hosts_dir() -> Result<PathBuf> {
    Ok(config_dir()?.join("hosts"))
}

/// Get specific host file
pub fn host_file(name: &str) -> Result<PathBuf> {
    Ok(hosts_dir()?.join(format!("{}.decl", name)))
}

/// Get modules directory (~/.config/declarch/modules)
pub fn modules_dir() -> Result<PathBuf> {
    Ok(config_dir()?.join("modules"))
}

/// Get specific module file
pub fn module_file(name: &str) -> Result<PathBuf> {
    Ok(modules_dir()?.join(format!("{}.decl", name)))
}

/// Get global config file (~/.config/declarch/config.decl)
pub fn config_file() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.decl"))
}

/// Get log file (~/.config/declarch/declarch.log)
pub fn log_file() -> Result<PathBuf> {
    Ok(config_dir()?.join("declarch.log"))
}

/// Check if path exists
pub fn exists(path: &Path) -> bool {
    path.exists()
}

/// Check if path is directory
pub fn is_dir(path: &Path) -> bool {
    path.is_dir()
}

/// Check if path is file
pub fn is_file(path: &Path) -> bool {
    path.is_file()
}

/// Detect hostname for auto-init
pub fn detect_hostname() -> Result<String> {
    // Try /etc/hostname first (Linux standard)
    if let Ok(content) = std::fs::read_to_string("/etc/hostname") {
        let hostname = content.trim().to_string();
        if !hostname.is_empty() {
            return Ok(hostname);
        }
    }

    // Fallback: hostname command
    let output = std::process::Command::new("hostname")
        .output()
        .map_err(|e| DeclarchError::SystemCommandFailed {
            command: "hostname".to_string(),
            reason: e.to_string(),
        })?;

    String::from_utf8(output.stdout)
        .map_err(|e| DeclarchError::Utf8Error(e))
        .map(|s| s.trim().to_string())
}

/// Expand tilde in paths (e.g., ~/path → /home/user/path)
pub fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]);
        }
    }
    PathBuf::from(path)
}
