use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Complete system state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    /// Currently active host
    pub current_host: String,

    /// Active modules for current host
    pub active_modules: Vec<String>,

    /// All installed packages
    pub packages: Vec<PackageEntry>,

    /// Packages excluded from modules
    pub excluded: Vec<String>,

    /// Last sync timestamp
    pub last_sync: Option<DateTime<Utc>>,

    /// Last sync method (sync, sync --prune, etc)
    pub last_sync_method: String,

    /// Schema version for migrations
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
}

fn default_schema_version() -> u32 {
    1
}

/// Individual package entry in state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PackageEntry {
    /// Package name
    pub name: String,

    /// Backend (pacman, aur, flatpak)
    pub backend: PackageBackend,

    /// Source (which module or override)
    pub from: PackageSource,
}

/// Package backend/manager
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum PackageBackend {
    Pacman,
    Aur,
    Flatpak,
}

impl std::fmt::Display for PackageBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pacman => write!(f, "pacman"),
            Self::Aur => write!(f, "aur"),
            Self::Flatpak => write!(f, "flatpak"),
        }
    }
}

/// Where package came from
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum PackageSource {
    /// From a module
    Module(String),

    /// From host-level package override
    HostOverride,

    /// Explicitly excluded
    Excluded,
}

impl State {
    /// Create new empty state
    pub fn new(hostname: String) -> Self {
        Self {
            current_host: hostname,
            active_modules: vec![],
            packages: vec![],
            excluded: vec![],
            last_sync: None,
            last_sync_method: String::new(),
            schema_version: default_schema_version(),
        }
    }

    /// Check if state is fresh (never synced)
    pub fn is_fresh(&self) -> bool {
        self.last_sync.is_none()
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new("localhost".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_new() {
        let state = State::new("laptop".to_string());
        assert_eq!(state.current_host, "laptop");
        assert!(state.is_fresh());
    }

    #[test]
    fn test_backend_display() {
        assert_eq!(PackageBackend::Pacman.to_string(), "pacman");
        assert_eq!(PackageBackend::Aur.to_string(), "aur");
        assert_eq!(PackageBackend::Flatpak.to_string(), "flatpak");
    }
}
