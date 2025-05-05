use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::fs::File;
use std::io::Write;

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
            "[{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} ({eta}) {msg_progress}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    // TODO: need to insert inside the template in order to work
    if response.status().is_success() {
        let mut f = File::create(file_path_name)?;
        // Create file
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.unwrap();
            f.write_all(&chunk).unwrap();
            pb.inc(chunk.len() as u64);
        }
        pb.finish_with_message(msg_progress);
        Ok(target_file_os)
    } else {
        panic!("Failed to download file. Status: {}", response.status());
    }
}
