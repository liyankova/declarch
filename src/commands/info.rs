use colored::Colorize;
use crate::utils::{output, errors::Result};
use crate::state;
use std::collections::HashMap;

/// Run info command
pub fn run(diff: bool) -> Result<()> {
    // Load state
    let state = match state::io::load_state() {
        Ok(s) => s,
        Err(_) => {
            output::warning("No state found. Run 'declarch init' first");
            return Ok(());
        }
    };

    output::header("System Status");

    // Current host
    output::keyval("Host", &state.current_host.cyan().bold().to_string());

    // Modules
    output::count_badge("Modules", state.active_modules.len());
    for module in &state.active_modules {
        output::indent(&format!("• {}", module), 1);
    }

    output::separator();

    // Packages summary
    if state.packages.is_empty() {
        output::warning("No packages installed");
    } else {
        output::count_badge("Packages", state.packages.len());

        // Group by source
        let mut by_source: HashMap<String, usize> = HashMap::new();
        for pkg in &state.packages {
            let source = match &pkg.from {
                crate::state::types::PackageSource::Module(m) => m.clone(),
                crate::state::types::PackageSource::HostOverride => "host-override".to_string(),
                crate::state::types::PackageSource::Excluded => "excluded".to_string(),
            };
            *by_source.entry(source).or_insert(0) += 1;
        }

        for (source, count) in by_source {
            output::indent(
                &format!("{}: {}", source, count.to_string().cyan().bold()),
                1,
            );
        }
    }

    if diff {
        output::separator();
        output::warning("Diff mode not yet implemented");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_runs() {
        let _ = run;
    }
}
