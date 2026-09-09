use crate::extraction;
use crate::parse_entry;
use crate::sys;
use std::path::Path;
use std::{fs::remove_dir_all, path::PathBuf};

pub fn purge_all(parsed_absoluted: &PathBuf) {
    remove_dir_all(parsed_absoluted).unwrap_or_else(|err| {
        eprintln!("Failed to remove all installations: {err}");
        std::process::exit(1);
    });
}

pub async fn purge_specific_version(
    val: &str,
    parsed_absolute: &Path,
    platform: &str,
    url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let (type_downloable, version) = parse_entry(val)?;

    let version_string = match version {
        "Stable" | "Beta" | "Dev" | "Canary" => {
            let response = extraction::fetch_cft(url).await?;
            let (v, _) = extraction::search_values_for_lts_know(
                &response,
                platform,
                version,
                type_downloable,
            )?;
            v
        }
        _ => version.to_string(),
    };

    let extraction_path =
        sys::utils::tree_directory(parsed_absolute, type_downloable, &version_string);

    if extraction_path.exists() {
        std::fs::remove_dir_all(&extraction_path).unwrap_or_else(|err| {
            eprintln!("Failed to remove {val}: {err}");
            std::process::exit(1);
        });
        println!("Successfully purged {type_downloable} version {version_string}.",);
    } else {
        println!("{type_downloable} version {version_string} is not installed.",);
    }

    Ok(())
}
