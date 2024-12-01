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

use clap::Parser;
use std::error::Error;

use crate::args::{Args, Command};
use crate::controller::BuildController;
use crate::utils::cold_start;

fn main() {
    let args = Args::parse();
    let exec_result = match args.command {
        Command::Init => init(&args),
        Command::Build => build(&args),
    };
    if let Some(e) = exec_result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    println!("Done");
    std::process::exit(0);
}

fn init(args: &Args) -> Option<Box<dyn Error>> {
    match cold_start(args) {
        Err(e)=> Some(e),
        _ => None
    }
}

fn build(args: &Args) -> Option<Box<dyn Error>> {
    print!("Building feeds...");
    let controller = match BuildController::init(&args) {
        Err(e) => return Some(e),
        Ok(ctrl) => ctrl,
    };
    match controller.process_feeds() {
        Err(e) => return Some(e),
        _ => return None,
    };
}
