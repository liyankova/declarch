use crate::config::loader;
use crate::config::types::{HostConfig, ModuleConfig};
use crate::utils::errors::Result;

/// Validation error details
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

/// Validate host configuration
pub fn validate_host(_hostname: &str, config: &HostConfig) -> Result<Vec<ValidationError>> {
    let mut errors = Vec::new();

    // Check that all referenced modules exist
    for module_name in &config.modules {
        // Handle local: prefix
        if module_name.starts_with("local:") {
            let path_str = &module_name[6..];
            let expanded = crate::utils::paths::expand_tilde(path_str);
            if !expanded.exists() {
                errors.push(ValidationError {
                    message: format!("Module path not found: {}", path_str),
                    severity: Severity::Error,
                });
            }
        } else {
            // Standard module
            match loader::load_module(module_name) {
                Ok(_) => {}
                Err(_) => {
                    errors.push(ValidationError {
                        message: format!("Module not found: {}", module_name),
                        severity: Severity::Warning,
                    });
                }
            }
        }
    }

    // Check for duplicate packages in modules
    let mut all_packages = std::collections::HashSet::new();
    for package in &config.packages {
        if !all_packages.insert(package.clone()) {
            errors.push(ValidationError {
                message: format!("Duplicate package in host: {}", package),
                severity: Severity::Warning,
            });
        }
    }

    Ok(errors)
}

/// Validate module configuration
pub fn validate_module(module_name: &str, config: &ModuleConfig) -> Result<Vec<ValidationError>> {
    let mut errors = Vec::new();

    // Check for duplicate packages
    let mut seen = std::collections::HashSet::new();
    for package in &config.packages {
        if !seen.insert(package.clone()) {
            errors.push(ValidationError {
                message: format!("Duplicate package in module {}: {}", module_name, package),
                severity: Severity::Warning,
            });
        }
    }

    Ok(errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::types::HostConfig;

    #[test]
    fn test_validate_empty_host() {
        let config = HostConfig::default();
        let errors = validate_host("test", &config).unwrap();
        assert!(errors.is_empty());
    }
}
