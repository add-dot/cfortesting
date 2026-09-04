use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

pub async fn download_browser(
    target_browser: String,
    target_version: String,
    type_to_download: &str,
    target_file_os: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(target_browser).send().await.unwrap();

    let content_length = response.content_length().unwrap_or(0);
    let file_path_name = target_file_os.clone() + type_to_download + "-" + &target_version + ".zip";
    let msg_progress = "Download Finish!\n the file is located ".to_owned() + &file_path_name;

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
        let mut f = File::create(&file_path_name)?;
        // Create file
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.unwrap();
            f.write_all(&chunk).unwrap();
            pb.inc(chunk.len() as u64);
        }
        pb.finish_with_message(msg_progress);
        Ok(file_path_name)
    } else {
        panic!("Failed to download file. Status: {}", response.status());
    }
}

pub fn decompress(
    target_file_os: String,
    parsed_absolute: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let file = fs::File::open(target_file_os).unwrap();

    println!("{parsed_absolute:?}");
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let Some(outpath) = file.enclosed_name() else { continue };
        let final_outpath: PathBuf = [
            parsed_absolute.to_string(),
            outpath.to_str().unwrap().to_owned(),
        ]
        .into_iter()
        .collect();
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if file.is_dir() {
            println!("File {} extracted to \"{}\"", i, final_outpath.display());
            fs::create_dir_all(&final_outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                final_outpath.display(),
                file.size()
            );
            if let Some(p) = final_outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&final_outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&final_outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    Ok(String::default())
}
