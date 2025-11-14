use kdl::KdlDocument;
use std::path::Path;
use crate::config::types::{HostConfig, ModuleConfig, GlobalConfig};
use crate::config::types::AurHelper;
use crate::utils::errors::{DeclarchError, Result};

/// Parse host configuration from .decl file
pub fn parse_host_file(path: &Path) -> Result<HostConfig> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| DeclarchError::FileReadError {
            path: path.to_path_buf(),
            reason: e.to_string(),
        })?;

    parse_host_content(&content, path)
}

/// Parse host configuration from string content
pub fn parse_host_content(content: &str, _path: &Path) -> Result<HostConfig> {
    let doc: KdlDocument = content
        .parse()
        .map_err(|e| DeclarchError::KdlParseError {
            reason: format!("{:?}", e),
        })?;

    let mut config = HostConfig::default();

    // Parse description
    if let Some(node) = doc.get("description") {
        if let Some(entry) = node.entries().first() {
            if let kdl::KdlValue::String(desc) = entry.value() {
                config.description = Some(desc.to_string());
            }
        }
    }

    // Parse modules
    if let Some(node) = doc.get("modules") {
        config.modules = parse_string_array(node)?;
    }

    // Parse packages
    if let Some(node) = doc.get("packages") {
        config.packages = parse_string_array(node)?;
    }

    // Parse exclude
    if let Some(node) = doc.get("exclude") {
        config.exclude = parse_string_array(node)?;
    }

    // Parse conflicts
    if let Some(node) = doc.get("conflicts") {
        config.conflicts = parse_string_array(node)?;
    }

    Ok(config)
}

/// Parse module configuration from .decl file
pub fn parse_module_file(path: &Path) -> Result<ModuleConfig> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| DeclarchError::FileReadError {
            path: path.to_path_buf(),
            reason: e.to_string(),
        })?;

    parse_module_content(&content, path)
}

/// Parse module configuration from string content
pub fn parse_module_content(content: &str, _path: &Path) -> Result<ModuleConfig> {
    let doc: KdlDocument = content
        .parse()
        .map_err(|e| DeclarchError::KdlParseError {
            reason: format!("{:?}", e),
        })?;

    let mut config = ModuleConfig::default();

    // Parse description
    if let Some(node) = doc.get("description") {
        if let Some(entry) = node.entries().first() {
            if let kdl::KdlValue::String(desc) = entry.value() {
                config.description = Some(desc.to_string());
            }
        }
    }

    // Parse packages
    if let Some(node) = doc.get("packages") {
        config.packages = parse_string_array(node)?;
    }

    Ok(config)
}

/// Parse global config from config.decl
pub fn parse_global_config(path: &Path) -> Result<GlobalConfig> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| DeclarchError::FileReadError {
            path: path.to_path_buf(),
            reason: e.to_string(),
        })?;

    parse_global_config_content(&content, path)
}

/// Parse global config from string content
fn parse_global_config_content(content: &str, _path: &Path) -> Result<GlobalConfig> {
    let doc: KdlDocument = content
        .parse()
        .map_err(|e| DeclarchError::KdlParseError {
            reason: format!("{:?}", e),
        })?;

    let mut config = GlobalConfig::default();

    // Parse aur_helper
    if let Some(node) = doc.get("aur_helper") {
        if let Some(entry) = node.entries().first() {
            if let kdl::KdlValue::String(helper) = entry.value() {
                config.aur_helper = match helper.as_str() {
                    "paru" => AurHelper::Paru,
                    "yay" => AurHelper::Yay,
                    _ => {
                        return Err(DeclarchError::InvalidSyntax {
                            file: "config.decl".to_string(),
                            line: 0,
                            message: format!("Unknown aur_helper: {}", helper),
                        })
                    }
                };
            }
        }
    }

    Ok(config)
}

/// Helper to parse string arrays from KDL
/// Handles both: packages "a" "b" "c" and packages [a b c]
fn parse_string_array(node: &kdl::KdlNode) -> Result<Vec<String>> {
    let mut result = Vec::new();

    // Handle inline entries: packages "a" "b" "c"
    for entry in node.entries() {
        if let kdl::KdlValue::String(s) = entry.value() {
            result.push(s.to_string());
        }
    }

    // Handle children nodes (array syntax)
    if let Some(children) = node.children() {
        for child_node in children.nodes() {
            // Each child node contains entries
            for entry in child_node.entries() {
                if let kdl::KdlValue::String(s) = entry.value() {
                    result.push(s.to_string());
                }
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_module_content_inline() {
        // KDL syntax: node name "arg1" "arg2" ...
        let content = r#"description "Test module"
packages "zsh" "git"
"#;
        let path = Path::new("test.decl");
        let config = parse_module_content(content, path).unwrap();
        assert_eq!(config.description, Some("Test module".to_string()));
        assert_eq!(config.packages.len(), 2);
        assert!(config.packages.contains(&"zsh".to_string()));
        assert!(config.packages.contains(&"git".to_string()));
    }

    #[test]
    fn test_parse_host_content() {
        // Proper KDL syntax
        let content = r#"description "Test host"
modules "base" "tools"
packages "neovim" "flatpak:obsidian"
exclude "vim"
conflicts "sway"
"#;
        let path = Path::new("test.decl");
        let config = parse_host_content(content, path).unwrap();
        assert_eq!(config.description, Some("Test host".to_string()));
        assert_eq!(config.modules.len(), 2);
        assert_eq!(config.packages.len(), 2);
        assert_eq!(config.exclude.len(), 1);
        assert_eq!(config.conflicts.len(), 1);
    }

    #[test]
    fn test_parse_global_config_from_content() {
        let content = r#"aur_helper "paru""#;
        let path = Path::new("config.decl");
        let config = parse_global_config_content(content, path).unwrap();
        assert_eq!(config.aur_helper, AurHelper::Paru);
    }

    #[test]
    fn test_parse_empty_config() {
        let content = "";
        let path = Path::new("test.decl");
        let config = parse_module_content(content, path).unwrap();
        assert_eq!(config.packages.len(), 0);
    }

    #[test]
    fn test_parse_yay_helper() {
        let content = r#"aur_helper "yay""#;
        let path = Path::new("config.decl");
        let config = parse_global_config_content(content, path).unwrap();
        assert_eq!(config.aur_helper, AurHelper::Yay);
    }
}
