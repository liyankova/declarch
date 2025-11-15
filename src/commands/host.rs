use crate::utils::{output, errors::Result};
use crate::config::loader;
use crate::state;

/// Host command actions
#[derive(Debug)]
pub enum HostAction {
    Enable(String),
    Status,
    List,
}

/// Run host command
pub fn run(action: HostAction) -> Result<()> {
    match action {
        HostAction::Enable(hostname) => enable_host(&hostname),
        HostAction::Status => show_status(),
        HostAction::List => list_hosts(),
    }
}

/// Enable a host and save to state
fn enable_host(hostname: &str) -> Result<()> {
    output::header(&format!("Enabling host: {}", hostname));

    // Verify host exists
    let host_config = loader::load_host(hostname)?;
    output::success(&format!("Found host config: {}", hostname));

    // Load or create state
    let mut state = match state::io::load_state() {
        Ok(s) => s,
        Err(_) => {
            output::info("Creating new state file");
            state::io::init_state(hostname.to_string())?
        }
    };

    // Update host and modules
    state.current_host = hostname.to_string();
    state.active_modules = host_config.modules.clone();
    state.last_sync_method = "enable".to_string();

    // Save state
    state::io::save_state(&state)?;
    output::success(&format!("Enabled host: {}", hostname));

    // Show summary
    output::separator();
    println!("Active modules:");
    for module in &state.active_modules {
        output::item(&module);
    }

    Ok(())
}

/// Show current host status
fn show_status() -> Result<()> {
    output::header("Host Status");

    // Load state
    let state = match state::io::load_state() {
        Ok(s) => s,
        Err(_) => {
            output::warning("No state file found. Run 'declarch init' first");
            return Ok(());
        }
    };

    // Display current host
    output::keyval("Current host", &state.current_host);

    // Display active modules
    if !state.active_modules.is_empty() {
        println!();
        println!("  Modules:");
        for module in &state.active_modules {
            output::indent(&format!("• {}", module), 2);
        }
    }

    // Display package count
    println!();
    output::keyval("Packages", &state.packages.len().to_string());

    Ok(())
}

/// List all available hosts
fn list_hosts() -> Result<()> {
    output::header("Available Hosts");

    // Get current host from state (if exists)
    let current = match state::io::load_state() {
        Ok(s) => Some(s.current_host),
        Err(_) => None,
    };

    // List all hosts
    let hosts = loader::list_hosts()?;

    if hosts.is_empty() {
        output::warning("No hosts found. Run 'declarch init' first");
        return Ok(());
    }

    for host in hosts {
        if let Some(ref current_host) = current {
            if &host == current_host {
                output::item_highlight(&format!("{} (current)", host));
                continue;
            }
        }
        output::item(&host);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host_action_enable() {
        match HostAction::Enable("test".to_string()) {
            HostAction::Enable(name) => assert_eq!(name, "test"),
            _ => panic!("Wrong variant"),
        }
    }
}
