use crate::package::trait_impl::{PackageManager, InstalledPackage};
use crate::utils::errors::Result;
use crate::config::types::AurHelper;

/// AUR package manager (paru or yay)
pub struct AurManager {
    pub helper: AurHelper,
}

impl PackageManager for AurManager {
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
        match self.helper {
            AurHelper::Paru => "paru",
            AurHelper::Yay => "yay",
        }
    }
}
