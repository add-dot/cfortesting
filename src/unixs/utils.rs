use dirs::home_dir;
use std::fs;
use std::path::absolute;

pub const ENVIRO_DIR: &str = "/.cft/";

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    CannotCreateDir,
}

pub fn create_dir() -> Result<String, Error> {
    let home: std::path::PathBuf = match home_dir() {
        Some(home_path) => home_path,
        None => {
            panic!("error")
        }
    };
    let home_parsed = home.to_str().unwrap();
    let absolute_home = absolute(home_parsed.to_owned() + ENVIRO_DIR).unwrap();
    let parsed_absolute = absolute_home.to_str().unwrap();
    let dir_all = fs::create_dir_all(parsed_absolute);
    match dir_all {
        Ok(..) => Ok(parsed_absolute.to_string()),
        Err(..) => Err(Error::CannotCreateDir),
    }
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
