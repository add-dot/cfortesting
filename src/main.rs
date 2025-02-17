use dirs::home_dir;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::absolute;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use futures_util::StreamExt;

const URL: &str =
    "https://googlechromelabs.github.io/chrome-for-testing/known-good-versions-with-downloads.json";

#[derive(Debug, Deserialize)]
struct ApiResponse {
    versions: Vec<Version>,
}

#[derive(Debug, PartialEq)]
enum CreationDirError {
    BadPath(String),
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Version {
    version: String,
    revision: String,
    downloads: Downloads,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Downloads {
    chrome: Vec<Download>,
}

// This is need for the struct to work on versions <= 115.0.5763.0 the CfT all versions >=
// 115.0.5763.0  only have chrome on the download vector
#[allow(dead_code)]
struct DownloadsOld {
    chrome: Vec<Download>,
    chromedriver: Vec<Download>,
    chrome_headless_shell: Vec<Download>,
}

#[derive(Debug, Deserialize)]
struct Download {
    platform: String,
    url: String,
}

async fn fetch_latest_chrome_download_url(
    platform: &str,
    browser_version: &str,
) -> Result<String, Box<dyn Error>> {
    use reqwest::header;
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json"),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let response: ApiResponse = client.get(URL).send().await.unwrap().json().await?;

    let mut downloable_version: String = String::default();

    for version in response.versions {
        if let Some(download) = version
            .downloads
            .chrome
            .iter()
            .find(|d| d.platform == platform)
        {
            println!("Latest Chrome version: {}", version.version);
            println!("Download URL: {}", download.url);
            let url = download.url.clone();
            if browser_version == version.version {
                downloable_version = url;
                return Ok(downloable_version);
            } else {
                downloable_version = url;
                return Ok(downloable_version);
            }
        }
    }

    println!("No download found for platform: {platform:?}");
    Ok(downloable_version)
}

fn get_platform() -> &'static str {
    match () {
        () if cfg!(target_os = "linux") => "linux64",
        () if cfg!(all(target_os = "macos", target_arch = "aarch64")) => "mac-arm64",
        () if cfg!(all(target_os = "macos", target_arch = "x86_64")) => "mac-x64",
        () if cfg!(all(target_os = "windows", target_pointer_width = "32")) => "win32",
        () if cfg!(all(target_os = "windows", target_pointer_width = "64")) => "win64",
        () => "unknown",
    }
}

async fn download_browser(
    target_browser: String,
    target_file_os: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(target_browser).send().await.unwrap();

    let content_length = response.content_length().unwrap_or(0);

    // Barra de progreso
    let pb = ProgressBar::new(content_length);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} ({eta})")?
        .progress_chars("#>-"));

    if response.status().is_success() {
        let mut f = File::create(target_file_os + "downloaded_file.zip")?;
        // Crear archivo
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            f.write_all(&chunk)?;
            pb.inc(chunk.len() as u64);
        }

        pb.finish_with_message("Descarga completada!");
    } else {
        println!("Failed to download file. Status: {}", response.status());
    }

    Ok(())
}

fn create_dir() -> Result<String, Box<dyn std::error::Error>> {
    let home = home_dir().unwrap();
    let home_parsed = home.to_str().unwrap();
    let absolute_home = absolute(home_parsed.to_owned() + "/.cft/").unwrap();
    let parsed_absolute = absolute_home.to_str().unwrap();
    let dir_all = fs::create_dir_all(parsed_absolute);
    match dir_all {
        Ok(..) => Ok(parsed_absolute.to_string()),
        Err(e) => panic!("{e:?}"),
    }
}
#[tokio::main]
async fn main() {
    let platform = get_platform();
    let parsed_absolute = create_dir().unwrap();
    println!("{parsed_absolute:?}");
    let version: &str = "latest";
    let cft_api_response = fetch_latest_chrome_download_url(platform, version)
        .await
        .unwrap();
    println!("{cft_api_response:?}");
    let _ = download_browser(cft_api_response, parsed_absolute).await;
}
