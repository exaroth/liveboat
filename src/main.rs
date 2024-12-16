mod args;
mod builder;
mod cli;
mod controller;
mod errors;
mod feed;
mod feed_item;
mod opts;
mod paths;
mod template;
mod urls;
mod utils;
mod db;

use clap::Parser;
use std::error::Error;

use crate::args::{Args, Command};
use crate::controller::BuildController;
use crate::utils::cold_start;
use log::info;

fn main() {
    let args = Args::parse();
    utils::init_logger(args.debug);
    let exec_result = match args.command {
        Command::Init => init(&args),
        Command::Build => build(&args),
    };
    if let Some(e) = exec_result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    std::process::exit(0);
}

/// Initialize configuration and fetch auxiliary data.
fn init(args: &Args) -> Option<Box<dyn Error>> {
    match cold_start(args) {
        Err(e)=> Some(e),
        _ => None
    }
}

/// Faciliate building and outputting feeds and template
/// data.
fn build(args: &Args) -> Option<Box<dyn Error>> {
    info!("Build command called");
    let controller = match BuildController::init(&args) {
        Err(e) => return Some(e),
        Ok(ctrl) => ctrl,
    };
    match controller.build() {
        Err(e) => return Some(e),
        _ => return None,
    };
}
