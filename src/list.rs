use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub fn list_installed(parsed_absolute: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut installations = BTreeMap::new();

    if parsed_absolute.exists() {
        for entry in fs::read_dir(parsed_absolute)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let type_name = entry.file_name().to_string_lossy().into_owned();
                let mut versions = Vec::new();

                if let Ok(version_entries) = fs::read_dir(&path) {
                    for v_entry in version_entries.flatten() {
                        if v_entry.path().is_dir() {
                            versions.push(v_entry.file_name().to_string_lossy().into_owned());
                        }
                    }
                }
                if !versions.is_empty() {
                    versions.sort();
                    installations.insert(type_name, versions);
                }
            }
        }
    }

    if installations.is_empty() {
        println!("No installations found.");
        println!("Try running: cftesting list-channels and cftesting install <resource>@version to begin");
    } else {
        for (type_name, versions) in installations {
            println!("{type_name}:");
            for version in versions {
                println!("  - {version}");
            }
        }
    }

    Ok(())
}
