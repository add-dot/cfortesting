use dirs::home_dir;
use std::env::consts::{ARCH, OS};
use std::path::{Path, PathBuf};
use std::{fmt, fs};

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    CannotCreateDir,
    HomeDirNotFound,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CannotCreateDir => {
                write!(f, "The required '.cft' directory cannot be created")
            }
            Error::HomeDirNotFound => {
                write!(f, "The User Home Directory is not Found")
            }
        }
    }
}
pub fn create_dir() -> Result<PathBuf, Error> {
    let home: std::path::PathBuf = home_dir().ok_or(Error::HomeDirNotFound)?;
    let target_dir = home.join(".cft");
    if fs::create_dir_all(&target_dir).is_err() {
        return Err(Error::CannotCreateDir);
    }
    Ok(target_dir)
}

pub fn get_platform() -> &'static str {
    match (OS, ARCH) {
        ("linux", _) => "linux64",
        ("macos", "aarch64") => "mac-arm64",
        ("macos", "x86_64") => "mac-x64",
        ("windows", "x86") => "win32",
        ("windows", "x86_64") => "win64",
        _ => "unkown",
    }
}

pub fn tree_directory(
    parsed_absoluted: &Path,
    type_downloable: &str,
    version_string: &str,
) -> PathBuf {
    parsed_absoluted.join(type_downloable).join(version_string)
}

pub fn is_already_isntalled(extraction_path: &Path, type_downloable: &str) -> bool {
    if !extraction_path.exists() {
        return false;
    }
    let binary_name = match (OS, type_downloable) {
        ("windows", "chrome") => "chrome.exe",
        ("windows", "chromedriver") => "chromedirver.exe",
        ("windows", "chrome-headless-shell") => "chrome-headless-shell.exe",
        (_, "chrome") => "chrome",
        (_, "chromedriver") => "chromedirver",
        (_, "chrome-headless-shell") => "chrome-headless-shell",
        _ => return true,
    };
    let binary_path = extraction_path.join(binary_name);
    binary_path.exists()
}
