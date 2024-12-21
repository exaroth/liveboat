use flate2::read::GzDecoder;
use log::info;
use rand::{distributions::Alphanumeric, Rng};
use std::cmp::Ordering;
use std::env::current_exe;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::io;
use std::io::copy as ioCopy;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use anyhow::{anyhow, Result};
use env_logger::Env;
use self_replace::self_replace;
use tar::Archive;

use crate::cli;
use crate::errors::FilesystemError;
use crate::opts::Options;
use crate::paths::Paths;
use crate::template::TemplateConfig;

const VERSION_FNAME: &str = "VERSION";
const LIVEBOAT_FNAME: &str = "liveboat";

const RELEASE_CHANNEL: &str = "https://github.com/exaroth/liveboat/releases/download";
const TEMPLATES_ARCHIVE_FNAME: &str = "templates.tar.gz";

pub const LIVEBOAT_UPDATE_BIN_PATH_ENV: &str = "LIVEBOAT_UPDATE_BIN_PATH";
const UPDATER_TEMP_BIN_PATH: &str = "/tmp/liveboat.__u_temp__";

static SUPPORTED_TARGETS: &'static [&str] = &[
    "x86_64-unknown-linux-musl",
    "x86_64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "aarch64-unknown-linux-gnu",
];

/// Representation of Versioning used by liveboat,
/// conforming to <major>.<minor>.<patch> format.
#[derive(Debug)]
struct Version {
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

/// Initialize logger for the app.
pub fn init_logger(debug: bool) {
    let llevel = match debug {
        true => "info",
        false => "warn",
    };
    env_logger::Builder::from_env(Env::default().default_filter_or(llevel)).init();
    info!("Logger initialized")
}

/// Initialize configuration for the app, prompting user for input.
pub fn cold_start(paths: &Paths) -> Result<()> {
    info!("Initializing cold start");
    info!("Paths are: {}", paths);
    let mut opts = Options::default();
    info!("Default options are: {}", opts);
    initialization_wizard(&mut opts, &paths)?;
    fs::create_dir_all(paths.template_dir())?;
    if !paths.config_file().exists() {
        opts.save(paths.config_file())?;
        println!(
            "Saved config file to {}",
            paths.config_file().to_str().unwrap()
        );
    } else {
        println!(
            "Config file already exists, skipping write at {}",
            paths.config_file().display()
        );
    }
    let dl_path = paths.tmp_dir().join("update");
    fs::create_dir_all(&dl_path)?;
    let release_channel = format!("{}/stable", RELEASE_CHANNEL);
    fetch_templates(
        &release_channel,
        paths.tmp_dir().join("update").as_path(),
        paths.template_dir(),
    )?;

    Ok(())
}

/// Check for Liveboat updates, if there is new version available
/// fetch both liveboat binary as template files.
pub fn update_files(debug: bool, paths: &Paths) -> Result<bool> {
    if !paths.initialized() {
        Err(FilesystemError::NotInitialized)?;
    }
    let dl_path = paths.tmp_dir().join("update");
    fs::create_dir_all(&dl_path)?;
    let mut release_channel = RELEASE_CHANNEL.to_string();
    match debug {
        true => {
            println!("Debug mode enabled, using dev channel for updates");
            release_channel = format!("{}/nightly", release_channel);
        }
        false => release_channel = format!("{}/stable", release_channel),
    }
    let mut restart_required = false;

    let new_version_available =
        check_newer_binary_version_available(&release_channel, dl_path.as_path())?;
    match new_version_available {
        true => {
            println!("Newer version of Liveboat found. Fetching...");
            restart_required = update_liveboat_binary(&release_channel, dl_path.as_path())?;
        }
        false => {
            if debug {
                println!("Debug mode enabled, forcing redownload...");
                restart_required = update_liveboat_binary(&release_channel, dl_path.as_path())?;
            } else {
                println!("Latest version of Liveboat is already installed.")
            }
        }
    }
    fetch_templates(&release_channel, dl_path.as_path(), paths.template_dir())?;
    Ok(restart_required)
}

/// Download and update local liveboat binary. We return bool in result
/// indicating whether to attempt to propagate to sudo in order to replace the binary.
fn update_liveboat_binary(release_chan: &String, dl_path: &Path) -> Result<bool> {
    let target_r = option_env!("TARGET");
    let bin_name_r = option_env!("BIN_NAME");
    if target_r.is_none() || bin_name_r.is_none() {
        println!("Looks like your version of Liveboat cannot be updated");
        println!("Use package manager to update or compile manually");
        return Ok(false);
    }
    let target = target_r.unwrap();
    if !SUPPORTED_TARGETS.contains(&target) {
        println!("Version of Liveboat you're using does not support automatic updates");
        println!("Use package manager to update or compile manually");
        return Ok(false);
    }
    let bin_name = bin_name_r.unwrap();
    let d_url = format!("{}/{}", release_chan, bin_name);
    info!("Download url for binary is {}", d_url);
    let d_dl_path = dl_path.join(LIVEBOAT_FNAME);
    info!("Download path for binary is {}", d_dl_path.display());
    download_file(&d_url, &d_dl_path.as_path())?;
    let exe_path = current_exe()?;
    println!("{:?}", exe_path.display());
    fs::set_permissions(&d_dl_path, fs::Permissions::from_mode(0o755))?;
    info!("Copying binary to {}", exe_path.display());
    let replace_result = self_replace(&d_dl_path);
    println!("{:?}", replace_result);
    if replace_result.is_err() {
        fs::copy(&d_dl_path, UPDATER_TEMP_BIN_PATH)?;
        std::env::set_var(LIVEBOAT_UPDATE_BIN_PATH_ENV, UPDATER_TEMP_BIN_PATH);
        return Ok(true);
    }
    Ok(false)
}

/// Download and update local templates, taking versions in config.toml under consideration.
fn fetch_templates(release_chan: &String, dl_path: &Path, tpl_dir: &Path) -> Result<()> {
    println!("Fetching templates");
    if !tpl_dir.is_dir() {
        fs::create_dir_all(tpl_dir)?;
    }
    info!("Tpl dir is {}", tpl_dir.display());
    let t_url = format!("{}/{}", release_chan, TEMPLATES_ARCHIVE_FNAME);
    info!("Template download url: {}", t_url);
    let t_dl_path = dl_path.join(TEMPLATES_ARCHIVE_FNAME);
    info!("Local template download path is {}", t_dl_path.display());
    download_file(&t_url, &t_dl_path.as_path())?;
    let tar_gz = File::open(&t_dl_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(&dl_path)?;

    let entries = std::fs::read_dir(dl_path.join("templates"))?;
    for tpl_e in entries {
        let dirpath = tpl_e.unwrap().path();
        if !dirpath.is_dir() {
            continue;
        }
        let mut components = dirpath.components();
        let dirname = components
            .next_back()
            .unwrap()
            .as_os_str()
            .to_string_lossy()
            .to_string();
        println!("Processing template {}", dirname);
        let out_t = tpl_dir.join(&dirname);
        info!("Local template path: {}", out_t.display());
        if out_t.exists() {
            let remote_config = TemplateConfig::get_config_for_template(&dirpath.as_path())?;
            let local_config = TemplateConfig::get_config_for_template(&out_t.as_path())?;
            println!(
                "Remote template has version: {}, local: {}",
                remote_config.version, local_config.version
            );
            let remote_v = Version::from_str(remote_config.version)?;
            let local_v = Version::from_str(local_config.version)?;
            if local_v.cmp(&remote_v) != Ordering::Less {
                println!("Skipping update");
                continue;
            }
        }
        println!("Updating to new version...");
        copy_all(&dirpath, out_t)?;
    }

    Ok(())
}

/// Fetch and compare VERSION file in the latest release channel and
/// compare against local version.
fn check_newer_binary_version_available(release_chan: &String, dl_path: &Path) -> Result<bool> {
    println!("Checking for new liveboat version...");
    let v_url = format!("{}/{}", release_chan, VERSION_FNAME);
    info!("Remote url for VERSION file is {}", v_url);
    let v_dl_path = dl_path.join(VERSION_FNAME);
    info!("Local download path is {}", v_dl_path.display());
    download_file(&v_url, &v_dl_path.as_path())?;
    let v_contents = read_to_string(v_dl_path)?;
    info!("Raw contents of VERSION is {}", v_contents);
    let v_contents = v_contents
        .strip_suffix("\r\n")
        .or(v_contents.strip_suffix("\n"))
        .unwrap_or(v_contents.as_str());
    println!("Found remote version: {}", v_contents);
    println!("Local version: {}", env!("CARGO_PKG_VERSION"));
    let remote_ver = Version::from_str(v_contents.to_string())?;
    let current_ver = Version::from_str(env!("CARGO_PKG_VERSION").to_string())?;
    return Ok(current_ver.cmp(&remote_ver) == Ordering::Less);
}

fn initialization_wizard(opts: &mut Options, paths: &Paths) -> Result<()> {
    opts.title = cli::prompt_string(opts.title.clone(), "Enter your feed page title:")?;
    info!("Title is : {}", opts.title);
    opts.newsboat_urls_file =
        cli::prompt_path(&paths.url_file(), true, "Enter path to Newsboat urls file:")?;
    info!("url f is : {}", opts.newsboat_urls_file);
    opts.newsboat_cache_file = cli::prompt_path(
        &paths.cache_file(),
        true,
        "Enter path to Newsboat cache db file:",
    )?;
    info!("cache f is : {}", opts.newsboat_cache_file);
    opts.time_threshold = cli::prompt_int(
        opts.time_threshold,
        "Enter number of days in the past Liveboat should generate feeds for:",
    )?;
    info!("tt is : {}", opts.time_threshold);
    opts.show_read_articles = cli::confirm(
        opts.show_read_articles,
        "Should feed page include articles marked as read by Newsboat?",
    );
    info!("show read is : {}", opts.show_read_articles);
    opts.build_dir = cli::prompt_path(
        &paths.build_dir(),
        false,
        "Where should Liveboat save generated pages to?",
    )?;
    info!("build dir is : {}", opts.build_dir);
    Ok(())
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
fn download_file(url: &str, f_name: &Path) -> Result<()> {
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
