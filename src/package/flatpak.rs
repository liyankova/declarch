use crate::package::trait_impl::{PackageManager, InstalledPackage};
use crate::utils::errors::Result;

/// Flatpak package manager
pub struct FlatpakManager;

impl PackageManager for FlatpakManager {
    fn install(&self, _packages: &[String]) -> Result<()> {
        // TODO: Implement
        Ok(())
    }

    fn check(&self, _package: &str) -> Result<bool> {
        // TODO: Implement
        Ok(false)
    }

    fn get_installed(&self) -> Result<Vec<InstalledPackage>> {
        // TODO: Implement
        Ok(vec![])
    }

    fn name(&self) -> &'static str {
        "flatpak"
    }
}
