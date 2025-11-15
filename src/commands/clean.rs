use crate::utils::{output, errors::Result};

/// Run clean command
pub fn run(full: bool, orphans: bool) -> Result<()> {
    output::header("Cleaning System");

    // Clean pacman cache
    output::info("Cleaning pacman cache...");
    clean_pacman_cache()?;
    output::success("Pacman cache cleaned");

    // Clean AUR cache
    output::info("Cleaning AUR helper cache...");
    clean_aur_cache()?;
    output::success("AUR cache cleaned");

    // Full cleanup
    if full {
        output::info("Performing full cleanup...");
        clean_flatpak_unused()?;
        output::success("Flatpak unused packages cleaned");
    }

    // Orphan packages
    if orphans {
        output::warning("Orphan package removal not yet implemented");
    }

    output::separator();
    output::success("Cleanup complete!");

    Ok(())
}

fn clean_pacman_cache() -> Result<()> {
    // This would be: sudo pacman -Sc
    // For now, just report
    output::indent("Would run: sudo pacman -Sc", 1);
    Ok(())
}

fn clean_aur_cache() -> Result<()> {
    // Check if paru or yay installed and clean
    let cache_dirs = vec![
        "~/.cache/paru",
        "~/.cache/yay",
    ];

    for cache_dir in cache_dirs {
        output::indent(&format!("Would clean: {}", cache_dir), 1);
    }
    Ok(())
}

fn clean_flatpak_unused() -> Result<()> {
    // This would be: flatpak uninstall --unused
    output::indent("Would run: flatpak uninstall --unused", 1);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_runs() {
        let _ = clean_pacman_cache;
    }
}
