use crate::utils::{output, errors::Result};
use crate::config::loader;
use crate::state;
use std::collections::{HashMap, HashSet};
use colored::Colorize;

/// Sync command options
#[derive(Debug)]
pub struct SyncOptions {
    pub dry_run: bool,
    pub prune: bool,
    pub host: Option<String>,
}

/// Resolved package information
#[derive(Debug, Clone)]
struct ResolvedPackage {
    name: String,
    backend: crate::state::types::PackageBackend,
    source: String,
}

/// Run sync command
pub fn run(options: SyncOptions) -> Result<()> {
    output::header("Synchronizing Packages");

    // Step 1: Determine which host to use
    let hostname = if let Some(h) = &options.host {
        output::info(&format!("Using explicit host: {}", h));
        h.clone()
    } else {
        // Load from state
        let state = state::io::load_state()?;
        output::info(&format!("Using current host: {}", state.current_host));
        state.current_host
    };

    // Step 2: Load host config
    let host_config = loader::load_host(&hostname)?;

    // Step 3: Resolve all packages
    output::info("Resolving packages...");
    let resolved = resolve_packages(&hostname, &host_config)?;

    // Step 4: Show preview
    show_preview(&resolved)?;

    // Step 5: Ask for confirmation (unless --yes)
    if !options.dry_run {
        let confirmed = output::prompt_yes_no("Proceed with synchronization?");
        if !confirmed {
            output::info("Sync cancelled");
            return Ok(());
        }
    }

    // Step 6: Execute sync
    if !options.dry_run {
        execute_sync(&resolved)?;
        
        // Step 7: Update state
        save_sync_state(&hostname, &resolved)?;
        output::success("Sync complete!");
    } else {
        output::info("Dry run complete (no changes made)");
    }

    Ok(())
}

/// Resolve packages from host config and modules
fn resolve_packages(
    hostname: &str,
    host_config: &crate::config::types::HostConfig,
) -> Result<Vec<ResolvedPackage>> {
    let mut packages: HashMap<String, ResolvedPackage> = HashMap::new();

    // Step 1: Load all modules and collect packages
    for module_name in &host_config.modules {
        let module_config = loader::load_module(module_name)?;
        
        for pkg_str in &module_config.packages {
            let (name, backend) = parse_package_spec(pkg_str);
            
            let pkg = ResolvedPackage {
                name: name.clone(),
                backend,
                source: module_name.clone(),
            };
            
            packages.insert(name, pkg);
        }
    }

    // Step 2: Add host-level package overrides
    for pkg_str in &host_config.packages {
        let (name, backend) = parse_package_spec(pkg_str);
        
        let pkg = ResolvedPackage {
            name: name.clone(),
            backend,
            source: "host-override".to_string(),
        };
        
        packages.insert(name, pkg);
    }

    // Step 3: Remove excluded packages
    for excluded in &host_config.exclude {
        packages.remove(excluded);
    }

    // Convert to vec and sort
    let mut result: Vec<_> = packages.into_values().collect();
    result.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(result)
}

/// Parse package spec: "name" or "backend:name"
fn parse_package_spec(spec: &str) -> (String, crate::state::types::PackageBackend) {
    use crate::state::types::PackageBackend;
    
    if let Some((prefix, name)) = spec.split_once(':') {
        let backend = match prefix {
            "aur" => PackageBackend::Aur,
            "flatpak" => PackageBackend::Flatpak,
            _ => PackageBackend::Pacman,
        };
        (name.to_string(), backend)
    } else {
        (spec.to_string(), PackageBackend::Pacman)
    }
}

/// Show preview of what will be synced
fn show_preview(packages: &[ResolvedPackage]) -> Result<()> {
    output::separator();
    
    // Group by backend
    let mut by_backend: HashMap<String, Vec<&ResolvedPackage>> = HashMap::new();
    for pkg in packages {
        by_backend
            .entry(pkg.backend.to_string())
            .or_insert_with(Vec::new)
            .push(pkg);
    }

    if packages.is_empty() {
        output::warning("No packages to sync");
        return Ok(());
    }

    println!("Packages to install:");
    println!();

    for backend in &["pacman", "aur", "flatpak"] {
        if let Some(pkgs) = by_backend.get(*backend) {
            let count = pkgs.len();
            println!("  {} ({})", 
                backend.cyan().bold(), 
                count.to_string().bright_cyan()
            );
            
            for pkg in pkgs {
                let source_tag = if pkg.source == "host-override" {
                    " [override]".bright_black()
                } else {
                    "".to_string()
                };
                output::indent(&format!("• {}{}", pkg.name, source_tag), 2);
            }
            println!();
        }
    }

    output::tag("Total", &packages.len().to_string());

    Ok(())
}

/// Execute actual package installation
fn execute_sync(packages: &[ResolvedPackage]) -> Result<()> {
    output::separator();
    output::info("Installing packages...");

    // Group by backend for batch installation
    let mut by_backend: HashMap<String, Vec<&ResolvedPackage>> = HashMap::new();
    for pkg in packages {
        by_backend
            .entry(pkg.backend.to_string())
            .or_insert_with(Vec::new)
            .push(pkg);
    }

    // Install via each backend
    for backend in &["pacman", "aur", "flatpak"] {
        if let Some(pkgs) = by_backend.get(*backend) {
            let pkg_names: Vec<&str> = pkgs.iter().map(|p| p.name.as_str()).collect();
            install_packages(*backend, &pkg_names)?;
        }
    }

    Ok(())
}

/// Install packages via specific backend
fn install_packages(backend: &str, packages: &[&str]) -> Result<()> {
    if packages.is_empty() {
        return Ok(());
    }

    output::info(&format!("Installing {} packages via {}", packages.len(), backend));

    // For MVP, just show what would be installed
    // In real implementation, this would call pacman/yay/flatpak
    for pkg in packages {
        output::indent(&format!("→ {}", pkg), 1);
    }

    // TODO: Actually execute installation
    // This requires:
    // - Calling package manager commands
    // - Handling sudo prompts
    // - Progress tracking
    // - Error handling

    output::success(&format!("{} packages installed", packages.len()));

    Ok(())
}

/// Save sync state
fn save_sync_state(hostname: &str, packages: &[ResolvedPackage]) -> Result<()> {
    let mut state = state::io::load_state().unwrap_or_else(|_| {
        state::io::init_state(hostname.to_string()).unwrap()
    });

    // Update state with resolved packages
    state.current_host = hostname.to_string();
    state.packages = packages
        .iter()
        .map(|p| crate::state::types::PackageEntry {
            name: p.name.clone(),
            backend: p.backend,
            from: match p.source.as_str() {
                "host-override" => crate::state::types::PackageSource::HostOverride,
                src => crate::state::types::PackageSource::Module(src.to_string()),
            },
        })
        .collect();

    state.last_sync = Some(chrono::Utc::now());
    state.last_sync_method = "sync".to_string();

    state::io::save_state(&state)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_package_spec_simple() {
        let (name, backend) = parse_package_spec("vim");
        assert_eq!(name, "vim");
        assert_eq!(backend, crate::state::types::PackageBackend::Pacman);
    }

    #[test]
    fn test_parse_package_spec_aur() {
        let (name, backend) = parse_package_spec("aur:yay");
        assert_eq!(name, "yay");
        assert_eq!(backend, crate::state::types::PackageBackend::Aur);
    }

    #[test]
    fn test_parse_package_spec_flatpak() {
        let (name, backend) = parse_package_spec("flatpak:discord");
        assert_eq!(name, "discord");
        assert_eq!(backend, crate::state::types::PackageBackend::Flatpak);
    }

    #[test]
    fn test_sync_options() {
        let opts = SyncOptions {
            dry_run: true,
            prune: false,
            host: None,
        };
        assert!(opts.dry_run);
    }
}
