use dirs::home_dir;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    CannotCreateDir,
    HomeDirNotFound,
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
    match () {
        () if cfg!(target_os = "linux") => "linux64",
        () if cfg!(all(target_os = "macos", target_arch = "aarch64")) => "mac-arm64",
        () if cfg!(all(target_os = "macos", target_arch = "x86_64")) => "mac-x64",
        () if cfg!(all(target_os = "windows", target_pointer_width = "32")) => "win32",
        () if cfg!(all(target_os = "windows", target_pointer_width = "64")) => "win64",
        () => "unknown",
    }
}

pub fn tree_directory(
    parsed_absoluted: &Path,
    type_downloable: &str,
    version_string: &str,
) -> PathBuf {
    parsed_absoluted.join(type_downloable).join(version_string)
}
