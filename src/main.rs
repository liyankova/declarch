use clap::Parser;
use declarch::cli::args::Cli;
use declarch::utils::output;

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
    // TODO: Implement command dispatch
    output::info("Command execution not yet implemented");
    Ok(())
}
