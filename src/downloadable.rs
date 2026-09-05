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
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(target_browser).send().await.unwrap();

    let content_length = response.content_length().unwrap_or(0);
    let mut file_path = temp_dir();
    let file_name = format!("{type_to_download}-{target_version}.zip");
    file_path.push(file_name);
    let file_path_name = file_path.to_str().unwrap().to_string();
    let msg_progress = format!("Download Finish!\n the file is located {file_path_name}");

    // Barra de progreso
    let pb = ProgressBar::new(content_length);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} ({eta}) {msg}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    if response.status().is_success() {
        let f = File::create(&file_path_name)?;
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
        Ok(file_path_name)
    } else {
        Err(format!("Failed to download file. Status: {}", response.status()).into())
    }
}

pub fn decompress(
    target_file_os: &str,
    parsed_absolute: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(target_file_os)?;

    println!("{parsed_absolute:?}");
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
        let final_outpath = Path::new(&parsed_absolute).join(stripped_path);
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if file.is_dir() {
            println!("File {} extracted to \"{}\"", i, final_outpath.display());
            fs::create_dir_all(&final_outpath)?;
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                final_outpath.display(),
                file.size()
            );
            if let Some(p) = final_outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&final_outpath)?;
            io::copy(&mut file, &mut outfile)?;
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
    let _ = fs::remove_file(target_file_os);
    Ok(())
}
