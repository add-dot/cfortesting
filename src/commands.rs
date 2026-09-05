use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cfort")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// list all chanels
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List channels of chrome, chromevdriver, chrome-headless-shell
    ListChannels,
    /// Select the Channels to download the chrome.
    Install {
        //#[arg(short, long)]
        // Yes this is a direct value not a argument value
        /// this need to be like chrome@latest, chrome@1.1.1
        value: String,
    },
}
