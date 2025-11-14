use crate::package::trait_impl::PackageManager;
use crate::package::{pacman::PacmanManager, aur::AurManager, flatpak::FlatpakManager};
use crate::config::types::AurHelper;
use crate::state::types::PackageBackend;
use crate::utils::errors::Result;

/// Factory for creating package managers
pub struct PackageManagerFactory;

impl PackageManagerFactory {
    /// Get package manager for given backend
    pub fn get(backend: PackageBackend, aur_helper: AurHelper) -> Result<Box<dyn PackageManager>> {
        match backend {
            PackageBackend::Pacman => Ok(Box::new(PacmanManager)),
            PackageBackend::Aur => Ok(Box::new(AurManager { helper: aur_helper })),
            PackageBackend::Flatpak => Ok(Box::new(FlatpakManager)),
        }
    }
}
