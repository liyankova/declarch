use crate::utils::{output, errors::Result};
use crate::config::{loader, validator, kdl};

/// Run check command
pub fn run(verbose: bool) -> Result<()> {
    output::header("Validating Configuration");

    let mut error_count = 0;
    let mut warning_count = 0;

    // Check global config
    if let Err(e) = check_global_config() {
        output::error(&format!("Global config: {}", e));
        error_count += 1;
    } else if verbose {
        output::success("Global config: OK");
    }

    // Check hosts
    match loader::list_hosts() {
        Ok(hosts) => {
            if hosts.is_empty() {
                output::warning("No hosts found");
            } else {
                for host in hosts {
                    match loader::load_host(&host) {
                        Ok(config) => {
                            match validator::validate_host(&host, &config) {
                                Ok(errors) => {
                                    if errors.is_empty() {
                                        if verbose {
                                            output::success(&format!("Host '{}': OK", host));
                                        }
                                    } else {
                                        for err in errors {
                                            match err.severity {
                                                validator::Severity::Warning => {
                                                    output::warning(&format!("Host '{}': {}", host, err.message));
                                                    warning_count += 1;
                                                }
                                                validator::Severity::Error => {
                                                    output::error(&format!("Host '{}': {}", host, err.message));
                                                    error_count += 1;
                                                }
                                                validator::Severity::Info => {
                                                    output::info(&format!("Host '{}': {}", host, err.message));
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    output::error(&format!("Host '{}': {}", host, e));
                                    error_count += 1;
                                }
                            }
                        }
                        Err(e) => {
                            output::error(&format!("Host '{}': {}", host, e));
                            error_count += 1;
                        }
                    }
                }
            }
        }
        Err(e) => {
            output::error(&format!("Failed to list hosts: {}", e));
            error_count += 1;
        }
    }

    // Check modules
    match loader::list_modules() {
        Ok(modules) => {
            if !modules.is_empty() {
                for module in modules {
                    match loader::load_module(&module) {
                        Ok(config) => {
                            match validator::validate_module(&module, &config) {
                                Ok(errors) => {
                                    if errors.is_empty() {
                                        if verbose {
                                            output::success(&format!("Module '{}': OK", module));
                                        }
                                    } else {
                                        for err in errors {
                                            match err.severity {
                                                validator::Severity::Warning => {
                                                    output::warning(&format!("Module '{}': {}", module, err.message));
                                                    warning_count += 1;
                                                }
                                                validator::Severity::Error => {
                                                    output::error(&format!("Module '{}': {}", module, err.message));
                                                    error_count += 1;
                                                }
                                                validator::Severity::Info => {
                                                    output::info(&format!("Module '{}': {}", module, err.message));
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    output::error(&format!("Module '{}': {}", module, e));
                                    error_count += 1;
                                }
                            }
                        }
                        Err(e) => {
                            output::error(&format!("Module '{}': {}", module, e));
                            error_count += 1;
                        }
                    }
                }
            }
        }
        Err(e) => {
            output::error(&format!("Failed to list modules: {}", e));
            error_count += 1;
        }
    }

    // Summary
    output::separator();
    if error_count == 0 && warning_count == 0 {
        output::success("All configurations valid!");
    } else {
        println!("Summary:");
        if error_count > 0 {
            output::error(&format!("{} error(s) found", error_count));
        }
        if warning_count > 0 {
            output::warning(&format!("{} warning(s) found", warning_count));
        }
    }

    Ok(())
}

fn check_global_config() -> Result<()> {
    use crate::utils::paths;
    let config_file = paths::config_file()?;
    if !config_file.exists() {
        return Ok(()); // Optional
    }
    let _config = kdl::parse_global_config(&config_file)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_runs() {
        // Just verify function exists
        let _ = check_global_config;
    }
}
