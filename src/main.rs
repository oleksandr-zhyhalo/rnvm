use clap::Parser;

mod cli;
mod config;
mod errors;
mod installer;
mod utils;
mod version;

#[tokio::main]

async fn main() -> errors::Result<()> {
    env_logger::init();

    let cli = cli::Cli::parse();

    if let Err(e) = cli::run_with(cli).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
