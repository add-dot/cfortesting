use std::fs;
use std::path::Path;

pub fn list_installed(parsed_absolute: &Path) {
    if !parsed_absolute.exists() {
        println!("No installations found.");
        return;
    }

    let mut found_any = false;

    if let Ok(entries) = fs::read_dir(parsed_absolute) {
        for entry in entries.flatten() {
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
                    found_any = true;
                    println!("{type_name}:");
                    for version in versions {
                        println!("  - {version}");
                    }
                }
            }
        }
    }
    if !found_any {
        println!("No installations found.");
        println!("Try running: cftesting list-channels and cftesting install <resource>@version to begin");
    }
}
