use std::fs;
use std::path::PathBuf;
use crate::utils::{output, paths, errors::Result, templates};
use crate::state;
use chrono::Local;
/// Options for init command
#[derive(Debug)]
pub struct InitOptions {
    /// Explicit hostname (if not provided, auto-detect)
    pub host: Option<String>,
    /// Whether to force overwrite existing
    pub force: bool,
}

/// Run init command
pub fn run(options: InitOptions) -> Result<()> {
    output::header("Initializing declarch");

    // Step 1: Detect or use provided hostname
    let hostname = match options.host {
        Some(h) => {
            output::info(&format!("Using provided hostname: {}", h));
            h
        }
        None => {
            let detected = hostname::get()
                .ok()
                .and_then(|h| h.into_string().ok())
                .unwrap_or_else(|| "localhost".to_string());
            output::success(&format!("Detected hostname: {}", detected));
            detected
        }
    };

    // Step 2: Check if declarch already exists
    let config_dir = paths::config_dir()?;
    if config_dir.exists() && !options.force {
        return handle_existing_config(&config_dir, &hostname);
    }

    // Step 3: Create directory structure
    output::info("Creating directory structure...");
    create_directories()?;
    output::success("Directory structure created");

    // Step 4: Generate template files
    output::info("Generating template files...");
    generate_templates(&hostname)?;
    output::success("Template files generated");

    // Step 5: Enable the host (create state)
    output::info("Enabling host...");
    enable_host_state(&hostname)?;
    output::success(&format!("Host '{}' enabled", hostname));

    // Step 6: Show summary
    print_init_summary(&hostname);

    Ok(())
}

/// Handle case where config already exists
fn handle_existing_config(config_dir: &PathBuf, hostname: &str) -> Result<()> {
    output::warning("Declarch configuration already exists!");
    output::keyval("Location", &config_dir.display().to_string());

    // Check if empty
    let has_content = fs::read_dir(config_dir)?
        .next()
        .is_some();

    if !has_content {
        output::info("Directory is empty, proceeding...");
        create_directories()?;
        generate_templates(hostname)?;
        enable_host_state(hostname)?;
        print_init_summary(hostname);
        return Ok(());
    }

    // Ask user what to do
    let choice = output::prompt_choice(
        "What would you like to do?",
        &["Abort", "Backup and reinitialize"],
    );

    match choice {
        Some(0) => {
            output::info("Initialization cancelled");
            std::process::exit(0);
        }
        Some(1) => {
            output::info("Backup and reinitialize...");
            backup_config(config_dir)?;
            create_directories()?;
            generate_templates(hostname)?;
            enable_host_state(hostname)?;
            print_init_summary(hostname);
        }
        _ => {
            output::error("Invalid choice");
            std::process::exit(1);
        }
    }

    Ok(())
}
/// Backup existing config
fn backup_config(config_dir: &PathBuf) -> Result<()> {
    use chrono::Local;
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let backup_dir = config_dir.parent().map(|p| {
        p.join(format!(
            ".declarch.backup-{}",
            timestamp
        ))
    });

    if let Some(backup) = backup_dir {
        fs::rename(config_dir, &backup)?;
        output::success(&format!("Backed up to: {}", backup.display()));
    }

    Ok(())
}

/// Create necessary directories
fn create_directories() -> Result<()> {
    let config_dir = paths::config_dir()?;
    let hosts_dir = paths::hosts_dir()?;
    let modules_dir = paths::modules_dir()?;

    fs::create_dir_all(&config_dir)?;
    fs::create_dir_all(&hosts_dir)?;
    fs::create_dir_all(&modules_dir)?;

    output::indent(&format!("✓ Created: {}", config_dir.display()), 1);
    output::indent(&format!("✓ Created: {}", hosts_dir.display()), 1);
    output::indent(&format!("✓ Created: {}", modules_dir.display()), 1);

    Ok(())
}

/// Generate template files
fn generate_templates(hostname: &str) -> Result<()> {
    // Generate global config
    let config_file = paths::config_file()?;
    if !config_file.exists() {
        let content = templates::default_config();
        fs::write(&config_file, content)?;
        output::indent(&format!("✓ Created: {}", config_file.display()), 1);
    } else {
        output::indent(&format!("⊘ Skipped: {} (already exists)", config_file.display()), 1);
    }

    // Generate host file
    let host_file = paths::host_file(hostname)?;
    if !host_file.exists() {
        let content = templates::default_host(hostname);
        fs::write(&host_file, content)?;
        output::indent(&format!("✓ Created: {}", host_file.display()), 1);
    } else {
        output::indent(&format!("⊘ Skipped: {} (already exists)", host_file.display()), 1);
    }

    // Generate base module
    let base_module = paths::module_file("base")?;
    if !base_module.exists() {
        let content = templates::default_base_module();
        fs::write(&base_module, content)?;
        output::indent(&format!("✓ Created: {}", base_module.display()), 1);
    } else {
        output::indent(&format!("⊘ Skipped: {} (already exists)", base_module.display()), 1);
    }

    Ok(())
}

/// Enable host and create initial state
fn enable_host_state(hostname: &str) -> Result<()> {
    use crate::config::loader;

    // Load host config
    let host_config = loader::load_host(hostname)?;

    // Create state
    let mut state = state::io::init_state(hostname.to_string())?;
    state.active_modules = host_config.modules.clone();
    state.last_sync_method = "init".to_string();

    // Save state
    state::io::save_state(&state)?;

    Ok(())
}

/// Print summary of what was initialized
fn print_init_summary(hostname: &str) {
    output::separator();
    output::success("Declarch initialization complete!");
    println!();
    println!("Next steps:");
    println!("  1. Edit configuration:");
    println!("     declarch edit config          # Global settings");
    println!("     declarch edit host {}         # Host packages", hostname);
    println!("     declarch edit module base     # Base module packages");
    println!();
    println!("  2. Add packages to modules:");
    println!("     vim ~/.config/declarch/modules/base.decl");
    println!();
    println!("  3. Sync packages:");
    println!("     declarch sync --dry-run      # Preview changes");
    println!("     declarch sync                # Apply changes");
    println!();
    println!("  4. View status:");
    println!("     declarch info                # Show installed packages");
    println!("     declarch host status         # Show current host");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_options_default() {
        let opts = InitOptions {
            host: None,
            force: false,
        };
        assert!(opts.host.is_none());
        assert!(!opts.force);
    }

    #[test]
    fn test_init_options_with_host() {
        let opts = InitOptions {
            host: Some("laptop".to_string()),
            force: false,
        };
        assert_eq!(opts.host, Some("laptop".to_string()));
    }
}
