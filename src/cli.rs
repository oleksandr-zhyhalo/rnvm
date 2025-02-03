use clap::{Parser, Subcommand};
use std::error::Error;

/// A simple Node.js version manager written in Rust.
#[derive(Parser)]
#[clap(name = "node-tool", version = "0.1", about = "Manage Node.js versions")]
pub struct Cli {
    /// Increase verbosity (-v, -vv, etc.)
    #[clap(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install a Node.js version.
    Install {
        version: String,
    },
    /// Switch to a Node.js version.
    Use {
        version: String,
    },
    /// List installed Node.js versions.
    List,
    /// Uninstall a Node.js version.
    Uninstall {
        version: String,
    },
    /// Upgrade a Node.js version.
    Upgrade {
        #[clap(short, long)]
        version: Option<String>,
    },
}

pub async fn run_with(cli: Cli) -> Result<(), Box<dyn Error>> {
    // Set logging level based on verbosity
    let log_level = match cli.verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };
    std::env::set_var("RUST_LOG", log_level);

    match &cli.command {
        Commands::Install { version } => {
            println!("Installing Node.js version: {}", version);
            match crate::installer::install_node_version(version).await {
                Ok(_) => println!("Installation complete."),
                Err(e) => eprintln!("Error installing version {}: {}", version, e),
            }
        }
        Commands::Use { version } => {
            println!("Switching to Node.js version: {}", version);
            match crate::version::use_node_version(version) {
                Ok(msg) => println!("{}", msg),
                Err(e) => eprintln!("Error switching to version {}: {}", version, e),
            }
        }
        Commands::List => {
            println!("Installed Node.js versions:");
            if let Err(e) = crate::list::list_installed_versions() {
                eprintln!("Error listing versions: {}", e);
            }
        }
        Commands::Uninstall { version } => {
            println!("Uninstalling Node.js version: {}", version);
            if let Err(e) = crate::uninstall::uninstall_node_version(version) {
                eprintln!("Error uninstalling version {}: {}", version, e);
            } else {
                println!("Uninstallation complete.");
            }
        }
        Commands::Upgrade { version } => {
            match crate::upgrade::upgrade_node_version(version.as_deref()).await {
                Ok(msg) => println!("{}", msg),
                Err(e) => eprintln!("Error upgrading version: {}", e),
            }
        }
    }
    Ok(())
}