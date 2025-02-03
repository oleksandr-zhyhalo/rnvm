use clap::Parser;

mod cli;
mod installer;
mod list;
mod uninstall;
mod upgrade;
mod utils;
mod version;

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();
    // Delegate to the CLI runner
    if let Err(e) = cli::run_with(cli::Cli::parse()).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}