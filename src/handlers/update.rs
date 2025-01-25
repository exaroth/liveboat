use std::fs;
use std::collections::HashMap;
use std::env::current_exe;
use std::os::unix::fs::PermissionsExt;
use std::fs::read_to_string;
use std::cmp::Ordering;
use std::path::Path;

use anyhow::Result;
use log::info;
use self_replace::self_replace;

use crate::errors::FilesystemError;
use crate::paths::Paths;
use crate::handlers::{RELEASE_CHANNEL, STABLE_CHANNEL_NAME, NIGHTLY_CHANNEL_NAME};
use crate::handlers::LIVEBOAT_UPDATE_BIN_PATH_ENV;
use crate::handlers::aux;
use crate::utils;


/// Static map of target->release bin name.
lazy_static::lazy_static! {
    static ref SUPPORTED_TARGETS: HashMap<&'static str, &'static str> = [
        ("x86_64-unknown-linux-musl", "liveboat-linux-musl"),
        ("x86_64-unknown-linux-gnu", "liveboat-linux-gnu"),
        ("x86_64-apple-darwin", "liveboat-darwin"),
        ("aarch64-unknown-linux-gnu", "liveboat-aarch64"),
    ].iter().copied().collect();
}

const LIVEBOAT_FNAME: &str = "liveboat";
const VERSION_FNAME: &str = "VERSION";

const UPDATER_TEMP_BIN_PATH: &str = "/tmp/liveboat.__u_temp__";

/// Check for Liveboat updates, if there is new version available
/// fetch both liveboat binary as template files.
pub fn update_files(debug: bool, use_nightly: bool, paths: &Paths) -> Result<bool> {
    if !paths.initialized() {
        Err(FilesystemError::NotInitialized)?;
    }
    let dl_path = paths.tmp_dir().join("update");
    fs::create_dir_all(&dl_path)?;

    let release_channel: String;
    match use_nightly {
        true => {
            println!("Nightly mode enabled, using dev channel for updates");
            release_channel = format!("{}/{}", RELEASE_CHANNEL, NIGHTLY_CHANNEL_NAME);
        }
        false => release_channel = format!("{}/{}", RELEASE_CHANNEL, STABLE_CHANNEL_NAME),
    }
    println!("Using {} as release channel", release_channel);

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
    aux::fetch_templates(&release_channel, dl_path.as_path(), paths.template_dir())?;
    if !restart_required {
        println!("Update completed");
    }
    Ok(restart_required)
}

/// Download and update local liveboat binary. We return bool in result
/// indicating whether to attempt to propagate to sudo in order to replace the binary.
fn update_liveboat_binary(release_chan: &String, dl_path: &Path) -> Result<bool> {
    let target_r = option_env!("TARGET");
    if target_r.is_none() {
        println!("Looks like your version of Liveboat cannot be updated");
        println!("Use package manager to update or compile manually");
        return Ok(false);
    }
    let target = target_r.unwrap();
    if !SUPPORTED_TARGETS.contains_key(target) {
        println!("Version of Liveboat you're using does not support automatic updates");
        println!("Use package manager to update or compile manually");
        return Ok(false);
    }
    let bin_name = SUPPORTED_TARGETS.get(target).unwrap();

    let d_url = format!("{}/{}", release_chan, bin_name);
    info!("Download url for binary is {}", d_url);
    let d_dl_path = dl_path.join(LIVEBOAT_FNAME);
    info!("Download path for binary is {}", d_dl_path.display());
    utils::download_file(&d_url, &d_dl_path.as_path())?;
    let exe_path = current_exe()?;
    fs::set_permissions(&d_dl_path, fs::Permissions::from_mode(0o755))?;
    info!("Copying binary to {}", exe_path.display());
    let replace_result = self_replace(&d_dl_path);
    if replace_result.is_err() {
        info!("Retrying update as superuser");
        fs::copy(&d_dl_path, UPDATER_TEMP_BIN_PATH)?;
        std::env::set_var(LIVEBOAT_UPDATE_BIN_PATH_ENV, UPDATER_TEMP_BIN_PATH);
        return Ok(true);
    }
    println!("Liveboat binary updated");
    Ok(false)
}


/// Fetch and compare VERSION file in the latest release channel and
/// compare against local version.
fn check_newer_binary_version_available(release_chan: &String, dl_path: &Path) -> Result<bool> {
    println!("Checking for new liveboat version...");
    let v_url = format!("{}/{}", release_chan, VERSION_FNAME);
    info!("Remote url for VERSION file is {}", v_url);
    let v_dl_path = dl_path.join(VERSION_FNAME);
    info!("Local download path is {}", v_dl_path.display());
    utils::download_file(&v_url, &v_dl_path.as_path())?;
    let v_contents = read_to_string(v_dl_path)?;
    info!("Raw contents of VERSION is {}", v_contents);
    let v_contents = v_contents
        .strip_suffix("\r\n")
        .or(v_contents.strip_suffix("\n"))
        .unwrap_or(v_contents.as_str());
    println!("Found remote version: {}", v_contents);
    println!("Local version: {}", env!("CARGO_PKG_VERSION"));
    let remote_ver = utils::Version::from_str(v_contents.to_string())?;
    let current_ver = utils::Version::from_str(env!("CARGO_PKG_VERSION").to_string())?;
    return Ok(current_ver.cmp(&remote_ver) == Ordering::Less);
}
