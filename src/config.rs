use std::fs;
use std::path::{Path, PathBuf};

use libnewsboat::configpaths::ConfigPaths as NConfig;
use libnewsboat::configpaths::NEWSBOAT_CONFIG_SUBDIR;

use crate::args::{Args, ArgumentError};

const LIVEBOAT_CONFIG_FILENAME: &str = "liveboat_config.toml";
const LIVEBOAT_BUILD_DIRNAME: &str = "build";
const LIVEBOAT_FEED_DIRNAME: &str = "feeds";

#[derive(Debug, Default)]
pub struct Paths {
    build_dir: PathBuf,
    config_file: PathBuf,
    feed_dir: PathBuf,
    cache_file: PathBuf,
    url_file: PathBuf,
    lock_file: PathBuf,
}

impl Paths {
    pub fn new(args: Args) -> Result<Paths, String> {
        let n_config = NConfig::new();

        if !n_config.initialized() {
            return Err(n_config.error_message().into());
        };

        let mut paths = Paths {
            cache_file: n_config.cache_file().to_path_buf(),
            url_file: n_config.url_file().to_path_buf(),
            lock_file: n_config.lock_file().to_path_buf(),
            config_file: PathBuf::new(),
            build_dir: PathBuf::new(),
            feed_dir: PathBuf::new(),
        };

        paths.config_file = paths.newsboat_home_dir().join(LIVEBOAT_CONFIG_FILENAME);
        paths.build_dir = paths.newsboat_home_dir().join(LIVEBOAT_BUILD_DIRNAME);
        paths.feed_dir = paths.build_dir.join(LIVEBOAT_FEED_DIRNAME);

        if let Some(e) = paths.process_args(args) {
            return Err(format!("{:?}", e));
        }
        return Ok(paths);
    }

    pub fn initialized(&self) -> bool {
        return self.config_file.is_file();
    }

    pub fn lock_exists(&self) -> bool {
        return self.lock_file.is_file();
    }

    pub fn url_file(&self) -> &Path {
        return &self.url_file;
    }

    pub fn config_file(&self) -> &Path {
        return &self.config_file;
    }

    pub fn build_dir(&self) -> &Path {
        return &self.build_dir;
    }

    pub fn cache_file(&self) -> &Path {
        return &self.cache_file;
    }

    pub fn feed_dir(&self) -> &Path {
        return &self.feed_dir
    }

    fn set_argval(
        &mut self,
        argname: &str,
        arg: Option<String>,
        check_exists: bool,
    ) -> Option<ArgumentError> {
        if let Some(argval) = arg {
            match fs::canonicalize(argval) {
                Err(e) => {
                    if check_exists {
                        Some(ArgumentError::new(String::from(argname), e.to_string()));
                    }
                }
                Ok(p) => {
                    self.cache_file = p;
                }
            }
        }
        None
    }

    fn newsboat_home_dir(&self) -> PathBuf {
        #[allow(deprecated)]
        if let Some(home) = std::env::home_dir() {
            return home.join(NEWSBOAT_CONFIG_SUBDIR);
        }
        // NOTE: This clause can only trigger on Windows however
        // since newsboat is not Windows compatible it should never happen
        panic!("Could not retrieve home directory");
    }

    fn process_args(&mut self, args: Args) -> Option<ArgumentError> {
        self.set_argval("--url_file", args.url_file, true)?;
        self.set_argval("--cache_file", args.cache_file, true)?;
        self.set_argval("--build_dir", args.build_dir, false)?;
        self.set_argval("--config_file", args.config_file, false)?;
        None
    }
}
