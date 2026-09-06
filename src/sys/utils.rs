use dirs::home_dir;
use std::env::consts::{ARCH, OS};
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    CannotCreateDir,
    HomeDirNotFound,
    InvalidPathUtf8,
}

pub fn create_dir() -> Result<String, Error> {
    let home: std::path::PathBuf = home_dir().ok_or(Error::HomeDirNotFound)?;
    let target_dir = home.join(".cft");
    if fs::create_dir_all(&target_dir).is_err() {
        return Err(Error::CannotCreateDir);
    }
    target_dir
        .to_str()
        .map(std::string::ToString::to_string)
        .ok_or(Error::InvalidPathUtf8)
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
    parsed_absoluted: &str,
    type_downloable: &str,
    version_string: &str,
) -> String {
    let final_extraction_path = Path::new(parsed_absoluted)
        .join(type_downloable)
        .join(version_string);
    final_extraction_path.to_string_lossy().to_string()
}
