use std::process::Command;
use crate::utils::{output, paths, errors::Result};
use crate::cli::args::EditTarget;

/// Run edit command
pub fn run(target: EditTarget, name: Option<String>) -> Result<()> {
    let file_path = match target {
        EditTarget::Config => {
            output::info("Opening global configuration");
            paths::config_file()?
        }
        EditTarget::Host => {
            let hostname = name.ok_or_else(|| {
                crate::utils::errors::DeclarchError::Other(
                    "Host name required for edit host".to_string(),
                )
            })?;
            output::info(&format!("Opening host configuration: {}", hostname));
            paths::host_file(&hostname)?
        }
        EditTarget::Module => {
            let module_name = name.ok_or_else(|| {
                crate::utils::errors::DeclarchError::Other(
                    "Module name required for edit module".to_string(),
                )
            })?;
            output::info(&format!("Opening module: {}", module_name));
            paths::module_file(&module_name)?
        }
    };

    // Verify file exists
    if !file_path.exists() {
        return Err(crate::utils::errors::DeclarchError::ConfigNotFound {
            path: file_path,
        });
    }

    // Open with $EDITOR
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

    let status = Command::new(&editor)
        .arg(&file_path)
        .status()
        .map_err(|e| crate::utils::errors::DeclarchError::SystemCommandFailed {
            command: format!("{} {:?}", editor, file_path),
            reason: e.to_string(),
        })?;

    if !status.success() {
        return Err(crate::utils::errors::DeclarchError::SystemCommandFailed {
            command: editor,
            reason: "Editor exited with error".to_string(),
        });
    }

    output::success("File updated");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_targets() {
        // Just verify EditTarget works
        let _config = EditTarget::Config;
        let _host = EditTarget::Host;
        let _module = EditTarget::Module;
    }
}
