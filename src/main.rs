mod args;
mod builders;
mod cli;
mod content;
mod db;
mod errors;
mod feed;
mod feed_item;
mod handlers;
mod opts;
mod paths;
mod template;
mod urls;
mod utils;

use anyhow::Result;
use clap::Parser;
use nix::unistd::Uid;
use self_replace::self_replace;
use std::path::PathBuf;
use sudo;

use crate::args::{Args, Command};
use crate::handlers::LIVEBOAT_UPDATE_BIN_PATH_ENV;
use crate::handlers::{build, init, update};
use crate::paths::Paths;
use crate::utils::tidy_up;
use log::info;

fn main() {
    let args = Args::parse();
    utils::init_logger(args.debug);
    let exec_result = match args.command {
        Command::Init => init(&args),
        Command::Build => build(&args),
        Command::Update => update(&args),
    };
    if let Err(e) = exec_result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    std::process::exit(0);
}

/// Initialize configuration and fetch auxiliary data.
fn init(args: &Args) -> Result<()> {
    let paths = Paths::new(&args.config_file)?;
    let result = init::cold_start(args.use_nightly, &paths);
    tidy_up(paths.tmp_dir());
    return result;
}

/// Update binary and templates when available.
fn update(args: &Args) -> Result<()> {
    let update_bin_path_r = std::env::var(LIVEBOAT_UPDATE_BIN_PATH_ENV);
    info!("Update path env is {:?}", update_bin_path_r);
    if update_bin_path_r.is_ok() && Uid::effective().is_root() {
        info!("Updating binary as root");
        let new_exec_path = PathBuf::from(update_bin_path_r.unwrap());
        if !new_exec_path.exists() {
            panic!(
                "Temporary binary not found for path: {}",
                new_exec_path.display()
            );
        }
        self_replace(&new_exec_path)?;
        _ = std::fs::remove_file(&new_exec_path);
        std::env::remove_var(LIVEBOAT_UPDATE_BIN_PATH_ENV);
        println!("Liveboat binary updated");
        return Ok(());
    }
    let paths = Paths::new(&args.config_file)?;
    let result = update::update_files(args.debug, args.use_nightly, &paths);
    if result.is_err() {
        tidy_up(paths.tmp_dir());
        return Err(result.unwrap_err());
    }
    let restart_required = result.unwrap();
    tidy_up(paths.tmp_dir());
    if restart_required {
        info!("Restarting update process with root privileges");
        sudo::with_env(&["LIVEBOAT_UPDATE_"]).unwrap();
    }
    Ok(())
}

/// Faciliate building and outputting feeds and template
/// data.
fn build(args: &Args) -> Result<()> {
    info!("Build command called");
    let controller = match build::BuildController::init(&args) {
        Err(e) => return Err(e),
        Ok(ctrl) => ctrl,
    };
    match controller.build() {
        Err(e) => return Err(e),
        _ => return Ok(()),
    };
}
