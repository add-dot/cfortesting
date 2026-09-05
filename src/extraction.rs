use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct ApiResponseLtsKnow {
    channels: HashMap<String, ChanelObject>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ChanelObject {
    channel: String,
    version: String,
    revision: String,
    downloads: HashMap<String, Vec<Download>>,
}

#[derive(Debug, Deserialize)]
struct Download {
    platform: String,
    url: String,
}

// This is need for the struct to work on versions <= 115.0.5763.0 the CfT all versions >=
// 115.0.5763.0  only have chrome on the download vector
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct DownloadsOld {
    chrome: Vec<Download>,
}

#[derive(Debug, Deserialize)]
struct ApiResponseVersions {
    versions: Vec<GoodKnowVersions>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct GoodKnowVersions {
    version: String,
    revision: String,
    downloads: HashMap<String, Vec<Download>>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Downloads {
    chrome: Vec<Download>,
    chromedriver: Vec<Download>,
    #[serde(rename = "chrome-headless-shell")]
    chrome_headless_shell: Vec<Download>,
}

/// This function get all the data from the github.io of chrome to later parse
pub async fn fetch_cft(url: &str) -> Result<String, Box<dyn Error>> {
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
    let response = client.get(url).send().await.unwrap();
    Ok(response.text().await?)
}

pub fn search_values_for_specifc_version(
    response: &str,
    platform: &str,
    browser_version: &str,
    type_of_chrome: &str,
) -> Result<(String, String), Box<dyn Error>> {
    let response: ApiResponseVersions = serde_json::from_str(response)?;
    let mut downloable_version: Option<&Download> = None;
    let all_versions: Vec<GoodKnowVersions> = response.versions;
    let chrome_version_to_download = all_versions.iter().find(|x| x.version == browser_version);
    if let Some(platforms) = chrome_version_to_download
        .unwrap()
        .downloads
        .get(type_of_chrome)
    {
        downloable_version = platforms.iter().find(|x| x.platform == platform);
    }
    match downloable_version {
        Some(download) => Ok((
            chrome_version_to_download.unwrap().version.clone(),
            download.url.clone(),
        )),
        None => panic!("Error, specifc version not found in the good versions"),
    }
}

pub fn search_values_for_lts_know(
    response: &str,
    platform: &str,
    browser_version: &str,
    type_of_chrome: &str,
) -> Result<(String, String), Box<dyn Error>> {
    let response: ApiResponseLtsKnow = serde_json::from_str(response)?;
    let mut download_version: Option<&Download> = None;
    let mut channels_version: String = String::new();
    // The browser_version in fact is  one of the (Stable, Beta)
    // test chrome@1.1.1.1 and see
    if let Some(channels) = response.channels.get(browser_version) {
        channels_version.clone_from(&channels.version);
        if let Some(platforms) = channels.downloads.get(type_of_chrome) {
            download_version = platforms.iter().find(|x| x.platform == platform);
        }
    }
    match download_version {
        Some(download) => Ok((channels_version, download.url.clone())),
        None => panic!("Error the version provides LTS is not good know version"),
    }
}

pub fn list_channels() -> Vec<String> {
    let channels: HashMap<&str, &str> = HashMap::from([
        ("stable", "Stable"),
        ("beta", "Beta"),
        ("dev", "Dev"),
        ("canary", "Canary"),
    ]);
    let mut vec_channels: Vec<String> = vec![];
    for value in channels.values() {
        vec_channels.push((*value).to_string());
    }
    vec_channels
}
