use crate::utils::errors::Result;
use std::fmt;

/// Trait for different package managers
pub trait PackageManager: Send + Sync {
    /// Install packages
    fn install(&self, packages: &[String]) -> Result<()>;

    /// Check if package exists
    fn check(&self, package: &str) -> Result<bool>;

    /// Get list of installed packages
    fn get_installed(&self) -> Result<Vec<InstalledPackage>>;

    /// Manager name
    fn name(&self) -> &'static str;
}

/// Installed package information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
}

impl fmt::Display for InstalledPackage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.version)
    }
}
