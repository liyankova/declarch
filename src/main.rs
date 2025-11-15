use clap::Parser;
use declarch::cli::args::{Cli, Command, HostAction};
use declarch::utils::output;
use declarch::commands;

fn main() {
    let args = Cli::parse();

    // Run command
    match run(&args) {
        Ok(_) => {
            std::process::exit(0);
        }
        Err(e) => {
            output::error(&format!("{}", e));
            std::process::exit(1);
        }
    }
}

fn run(args: &Cli) -> declarch::utils::errors::Result<()> {
    // Dispatch to appropriate command handler
    match &args.command {
        Some(Command::Init { host }) => {
            commands::init::run(commands::init::InitOptions {
                host: host.clone(),
                force: args.global.force,
            })
        }
        Some(Command::Sync {
            dry_run: _,
            prune: _,
            host: _,
        }) => {
            output::info("Sync command not yet implemented");
            Ok(())
        }
        Some(Command::Edit { target, name }) => {
            commands::edit::run(target.clone(), name.clone())
        }
        Some(Command::Host { action }) => {
            let host_action = match action {
                HostAction::Enable { name } => {
                    commands::host::HostAction::Enable(name.clone())
                }
                HostAction::Status => commands::host::HostAction::Status,
                HostAction::List => commands::host::HostAction::List,
            };
            commands::host::run(host_action)
        }
        Some(Command::Check { verbose }) => {
            commands::check::run(*verbose)
        }
        Some(Command::Info { diff }) => {
            commands::info::run(*diff)
        
        }
        Some(Command::Clean { full, orphans }) => {
            commands::clean::run(*full, *orphans)
        }
        None => {
            // No command provided, show help
            output::info("No command provided. Use --help for usage information");
            Ok(())
        }
    }
}
