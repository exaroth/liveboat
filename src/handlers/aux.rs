use std::fs;
use log::info;
use std::io::Write;
use std::path::Path;

use flate2::read::GzDecoder;
use tar::Archive;
use std::cmp::Ordering;

use anyhow::Result;
use crate::template::{TemplateConfig, TEMPLATE_CONFIG_FNAME};
use crate::utils;

const TEMPLATES_ARCHIVE_FNAME: &str = "templates.tar.gz";


/// Download and update local templates, taking versions in config.toml under consideration.
pub fn fetch_templates(release_chan: &String, dl_path: &Path, tpl_dir: &Path) -> Result<()> {
    println!("Fetching templates");
    if !tpl_dir.is_dir() {
        fs::create_dir_all(tpl_dir)?;
    }
    info!("Tpl dir is {}", tpl_dir.display());
    let t_url = format!("{}/{}", release_chan, TEMPLATES_ARCHIVE_FNAME);
    info!("Template download url: {}", t_url);
    let t_dl_path = dl_path.join(TEMPLATES_ARCHIVE_FNAME);
    info!("Local template download path is {}", t_dl_path.display());
    utils::download_file(&t_url, &t_dl_path.as_path())?;
    let tar_gz = fs::File::open(&t_dl_path)?;
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
            let mut remote_config = TemplateConfig::get_config_for_template(&dirpath.as_path())?;
            let local_config = TemplateConfig::get_config_for_template(&out_t.as_path())?;
            println!(
                "Remote template has version: {}, local: {}",
                remote_config.version, local_config.version
            );
            let remote_v = utils::Version::from_str(remote_config.version.clone())?;
            let local_v = utils::Version::from_str(local_config.version)?;
            if local_v.cmp(&remote_v) != Ordering::Less {
                println!("Skipping update");
                continue;
            }
            println!("Updating to new version...");
            remote_config.template_settings = local_config.template_settings;
            let t = toml::to_string(&remote_config)?;
            utils::copy_all(&dirpath, &out_t)?;
            let temp_cfg_path = out_t.join("config.toml.__temp__");
            let mut f = fs::File::create(&temp_cfg_path)?;
            f.write_all(t.as_bytes())?;
            fs::copy(&temp_cfg_path, out_t.join(TEMPLATE_CONFIG_FNAME))?;
            _ = fs::remove_file(temp_cfg_path);
        } else {
            utils::copy_all(&dirpath, out_t)?;
        }
    }

    Ok(())
}
