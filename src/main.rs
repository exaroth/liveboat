mod args;
mod builder;
mod cli;
mod controller;
mod feed;
mod feed_item;
mod opts;
mod paths;
mod template;
mod urls;
mod errors;

use clap::Parser;
use std::error::Error;

use crate::args::{Args, Command};
use crate::controller::Controller;

fn main() {
    let args = Args::parse();
    let exec_result = match args.command {
        Command::Init => cold_start(&args),
        Command::Build => build(&args),
        _ => None,
    };
    if let Some(e) = exec_result {
        eprintln!("Error: {}", e)
    }
}

fn cold_start(args: &Args) -> Option<Box<dyn Error>> {
    let controller = Controller::cold_start(args);
    None
}

fn build(args: &Args) -> Option<Box<dyn Error>> {
    let controller = match Controller::init(&args) {
        Err(e) => return Some(e),
        Ok(ctrl) => ctrl,
    };
    match controller.process_feeds() {
        Err(e) => return Some(e),
        _ => return None
    };
}
