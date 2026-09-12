use clap::Parser;
use parse_install::parse_entry;

use crate::remove::purge_specific_version;
mod commands;
mod downloadable;
mod extraction;
mod list;
mod parse_install;
mod remove;
mod sys;
mod tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const URL_GV: &str =
        "https://googlechromelabs.github.io/chrome-for-testing/known-good-versions-with-downloads.json";
    const URL_LTS_GK: &str = "https://googlechromelabs.github.io/chrome-for-testing/last-known-good-versions-with-downloads.json";
    let platform = sys::utils::get_platform();
    let parsed_absolute = sys::utils::create_dir().unwrap_or_else(|err| {
        eprintln!("Error when initilizing enviroment {err:?}");
        std::process::exit(1);
    });
    let cli = commands::Cli::parse();
    match cli.command {
        commands::Commands::ListChannels => {
            match extraction::list_channels(URL_LTS_GK).await {
                Ok(listed_channels) => {
                    println!("Available channels and their current versions on last know good verisions:");
                    for channel in listed_channels {
                        println!("  {channel}");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to fetch channel information: {e}");
                    std::process::exit(1);
                }
            };
        }
        commands::Commands::ListInstalled => {
            list::list_installed(&parsed_absolute)?;
        }
        commands::Commands::List => {
            match extraction::list_all_versions(URL_GV).await {
                Ok(versions) => {
                    println!("Available known good versions:");
                    for version in &versions {
                        println!("  {version}");
                    }
                    println!("Total versions found: {}", versions.len());
                }
                Err(e) => {
                    eprintln!("Failed to fetch versions: {e}");
                    std::process::exit(1);
                }
            };
        }
        commands::Commands::Purge { value, all } => match (all, value) {
            (true, _) => {
                remove::purge_all(&parsed_absolute);
            }
            (false, Some(val)) => {
                purge_specific_version(&val, &parsed_absolute, platform, URL_LTS_GK).await?;
            }
            (false, None) => {
                eprintln!("Please provide a specific version to purge or use the --all flag.");
                std::process::exit(1);
            }
        },
        commands::Commands::Install { value } => {
            let (type_downloable, version) = parse_entry(&value).unwrap_or_else(|err| {
                eprintln!("Error parsing entry value: {err:?}");
                std::process::exit(1);
            });
            let (version_string, url) = match version {
                "Stable" | "Beta" | "Dev" | "Canary" => {
                    let response = extraction::fetch_cft(URL_LTS_GK).await?;
                    extraction::search_values_for_lts_know(
                        &response,
                        platform,
                        version,
                        type_downloable,
                    )?
                }
                _ => {
                    let response = extraction::fetch_cft(URL_GV).await?;
                    extraction::search_values_for_specific_version(
                        &response,
                        platform,
                        version,
                        type_downloable,
                    )?
                }
            };

            let extraction_path =
                sys::utils::tree_directory(&parsed_absolute, type_downloable, &version_string);
            if sys::utils::is_already_isntalled(&extraction_path, type_downloable) {
                println!(
                    "{} version {} is alredy installed at\n {}",
                    type_downloable,
                    version_string,
                    extraction_path.display()
                );
                return Ok(());
            }
            let target_file_os =
                downloadable::download_browser(url, version_string, type_downloable).await?;
            downloadable::decompress(&target_file_os, &extraction_path)?;
        }
    }
    Ok(())
}
