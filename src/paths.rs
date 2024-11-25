use rand::{distributions::Alphanumeric, Rng};
use std::fs;
use std::path::{Path, PathBuf}; 
use handlebars::Handlebars;

use libnewsboat::configpaths::ConfigPaths as NConfig;
use libnewsboat::configpaths::NEWSBOAT_CONFIG_SUBDIR;

use crate::args::{Args, ArgumentError};

const LIVEBOAT_CONFIG_FILENAME: &str = "liveboat_config.toml";
const LIVEBOAT_BUILD_DIRNAME: &str = "build";
const LIVEBOAT_FEED_DIRNAME: &str = "feeds";

#[derive(Debug, Default)]
pub struct Paths {
    /// Default path to build directory used to store generated output.
    build_dir: PathBuf,
    /// Path to Liveboat TOML configuration file.
    config_file: PathBuf,
    /// Path to Newsboat cache db file.
    cache_file: PathBuf,
    /// Path to file containing page templates
    template_dir: PathBuf,
    /// Path to Newsboat urls file.
    url_file: PathBuf,
    /// Path to Newsboat lock file.
    lock_file: PathBuf,
    /// Path to temporary file used for building the page.
    tmp_dir: PathBuf,
}

fn generate_random_string(len: usize) -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
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
            tmp_dir: PathBuf::new(),
            template_dir: PathBuf::new(),
        };

        paths.config_file = paths.newsboat_home_dir().join(LIVEBOAT_CONFIG_FILENAME);
        paths.build_dir = paths.home().join(LIVEBOAT_BUILD_DIRNAME);
        // TODO: change to tmp
        paths.tmp_dir = Path::new("/home/exaroth/test").join(format!("liveboat-{}", generate_random_string(5)));
        // TODO: change after
        paths.template_dir = Path::new("/home/exaroth/templates").join("");

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

    pub fn tmp_dir(&self) -> &Path {
        return &self.tmp_dir;
    }

    pub fn template_dir(&self) -> &Path {
        return &self.template_dir;
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

    fn home(&self) -> PathBuf {
        #[allow(deprecated)]
        if let Some(home) = std::env::home_dir() {
            return home;
        }
        // NOTE: This clause can only trigger on Windows however
        // since newsboat is not Windows compatible it should never happen
        panic!("Could not retrieve home directory");
    }

    fn newsboat_home_dir(&self) -> PathBuf {
        return self.home().join(NEWSBOAT_CONFIG_SUBDIR);
    }

    fn process_args(&mut self, args: Args) -> Option<ArgumentError> {
        self.set_argval("--url_file", args.url_file, true)?;
        self.set_argval("--cache_file", args.cache_file, true)?;
        self.set_argval("--build_dir", args.build_dir, false)?;
        self.set_argval("--config_file", args.config_file, false)?;
        None
    }
}
