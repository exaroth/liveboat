use std::fmt;

use clap::Parser;

// TODO: add -x option allowing custom commands to be used
// generate, upload, serve, update
// add deug option
/// Static page generator for newsboat feeds, use -h to see help
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    /// Path to newsboat db cache (default ~/.newsboat/cache.db)
    pub cache_file: Option<String>,
    /// Path to newsboat urls file to use (default ~/.newsboat/urls)
    #[arg(long)]
    pub url_file: Option<String>,
    /// Path to directory where built static files will be stored (default ~/.newsboat/build")
    #[arg(long)]
    pub build_dir: Option<String>,
    /// Path to directory containing templates.
    #[arg(long)]
    pub template_dir: Option<String>,
    /// Filepath to liveboat config file (default ~/.newsboat/liveboat_config.yml)
    #[arg(long)]
    pub config_file: Option<String>,

    // TODO
    // add debug
    // add command specifier
}

#[derive(Debug, Clone)]
pub struct ArgumentError {
    arg: String,
    msg: String 
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid argument: {}: {}", self.arg, self.msg)
    }
}

impl ArgumentError {
    pub fn new(argname: String, message: String) -> ArgumentError {
        ArgumentError{
            arg: argname,
            msg: message,
        }
    }
}
