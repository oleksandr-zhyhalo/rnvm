use crate::errors::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(
    name = "rnvm",
    about = "Fast Node.js version manager written in Rust",
    long_about = "A lightweight and fast Node.js version manager that supports aliases, project-specific versions, and LTS releases"
)]
pub struct Cli {
    #[arg(
        short,
        long,
        global = true,
        action = clap::ArgAction::Count,
        help = "Increase logging verbosity"
    )]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Install a Node.js version", long_about = "Install a Node.js version. Examples:\n  rnvm install lts     # Install latest LTS\n  rnvm install 20.9.0  # Install specific version\n  rnvm install 20      # Install latest from major version")]
    Install {
        #[arg(help = "Version to install (e.g., '20.9.0', '18', 'lts', 'latest')")]
        version: String,
    },

    #[command(about = "Switch to a Node.js version")]
    Use {
        #[arg(help = "Version or alias to use (e.g., '20.9.0', 'lts', 'stable')")]
        version: String,
        #[arg(short, long, help = "Set this version as the default")]
        default: bool,
    },

    #[command(about = "List Node.js versions")]
    List {
        #[arg(short, long, help = "Show remote versions available to install")]
        remote: bool,
        #[arg(short, long, help = "Show only LTS versions")]
        lts: bool,
    },

    #[command(about = "Create an alias for a version")]
    Alias {
        #[arg(help = "Name of the alias (e.g., 'stable', 'prod')")]
        name: String,
        #[arg(help = "Version to alias")]
        version: String,
    },

    #[command(about = "Remove an alias")]
    Unalias {
        #[arg(help = "Name of the alias to remove")]
        name: String,
    },

    #[command(about = "Show current active version")]
    Current,

    #[command(about = "Set local version for current directory", long_about = "Create a .nvmrc file in the current directory to specify the Node.js version for this project")]
    Local {
        #[arg(help = "Version to set locally")]
        version: String,
    },

    #[command(about = "Show which version would be used in current directory", long_about = "Display which Node.js version would be used in the current directory, checking .nvmrc and default version")]
    Which,

    #[command(about = "Remove a Node.js version")]
    Uninstall {
        #[arg(help = "Version to remove")]
        version: String,
    },
}

pub async fn run_with(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Install { version } => {
            let resolved_version = crate::version::get_matching_version(&version).await?;
            println!("Installing Node.js {}...", resolved_version.version_str());
            crate::installer::install_version(&resolved_version).await?;
            println!("✓ Installed Node.js {}", resolved_version.version_str().green());
        }

        Commands::Use { version, default } => {
            let version_str = if let Some(aliased) = crate::config::alias::get_alias(&version)? {
                aliased
            } else {
                version.clone()
            };

            if !crate::version::local::is_installed(&version_str)? {
                println!("Version {} is not installed. Installing...", version_str);
                let resolved = crate::version::get_matching_version(&version_str).await?;
                crate::installer::install_version(&resolved).await?;
            }

            crate::version::local::use_version(&version_str)?;
            println!("✓ Now using Node.js {}", version_str.green());

            if default {
                crate::config::alias::set_alias("default", &version_str)?;
                println!("✓ Set {} as default version", version_str.green());
            }
        }

        Commands::List { remote, lts } => {
            if remote {
                let versions = crate::version::remote::fetch_version_list().await?;
                let versions: Vec<_> = versions
                    .into_iter()
                    .filter(|v| !lts || v.lts)
                    .collect();

                println!("Remote versions available:");
                for v in versions {
                    let lts_marker = if v.lts { " (LTS)".yellow() } else { "".normal() };
                    println!("  {}{}", v.version_str().green(), lts_marker);
                }
            } else {
                if crate::version::local::is_empty() {
                    println!("No Node.js versions installed yet. Use 'rnvm install <version>' to install one.");
                    return Ok(());
                }

                let versions = crate::version::local::get_installed_versions()?;
                let aliases = crate::config::alias::list_aliases()?;

                println!("Installed versions:");
                for ver in versions {
                    let prefix = if ver.is_current {
                        "* ".green()
                    } else {
                        "  ".normal()
                    };

                    // Find aliases for this version
                    let alias_list: Vec<String> = aliases
                        .iter()
                        .filter(|(_, v)| v == &&ver.version_str())
                        .map(|(k, _)| k.clone())
                        .collect();

                    let alias_str = if !alias_list.is_empty() {
                        format!(" (→ {})", alias_list.join(", ")).yellow()
                    } else {
                        "".normal()
                    };

                    println!("{}{}{}", prefix, ver.version_str().green(), alias_str);
                }
            }
        }

        Commands::Alias { name, version } => {
            crate::config::alias::set_alias(&name, &version)?;
            println!("✓ Created alias {} → {}", name.yellow(), version.green());
        }

        Commands::Unalias { name } => {
            crate::config::alias::remove_alias(&name)?;
            println!("✓ Removed alias {}", name.yellow());
        }

        Commands::Current => {
            if let Some(version) = crate::version::local::get_current_version()? {
                println!("Current version: {}", version.green());
            } else {
                println!("No active Node.js version");
            }
        }

        Commands::Local { version } => {
            let path = crate::config::local::create_nvmrc(&version)?;
            println!("✓ Created {} with version {}", path.display(), version.green());
        }

        Commands::Which => {
            if let Some((file, version)) = crate::config::local::find_version_file() {
                println!("Found version {} in {}", version.green(), file.display());
            } else if let Some(version) = crate::version::local::get_current_version()? {
                println!("Using global version: {}", version.green());
            } else if let Some(version) = crate::config::alias::get_alias("default")? {
                println!("Using default version: {}", version.green());
            } else {
                println!("No Node.js version specified");
            }
        }

        Commands::Uninstall { version } => {
            crate::version::local::remove_version(&version)?;
            println!("✓ Uninstalled Node.js {}", version.green());
        }
    }

    Ok(())
}