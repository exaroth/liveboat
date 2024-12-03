use std::error::Error;
use std::fs;

use rand::{distributions::Alphanumeric, Rng};

use crate::args::Args;
use crate::cli;
use crate::opts::Options;
use crate::paths::Paths;
use log::info;

use env_logger::Env;

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
pub fn cold_start(args: &Args) -> Result<(), Box<dyn Error>> {
    info!("Initializing cold start");
    let mut paths = Paths::new(&args.config_file)?;
    paths.update_with_args(args)?;
    info!("Paths are: {}", paths);
    let mut opts = Options::default();
    info!("Default options are: {}", opts);
    initialization_wizard(&mut opts, &paths)?;
    fs::create_dir_all(paths.template_dir())?;
    // TODO: check if opts file eists;
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
    // TODO: download templates to template dir
    Ok(())
}

fn initialization_wizard(opts: &mut Options, paths: &Paths) -> Result<(), Box<dyn Error>> {
    opts.title = cli::prompt_string(opts.title.clone(), "Enter your feed page title:")?;
    info!("Title is : {}", opts.title);
    opts.site_url = cli::prompt_string(String::new(), "Enter url you plan to host the site at")?;
    info!("Site URL is : {}", opts.site_url);
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
