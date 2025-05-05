use clap::Parser;
use extraction::list_channels;
use parse_install::parse_entry;
mod commands;
mod downloadable;
mod extraction;
mod parse_install;
mod tests;
mod unixs;

#[tokio::main]
async fn main() {
    const URL_GV: &str =
        "https://googlechromelabs.github.io/chrome-for-testing/known-good-versions-with-downloads.json";
    const URL_LTS_GK: &str = "https://googlechromelabs.github.io/chrome-for-testing/last-known-good-versions-with-downloads.json";
    let platform = unixs::utils::get_platform();
    let parsed_absolute = unixs::utils::create_dir().unwrap();
    let cli = commands::Cli::parse();
    match cli.command {
        commands::Commands::ListChannels => {
            let listed_channels = list_channels();
            println!("{listed_channels:?}");
        }
        commands::Commands::Install { value } => {
            let (type_downloable, version) = parse_entry(&value).unwrap();
            let mut version_string: String = String::default();
            let mut url: String = String::default();
            match version {
                "Stable" | "Beta" | "Dev" | "Canary" => {
                    let response = extraction::fetch_cft(URL_LTS_GK).await.unwrap();
                    (version_string, url) = extraction::search_values_for_lts_know(
                        &response,
                        platform,
                        version,
                        type_downloable,
                    )
                    .unwrap();
                }
                _ => {
                    let response = extraction::fetch_cft(URL_GV).await.unwrap();
                    (version_string, url) = extraction::search_values_for_specifc_version(
                        &response,
                        platform,
                        version,
                        type_downloable,
                    )
                    .unwrap();
                }
            }
            let _ = downloadable::download_browser(
                url,
                version_string,
                type_downloable,
                parsed_absolute,
            )
            .await
            .ok();
        }
    }
}
