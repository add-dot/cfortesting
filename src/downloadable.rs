use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::env::temp_dir;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub async fn download_browser(
    target_browser: String,
    target_version: String,
    type_to_download: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(target_browser).send().await?;

    let content_length = response.content_length().unwrap_or(0);
    let mut file_path = temp_dir();
    let file_name = format!("{type_to_download}-{target_version}.zip");
    file_path.push(file_name);

    let msg_progress = "\nSuccess, now decompressing files...".to_string();
    let pb = ProgressBar::new(content_length);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} ({eta}) {msg}",
        )?
        .progress_chars("#>-"),
    );

    if response.status().is_success() {
        let f = File::create(&file_path)?;
        let mut writer = std::io::BufWriter::new(f);
        // Create file
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            writer.write_all(&chunk)?;
            pb.inc(chunk.len() as u64);
        }
        writer.flush()?;
        pb.finish_with_message(msg_progress);
        Ok(file_path)
    } else {
        Err(format!("Failed to download file. Status: {}", response.status()).into())
    }
}
// Implemtation with lifetime of file clean up on this option we use this  Drop Guard to drop the
// file after the decompress is made.
struct FileCleanup<'a>(&'a Path);
impl Drop for FileCleanup<'_> {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(self.0);
    }
}

pub fn decompress(
    target_file_os: &Path,
    parsed_absolute: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let _cleanup = FileCleanup(target_file_os);
    let file = fs::File::open(target_file_os)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let Some(outpath) = file.enclosed_name() else {
            continue;
        };
        let mut components = outpath.components();
        components.next();
        let stripped_path: PathBuf = components.collect();
        if stripped_path.as_os_str().is_empty() {
            continue;
        }
        let final_outpath = parsed_absolute.join(stripped_path);
        if file.is_dir() {
            fs::create_dir_all(&final_outpath)?;
        } else {
            if let Some(p) = final_outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let outfile = fs::File::create(&final_outpath)?;
            let mut writer = std::io::BufWriter::new(outfile);
            io::copy(&mut file, &mut writer)?;
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&final_outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }
    let marker_path = parsed_absolute.join(".cft_success");
    fs::File::create(&marker_path)?;
    println!("Completed successfully to: {}", parsed_absolute.display());
    Ok(())
}
