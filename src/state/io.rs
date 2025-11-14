use crate::state::types::State;
use crate::utils::errors::{DeclarchError, Result};
use crate::utils::paths;
use std::fs;

/// Load state from ~/.config/declarch/.state.json
pub fn load_state() -> Result<State> {
    let state_path = paths::state_file()?;

    if !state_path.exists() {
        return Err(DeclarchError::ConfigNotFound {
            path: state_path,
        });
    }

    let content = fs::read_to_string(&state_path)
        .map_err(|e| DeclarchError::FileReadError {
            path: state_path.clone(),
            reason: e.to_string(),
        })?;

    serde_json::from_str(&content)
        .map_err(|e| DeclarchError::StateDeserializeError {
            reason: e.to_string(),
        })
}

/// Save state to ~/.config/declarch/.state.json
pub fn save_state(state: &State) -> Result<()> {
    let state_path = paths::state_file()?;

    // Ensure directory exists
    if let Some(parent) = state_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| DeclarchError::FileWriteError {
                path: parent.to_path_buf(),
                reason: e.to_string(),
            })?;
    }

    let content = serde_json::to_string_pretty(state)
        .map_err(|e| DeclarchError::Other(format!("Failed to serialize state: {}", e)))?;

    fs::write(&state_path, content)
        .map_err(|e| DeclarchError::FileWriteError {
            path: state_path,
            reason: e.to_string(),
        })?;

    Ok(())
}

/// Create fresh state for new host
pub fn init_state(hostname: String) -> Result<State> {
    Ok(State::new(hostname))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_roundtrip() {
        let state = State::new("test-host".to_string());
        let json = serde_json::to_string(&state).unwrap();
        let loaded: State = serde_json::from_str(&json).unwrap();
        assert_eq!(state.current_host, loaded.current_host);
    }
}
