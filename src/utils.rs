use log::info;
use rand::{distributions::Alphanumeric, Rng};
use chrono::DateTime;
use std::fs;
use std::fs::File;
use std::io;
use std::io::copy as ioCopy;
use std::path::Path;
use std::cmp::Ordering;

#[cfg(not(test))]
use chrono::Local;

#[cfg(test)]
use chrono::Utc;

use anyhow::{anyhow, Result};
use env_logger::Env;



/// Representation of Versioning used by liveboat,
/// conforming to <major>.<minor>.<patch> format.
#[derive(Debug)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl Version {
    /// Load version instance from string.
    pub fn from_str(s: String) -> Result<Version> {
        let parts = s.split(".").collect::<Vec<&str>>();
        if parts.len() != 3 {
            return Err(anyhow!(
                "Invalid version detected, proper format is <major>.<minor>.<patch>: {}",
                s
            ));
        }
        let mut v = Version {
            major: 0,
            minor: 0,
            patch: 0,
        };
        v.major = parts[0].to_string().parse::<u64>()?;
        v.minor = parts[1].to_string().parse::<u64>()?;
        v.patch = parts[2].to_string().parse::<u64>()?;
        Ok(v)
    }
    /// Compare 2 versions returning ordering result in response.
    pub fn cmp(&self, other: &Version) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => match self.minor.cmp(&other.minor) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => match self.patch.cmp(&other.patch) {
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => Ordering::Equal,
                },
            },
        }
    }
}

#[cfg(not(test))]
pub fn now() -> DateTime<Local> {
    Local::now()
}

#[cfg(test)]
pub fn now() -> DateTime<Utc> {
    DateTime::from_timestamp(1733974974, 0).unwrap()
}

/// Initialize logger for the app.
pub fn init_logger(debug: bool) {
    let llevel = match debug {
        true => "info",
        false => "warn",
    };
    env_logger::Builder::from_env(Env::default().default_filter_or(llevel)).init();
    info!("Logger initialized")
}



/// Generate random string with given length.
pub fn generate_random_string(len: usize) -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
}

/// Fetch file from url saving it under path specified
pub fn download_file(url: &str, f_name: &Path) -> Result<()> {
    let mut response = reqwest::blocking::get(url)?;
    let mut file = File::create(f_name)?;
    ioCopy(&mut response, &mut file)?;

    Ok(())
}

/// Cleanup temp dir
pub fn tidy_up(tmp_dir: &Path) {
    _ = fs::remove_dir_all(tmp_dir);
}

/// Helper func for copying all the contents of directory
/// to another.
pub fn copy_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_loading_version_from_string() {
        let mut result = Version::from_str("1.2.3".to_string());
        assert!(result.is_ok());
        let v = result.unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        result = Version::from_str("12.3".to_string());
        assert!(result.is_err());
        result = Version::from_str("1.2.3.4".to_string());
        assert!(result.is_err());
        result = Version::from_str("gibberish".to_string());
        assert!(result.is_err());
        result = Version::from_str("1.2.3-alpha".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_comparing_versions() {
        let mut v1 = Version::from_str("1.2.3".to_string()).unwrap();
        let mut v2 = Version::from_str("1.2.3".to_string()).unwrap();
        assert_eq!(v1.cmp(&v2), Ordering::Equal);
        v1 = Version::from_str("1.2.3".to_string()).unwrap();
        v2 = Version::from_str("2.2.3".to_string()).unwrap();
        assert_eq!(v1.cmp(&v2), Ordering::Less);
        v1 = Version::from_str("1.2.4".to_string()).unwrap();
        v2 = Version::from_str("1.2.3".to_string()).unwrap();
        assert_eq!(v1.cmp(&v2), Ordering::Greater);
        v1 = Version::from_str("1.2.3".to_string()).unwrap();
        v2 = Version::from_str("1.4.3".to_string()).unwrap();
        assert_eq!(v1.cmp(&v2), Ordering::Less);
    }
}
