mod args;
mod builder;
mod cli;
mod controller;
mod db;
mod errors;
mod feed;
mod feed_item;
mod opts;
mod paths;
mod template;
mod urls;
mod utils;

use anyhow::Result;
use clap::Parser;

use crate::args::{Args, Command};
use crate::controller::BuildController;
use crate::paths::Paths;
use crate::utils::{cold_start, tidy_up, update_files};
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
    cold_start(args)?;
    Ok(())
}

/// Update binary and templates when available.
fn update(args: &Args) -> Result<()> {
    let paths = Paths::new(&args.config_file)?;
    let result = update_files(args.debug, &paths);
    tidy_up(paths.tmp_dir());
    return result;
}

/// Faciliate building and outputting feeds and template
/// data.
fn build(args: &Args) -> Result<()> {
    info!("Build command called");
    let controller = match BuildController::init(&args) {
        Err(e) => return Err(e),
        Ok(ctrl) => ctrl,
    };
    match controller.build() {
        Err(e) => return Err(e),
        _ => return Ok(()),
    };
}
