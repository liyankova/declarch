use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "declarch",
    version = "0.1.0",
    about = "Declarative Arch packages manager",
    long_about = "A declarative package manager for Arch Linux, inspired by NixOS"
)]
pub struct Cli {
    /// Global flags
    #[command(flatten)]
    pub global: GlobalFlags,

    /// Subcommands
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Parser, Debug)]
pub struct GlobalFlags {
    /// Verbose output (show detailed information)
    #[arg(short = 'v', long, global = true)]
    pub verbose: bool,

    /// Quiet mode (suppress non-error output)
    #[arg(short = 'q', long, global = true)]
    pub quiet: bool,

    /// Non-interactive mode (assume yes to all prompts)
    #[arg(short = 'y', long = "yes", global = true)]
    pub yes: bool,

    /// Force operations (override safety checks)
    #[arg(short = 'f', long, global = true)]
    pub force: bool,

    /// Config directory override (future use)
    #[arg(short = 'c', long, global = true)]
    pub config: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialize declarch configuration
    #[command(about = "Initialize declarch setup")]
    Init {
        /// Specify host name explicitly
        #[arg(long)]
        host: Option<String>,
    },

    /// Synchronize packages with configuration
    #[command(about = "Synchronize system packages with .decl files")]
    Sync {
        /// Show what would happen without making changes
        #[arg(long)]
        dry_run: bool,

        /// Remove orphan packages (not in any .decl)
        #[arg(long)]
        prune: bool,

        /// Temporary host override
        #[arg(long)]
        host: Option<String>,
    },

    /// Edit configuration files
    #[command(about = "Edit configuration files")]
    Edit {
        /// What to edit (config, host, module)
        #[arg(value_name = "TYPE")]
        target: EditTarget,

        /// Name (for host or module)
        #[arg(value_name = "NAME")]
        name: Option<String>,
    },

    /// Manage hosts
    #[command(about = "Manage host configurations")]
    Host {
        #[command(subcommand)]
        action: HostAction,
    },

    /// Check configuration validity
    #[command(about = "Verify configuration syntax")]
    Check {
        /// Show detailed validation
        #[arg(long)]
        verbose: bool,
    },

    /// Show system information
    #[command(about = "Show installed packages and modules")]
    Info {
        /// Show differences between .decl and installed
        #[arg(long)]
        diff: bool,
    },

    /// Cleanup system
    #[command(about = "Clean caches and orphan packages")]
    Clean {
        /// Aggressive cleanup (cache + unused flatpak)
        #[arg(long)]
        full: bool,

        /// Also remove orphan packages
        #[arg(long)]
        orphans: bool,
    },
}

#[derive(Debug, Clone)]
pub enum EditTarget {
    Config,
    Host,
    Module,
}

impl std::str::FromStr for EditTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "config" => Ok(EditTarget::Config),
            "host" => Ok(EditTarget::Host),
            "module" => Ok(EditTarget::Module),
            _ => Err(format!("Unknown target: {}", s)),
        }
    }
}

impl std::fmt::Display for EditTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditTarget::Config => write!(f, "config"),
            EditTarget::Host => write!(f, "host"),
            EditTarget::Module => write!(f, "module"),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum HostAction {
    /// Enable a host
    Enable {
        #[arg(value_name = "NAME")]
        name: String,
    },

    /// Show current host status
    Status,

    /// List all available hosts
    List,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_target_parse() {
        assert_eq!(
            "config".parse::<EditTarget>().unwrap().to_string(),
            "config"
        );
    }
}
