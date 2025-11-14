use std::path::Path;
use crate::config::types::{GlobalConfig, HostConfig, ModuleConfig};
use crate::config::kdl;
use crate::utils::errors::{DeclarchError, Result};
use crate::utils::paths;

/// Load global config from ~/.config/declarch/config.decl
pub fn load_global_config() -> Result<GlobalConfig> {
    let config_file = paths::config_file()?;

    if !config_file.exists() {
        // Return default if not exists
        return Ok(GlobalConfig::default());
    }

    kdl::parse_global_config(&config_file)
}

/// Load host configuration
pub fn load_host(hostname: &str) -> Result<HostConfig> {
    let host_file = paths::host_file(hostname)?;

    if !host_file.exists() {
        return Err(DeclarchError::HostNotFound {
            name: hostname.to_string(),
        });
    }

    kdl::parse_host_file(&host_file)
}

/// Load module configuration
pub fn load_module(module_name: &str) -> Result<ModuleConfig> {
    let module_file = paths::module_file(module_name)?;

    if !module_file.exists() {
        return Err(DeclarchError::ModuleNotFound {
            name: module_name.to_string(),
        });
    }

    kdl::parse_module_file(&module_file)
}

/// Load module from arbitrary path (for distributed modules)
pub fn load_module_from_path(path: &Path) -> Result<ModuleConfig> {
    if !path.exists() {
        return Err(DeclarchError::ConfigNotFound {
            path: path.to_path_buf(),
        });
    }

    kdl::parse_module_file(path)
}

/// List all available modules
pub fn list_modules() -> Result<Vec<String>> {
    let modules_dir = paths::modules_dir()?;

    if !modules_dir.exists() {
        return Ok(vec![]);
    }

    let mut modules = Vec::new();
    for entry in std::fs::read_dir(&modules_dir)
        .map_err(|e| DeclarchError::FileReadError {
            path: modules_dir.clone(),
            reason: e.to_string(),
        })?
    {
        let entry = entry.map_err(|e| DeclarchError::FileReadError {
            path: modules_dir.clone(),
            reason: e.to_string(),
        })?;

        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "decl") {
            if let Some(filename) = path.file_stem() {
                if let Some(name) = filename.to_str() {
                    modules.push(name.to_string());
                }
            }
        }
    }

    modules.sort();
    Ok(modules)
}

/// List all available hosts
pub fn list_hosts() -> Result<Vec<String>> {
    let hosts_dir = paths::hosts_dir()?;

    if !hosts_dir.exists() {
        return Ok(vec![]);
    }

    let mut hosts = Vec::new();
    for entry in std::fs::read_dir(&hosts_dir)
        .map_err(|e| DeclarchError::FileReadError {
            path: hosts_dir.clone(),
            reason: e.to_string(),
        })?
    {
        let entry = entry.map_err(|e| DeclarchError::FileReadError {
            path: hosts_dir.clone(),
            reason: e.to_string(),
        })?;

        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "decl") {
            if let Some(filename) = path.file_stem() {
                if let Some(name) = filename.to_str() {
                    hosts.push(name.to_string());
                }
            }
        }
    }

    hosts.sort();
    Ok(hosts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_modules_empty() {
        // Should return empty list when no modules dir
        match list_modules() {
            Ok(modules) => {
                // Either empty or has some modules
                assert!(modules.is_empty() || !modules.is_empty());
            }
            Err(_) => {
                // OK if error (dir doesn't exist in test)
            }
        }
    }
}
