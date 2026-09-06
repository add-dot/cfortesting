use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    InvalidEntryFormat,
    InvalidName,
    EmptyValue,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidEntryFormat => {
                write!(f, "Invalid entry format, expected format: name@version")
            }
            Error::InvalidName => write!(
                f,
                "Invalid name, must be chrome, chromedriver, chrome-headless-shell"
            ),
            Error::EmptyValue => write!(f, "The provided entry value is empty"),
        }
    }
}
impl StdError for Error {}

/// entry: &str input of the user on the cli this functions parse the entry argument of the user
/// and make it valid to the other functions so it make it easy to use.
///
/// The last else case is only when no @ is given in the input always install the stable version of
/// the given chrome service.
/// Example:
/// ``` bash
/// cfortesting install chrome@1.1.1
/// cfortesting install chrome
/// cfortesting install chrome@stable
/// cfortesting install chrome@Stable
/// cfortesting install chrome@Canary
/// cfortesting install chrome@dev
/// ```
pub fn parse_entry(entry: &str) -> Result<(&str, &str), Error> {
    let trimmed_entry = entry.trim();
    if trimmed_entry.is_empty() {
        return Err(Error::EmptyValue);
    }
    if let Some((name, version)) = trimmed_entry.split_once('@') {
        // Validate the name matches expected values
        if version.is_empty() {
            return Err(Error::InvalidEntryFormat);
        }
        match name {
            "chrome" | "chromedriver" | "chrome-headless-shell" => {
                match version.to_lowercase().as_str() {
                    "stable" => Ok((name, "Stable")),
                    "beta" => Ok((name, "Beta")),
                    "dev" => Ok((name, "Dev")),
                    "canary" => Ok((name, "Canary")),
                    _ => Ok((name, version)),
                }
            }
            _ => Err(Error::InvalidName),
        }
    } else {
        let name = trimmed_entry;
        match name {
            "chrome" | "chromedriver" | "chrome-headless-shell" => Ok((name, "Stable")),
            _ => Err(Error::InvalidName),
        }
    }
}
