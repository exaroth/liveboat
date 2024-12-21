use clap::Parser;

/// Command represents list of available
/// commands which can be invoked via -x
/// argument.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Command {
    Init,
    Build,
    Update,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Init => "init",
            Self::Build => "build",
            Self::Update => "update",
        };
        s.fmt(f)
    }
}

impl std::str::FromStr for Command {

    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "init" => Ok(Self::Init),
            "build" => Ok(Self::Build),
            "update" => Ok(Self::Update),
            _ => Err(format!("Unknown command: {s}")),
        }
    }
}

/// Static page generator for newsboat feeds, use -h to see help
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {

    #[arg(long)]
    /// Path to newsboat db cache.
    pub cache_file: Option<String>,
    /// Path to newsboat urls file.
    #[arg(long)]
    pub url_file: Option<String>,
    /// Path to build directory.
    #[arg(long)]
    pub build_dir: Option<String>,
    /// Path to directory containing Liveboat template.
    #[arg(long)]
    pub template_path: Option<String>,
    /// path to liveboat config file.
    #[arg(long)]
    pub config_file: Option<String>,
    /// Print verbose code execution info.
    #[arg(
        long,
        default_value_t = false
        )]
    pub debug: bool,
    /// If set will use nightly channel for updates.
    #[arg(
        long,
        default_value_t = false
        )]
    pub use_nightly: bool,
    /// Command to execute [available options: build, init, update]
    #[arg(
        short = 'x',
        default_value_t = Command::Build,
    )]
    pub command: Command,
    /// Optional path to build directory
    #[arg(
        num_args(0..)
    )]
    pub build_target: Option<String>,
}
