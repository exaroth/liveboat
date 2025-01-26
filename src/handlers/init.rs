use log::info;
use std::fs;

use anyhow::Result;

use crate::cli;
use crate::handlers::aux;
use crate::handlers::{
    NIGHTLY_CHANNEL_NAME, RELEASE_CHANNEL, STABLE_CHANNEL_NAME,
};
use crate::opts::Options;
use crate::paths::Paths;

/// Initialize configuration for the app, prompting user for input.
pub fn cold_start(use_nightly: bool, paths: &Paths) -> Result<()> {
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

    let release_channel: String;
    if use_nightly {
        release_channel =
            format!("{}/{}", RELEASE_CHANNEL, NIGHTLY_CHANNEL_NAME);
    } else {
        release_channel =
            format!("{}/{}", RELEASE_CHANNEL, STABLE_CHANNEL_NAME);
    }
    println!("Using {} as release channel", release_channel);

    aux::fetch_templates(
        &release_channel,
        paths.tmp_dir().join("update").as_path(),
        paths.template_dir(),
    )?;
    println!("Done");
    Ok(())
}

fn initialization_wizard(opts: &mut Options, paths: &Paths) -> Result<()> {
    opts.title =
        cli::prompt_string(opts.title.clone(), "Enter your feed page title:")?;
    info!("Title is : {}", opts.title);
    opts.newsboat_urls_file = cli::prompt_path(
        &paths.url_file(),
        true,
        "Enter path to Newsboat urls file:",
    )?;
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
