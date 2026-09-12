use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cfortesting")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// list all chanels
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List Latest know good channels of chrome, chromevdriver, chrome-headless-shell
    ListChannels,
    /// List local installed resources.
    ListInstalled,
    /// List all Good Know Versions
    List,
    /// Select the Channels to download the chrome.
    Install {
        //#[arg(short, long)]
        // Yes this is a direct value not a argument value
        /// this need to be like chrome@latest, chrome@1.1.1
        value: String,
    },
    /// Remove and delete previous installations
    Purge {
        /// For specific values use <resource>@<version>
        /// Ex: chrome@1.1.1.1
        #[arg(conflicts_with = "all")]
        value: Option<String>,
        /// Remove all installed resources
        #[arg(long, short, conflicts_with = "value")]
        all: bool,
    },
}
